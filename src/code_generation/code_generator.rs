use code_generation::generation_context::{CGContext, Val, Register};
use std::fs::File;
use ast::*;

pub fn generate(out: &mut File, p: &Program) {
    let mut context = CGContext::new();
    p.generate(&mut context);
    context.write(out);
}

trait Generator<T> {
    fn generate(&self, context: &mut CGContext) -> T;
}

impl Generator<()> for Program {
    fn generate(&self, context: &mut CGContext) -> () {
        let Program(ref defs) = *self;

        context.add_code(format!("declare void @printInt(i32)"));

        for def in defs {
            match def {
                &Def::DFun(ref ret_type, ref name, _, _) => context.add_function(name, ret_type),
            }
        }

        for def in defs {
            context.in_new_function_scope(|context| def.generate(context));
        }
    }
}

impl Generator<()> for Def {
    fn generate(&self, context: &mut CGContext) -> () {
        match *self {
            Def::DFun(ref ret_type, ref name, ref args, ref stmts) => {
                let mut code = format!("define {} @{}(", ret_type.to_llvm(), name);
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        code = format!("{}, ", code);
                    }
                    code = format!("{}{}", code, arg.generate(context));
                }
                code = format!("{}) {}", code, '{');
                context.add_code(code);
                context.add_fun_prologue(ret_type);
                args.iter().map(|arg| generate_local_var(context, arg)).collect::<Vec<()>>();
                stmts.iter().map(|stmt| stmt.generate(context)).collect::<Vec<()>>();
                context.add_fun_epilogue(ret_type);
                context.add_code(String::from("}"));
            }
        }
    }
}

impl Generator<String> for Arg {
    fn generate(&self, context: &mut CGContext) -> String {
        let Arg(ref arg_type, ref id) = *self;
        let reg = context.add(id, arg_type);
        format!("{} {}", arg_type.to_llvm(), reg)
    }
}

fn generate_local_var(context: &mut CGContext, arg: &Arg) {
    let Arg(ref arg_type, ref id) = *arg;
    let val_reg = context.get_register(id);
    let addr_reg = generate_assign(context, format!("alloca {}", arg_type.to_llvm()));
    context.switch_reg(id, addr_reg);
    store_var(id, &Val::Register(val_reg), &arg_type.to_llvm(), context);
}

impl Generator<()> for Stmt {
    fn generate(&self, context: &mut CGContext) -> () {
        match *self {
            Stmt::SAss(ref id, ref expr) => {
                let val = expr.generate(context);
                let rhs_type = expr.get_type(context).to_llvm();
                store_var(id, &val, &rhs_type, context)
            },
            Stmt::SDecl(ref item_type, ref items) =>
                for item in items {
                    generate_item(item_type, item, context);
                },
            Stmt::SInc(ref id) => manipulate_variable(id, String::from("add i32 1"), context),
            Stmt::SDecr(ref id) => manipulate_variable(id, String::from("sub i32 1"), context),
            Stmt::SExpr(ref expr) => {
                expr.generate(context);
            },
            Stmt::SVRet => {
                let ret_label = context.current_func_ret_label();
                context.add_code(format!("br label {}", ret_label))
            },
            Stmt::SRet(ref expr) => {
                let ret_addr = context.current_func_ret_addr();
                let val = expr.generate(context);
                let val_type = expr.get_type(context).to_llvm();
                let ret_label = context.current_func_ret_label();
                context.add_code(format!("store {} {}, {}* {}", val_type, val, val_type, ret_addr));
                context.add_code(format!("br label {}", ret_label));
            },
            Stmt::SBlock(ref stmts) => {
                context.in_new_scope(|context| stmts.iter().map(|stmt| stmt.generate(context)).collect::<Vec<()>>());
            }
            Stmt::SIf(ref expr, ref block) => {
                let expr_val = expr.generate(context);
                let if_label = context.next_label();
                let after_label = context.next_label();
                context.add_code(format!("br i1 {}, label {}, label {}", expr_val, if_label, after_label));
                context.add_label(&if_label);
                context.in_new_scope(|mut context| block.generate(context));
                context.add_code(format!("br label {}", after_label));
                context.add_label(&after_label);
            }
            Stmt::SIfElse(ref expr, ref block1, ref block2) => {
                let expr_val = expr.generate(context);
                let if_label = context.next_label();
                let else_label = context.next_label();
                let after_label = context.next_label();
                context.add_code(format!("br i1 {}, label {}, label {}", expr_val, if_label, else_label));
                context.add_label(&if_label);
                context.in_new_scope(|mut context| block1.generate(context));
                context.add_code(format!("br label {}", after_label));
                context.add_label(&else_label);
                context.in_new_scope(|mut context| block2.generate(context));
                context.add_code(format!("br label {}", after_label));
                context.add_label(&after_label);
            },
            Stmt::SWhile(ref expr, ref block) => {
                let while_label = context.next_label();
                let body_label = context.next_label();
                let after_label = context.next_label();
                context.add_code(format!("br label {}", while_label));
                context.add_label(&while_label);
                let expr_val = expr.generate(context);
                context.add_code(format!("br i1 {}, label {}, label {}", expr_val, body_label, after_label));
                context.add_label(&body_label);
                context.in_new_scope(|mut context| block.generate(context));
                context.add_code(format!("br label {}", while_label));
                context.add_label(&after_label);
            },
            Stmt::Empty => (),
        }
    }
}

fn generate_assign(context: &mut CGContext, rhs: String) -> Register {
    let reg = context.next_register();
    context.add_code(format!("{} = {}", reg, rhs));
    reg
}

fn manipulate_variable(var: &String, operation: String, context: &mut CGContext) -> () {
    let ptr = read_var(var, context);
    let var_type = context.get_type(var).to_llvm();
    let val = generate_assign(context, format!("{}, {}", operation, ptr));
    store_var(var, &Val::Register(val), &var_type, context)
}

fn read_var(var: &String, context: &mut CGContext) -> Val {
    let var_reg = context.get_register(var);
    let var_type = context.get_type(var).to_llvm();
    Val::Register(generate_assign(context, format!("load {}, {}* {}", var_type, var_type, var_reg)))
}

fn store_var(var: &String, val: &Val, var_type: &String, context: &mut CGContext) -> () {
    let reg = context.get_register(var);
    context.add_code(format!("store {} {}, {}* {}", var_type, val.clone(), var_type, reg))
}

fn generate_item(item_type: &Type, item: &Item, context: &mut CGContext) -> () {
    let reg = context.add(&item.get_id(), item_type);
    context.add_code(format!("{} = alloca {}", reg, item_type.to_llvm()));
    match *item {
        Item::Init(ref id, ref expr) => Stmt::SAss(id.clone(), expr.clone()).generate(context),
        Item::NoInit(ref id) => Stmt::SAss(id.clone(), item_type.default_value()).generate(context),
    }
}

impl Type {
    fn default_value(&self) -> Expr {
        match *self {
            Type::TBool => Expr::EBoolLit(false),
            Type::TInt => Expr::EIntLit(0),
            Type::TString => Expr::EStringLit(String::new()),
            _ => unreachable!(),
        }
    }

    pub fn to_llvm(&self) -> String {
        String::from(match *self {
            Type::TInt => "i32",
            Type::TBool => "i1",
            Type::TString => "i8*",
            Type::TVoid => "void",
            _ => unreachable!()
        })
    }
}


impl Generator<Val> for Expr {
    fn generate(&self, context: &mut CGContext) -> Val {
        match *self {
            Expr::EIntLit(x) => Val::IConst(x),
            Expr::EBoolLit(b) =>
                Val::BConst(match b {
                    true => 1,
                    false => 0,
                }),
            Expr::EVar(ref id) => read_var(id, context),
            Expr::ENeg(ref expr) => {
                let e = expr.generate(context);
                Val::Register(generate_assign(context, format!("sub i32 0, {}", e)))
            },
            Expr::ENot(ref expr) => {
                let e = expr.generate(context);
                Val::Register(generate_assign(context, format!("sub i1 1, {}", e)))
            },
            Expr::EOp(ref lhs, ref op, ref rhs) => Val::Register(generate_op(lhs, op, rhs, context)),
            Expr::EApp(ref s, ref args) => {
                let llvm_args = args_to_llvm(args, context);
                let ret_type = context.get_type(s);
                let res = context.next_register();
                let mut call = match ret_type {
                    Type::TVoid => format!("call {} @{}(", ret_type.to_llvm(), s),
                    _ => format!("{} = call {} @{}(", res, ret_type.to_llvm(), s),

                };
                for (i, (val, arg_type)) in llvm_args.into_iter().enumerate() {
                    if i > 0 {
                        call = format!("{}, ", call);
                    }
                    call = format!("{}{} {}", call, arg_type.to_llvm(), val);
                }
                context.add_code(format!("{})", call));
                Val::Register(res)
            },
            Expr::EPredef(ref predef) => predef.generate(context),
            Expr::EStringLit(ref s) => unimplemented!()
        }
    }
}

fn generate_op(lhs: &Expr, op: &BinOp, rhs: &Expr, context: &mut CGContext) -> Register {
    match *op {
        BinOp::And => {
            let lhs_label = context.next_label();
            let rhs_label = context.next_label();
            let end_label = context.next_label();
            context.add_code(format!("br label {}", lhs_label));

            context.add_label(&lhs_label);
            let lhs = lhs.generate(context);
            context.add_code(format!("br i1 {}, label {}, label {}", lhs, rhs_label, end_label));

            context.add_label(&rhs_label);
            let rhs = rhs.generate(context);
            context.add_code(format!("br label {}", end_label));

            context.add_label(&end_label);
            generate_assign(context, format!("phi i1 [0, {}], [{}, {}]", lhs_label, rhs, rhs_label))
        },
        BinOp::Or => {
            let lhs_label = context.next_label();
            let rhs_label = context.next_label();
            let end_label = context.next_label();
            context.add_code(format!("br label {}", lhs_label));

            context.add_label(&lhs_label);
            let lhs = lhs.generate(context);
            context.add_code(format!("br i1 {}, label {}, label {}", lhs, end_label, rhs_label));

            context.add_label(&rhs_label);
            let rhs = rhs.generate(context);
            context.add_code(format!("br label {}", end_label));

            context.add_label(&end_label);
            generate_assign(context, format!("phi i1 [1, {}], [{}, {}]", lhs_label, rhs, rhs_label))
        },
        _ => {
            let t = lhs.get_type(context).to_llvm();
            let lhs = lhs.generate(context);
            let rhs = rhs.generate(context);
            generate_assign(context, format!("{} {} {}, {}", op.to_llvm(), t, &lhs, &rhs))
        },
    }
}

impl Generator<Val> for Predef {
    fn generate(&self, context: &mut CGContext) -> Val {
        match *self {
            Predef::PrintInt(ref e) => {
                let arg = e.generate(context);
                context.add_code(format!("call void @printInt(i32 {})", arg));
                Val::Register(context.next_register())
            },
            _ => unimplemented!(),
        }
    }
}

impl BinOp {
    fn to_llvm(&self) -> String {
        String::from(match *self {
            BinOp::Add => "add",
            BinOp::Sub => "sub",
            BinOp::Mul => "mul",
            BinOp::Mod => "srem",
            BinOp::Div => "sdiv",
            BinOp::LT => "icmp slt",
            BinOp::GT => "icmp sgt",
            BinOp::LE => "icmp sle",
            BinOp::GE => "icmp sge",
            BinOp::EQ => "icmp eq",
            BinOp::NEQ => "icmp ne",
            _ => unimplemented!(),
        })
    }

    pub fn get_type(&self) -> Option<Type> {
        match *self {
            BinOp::Sub |
            BinOp::Mul |
            BinOp::Div |
            BinOp::Mod => Some(Type::TInt),
            BinOp::And |
            BinOp::NEQ |
            BinOp::Or |
            BinOp::GE |
            BinOp::GT |
            BinOp::LE |
            BinOp::LT => Some(Type::TBool),
            _ => None,
        }
    }
}


fn args_to_llvm(args: &Vec<Expr>, context: &mut CGContext) -> Vec<(Val, Type)> {
    let mut llvm_args = vec![];

    for arg in args {
        let val = arg.generate(context);
        llvm_args.push((val, arg.get_type(context)))
    }

    llvm_args
}

impl Expr {
    fn get_type(&self, context: &CGContext) -> Type {
        match *self {
            Expr::EOp(ref lhs, ref op, _) => op.get_type().unwrap_or(lhs.get_type(context)),
            Expr::EApp(ref id, _) => context.get_type(id),
            Expr::EBoolLit(_) |
            Expr::ENot(_) => Type::TBool,
            Expr::EIntLit(_) |
            Expr::ENeg(_) => Type::TInt,
            Expr::EStringLit(_) => Type::TString,
            Expr::EPredef(ref predef) => predef.get_type(),
            Expr::EVar(ref id) => context.get_type(id),
        }
    }
}