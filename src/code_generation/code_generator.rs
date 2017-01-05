use code_generation::generation_context::{CGContext, Val};
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
                args.iter().map(|arg| generate_local_var(context, arg)).collect::<Vec<()>>();
                stmts.iter().map(|stmt| stmt.generate(context)).collect::<Vec<()>>();
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
    context.switch_reg(id, &addr_reg);
    store_var(id, &val_reg, &arg_type.to_llvm(), context);
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
            Stmt::SVRet => context.add_code(format!("ret void")),
            Stmt::SRet(ref expr) => {
                let val = expr.generate(context);
                let val_type = expr.get_type(context).to_llvm();
                context.add_code(format!("ret {} {}", val_type, val));
            },
            Stmt::SBlock(ref stmts) => {
                context.in_new_scope(|mut context|
                    for stmt in stmts {
                        stmt.generate(context);
                    })
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
                let expr_val = expr.generate(context);
                let body_label = context.next_label();
                let after_label = context.next_label();
                context.add_code(format!("br i1 {}, label {}, label {}", expr_val, body_label, after_label));
                context.add_label(&body_label);
                context.in_new_scope(|mut context| block.generate(context));
                context.add_code(format!("br label {}", body_label));
                context.add_label(&after_label);
            },
            Stmt::Empty => (),
        }
    }
}

fn generate_assign(context: &mut CGContext, rhs: String) -> Val {
    let reg = context.next_register();
    context.add_code(format!("{} = {}", reg, rhs));
    reg
}

fn manipulate_variable(var: &String, operation: String, context: &mut CGContext) -> () {
    let ptr = read_var(var, context);
    let var_type = context.get_type(var).to_llvm();
    let val = generate_assign(context, format!("{}, {} {}", operation, var_type, ptr));
    store_var(var, &val, &var_type, context)
}

fn read_var(var: &String, context: &mut CGContext) -> Val {
    let var_reg = context.get_register(var);
    let var_type = context.get_type(var).to_llvm();
    generate_assign(context, format!("load {}, {}* {}", var_type, var_type, var_reg))
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

impl Stmt {
    pub fn early_return(&self) -> bool {
        match *self {
            Stmt::SVRet | Stmt::SRet(_) => true,
            Stmt::SWhile(_, ref stmts) => stmts.early_return(),
            Stmt::SIfElse(_, ref b1, ref b2) => b1.early_return() && b2.early_return(),
            _ => false,
        }
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

    fn to_llvm(&self) -> String {
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
                generate_assign(context, format!("sub i32 0, {}", e))
            },
            Expr::ENot(ref expr) => {
                let e = expr.generate(context);
                generate_assign(context, format!("sub i1 1, {}", e))
            },
            Expr::EOp(ref lhs, ref op, ref rhs) => {
                let rhs = rhs.generate(context);
                let lhs = lhs.generate(context);
                let op = op.generate(context);
                let types = self.get_type(context).to_llvm();
                generate_assign(context, format!("{} {} {}, {} ", op, types, lhs, rhs))
            },
            Expr::EApp(ref s, ref args) => {
                let llvm_args = args_to_llvm(args, context);
                let ret_type = context.get_type(s).get_return_type().to_llvm();
                let res = context.next_register();
                let mut call = format!("{} = call {} @{}(", res, ret_type, s);
                for (i, (val, arg_type)) in llvm_args.into_iter().enumerate() {
                    if i > 0 {
                        call = format!("{}, ", call);
                    }
                    call = format!("{}{} {}", call, arg_type, val);
                }
                context.add_code(format!("{})", call));
                res
            },
            _ => unimplemented!()
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
            Expr::EApp(ref id, _) => context.get_type(id).get_return_type(),
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

impl Generator<String> for BinOp {
    fn generate(&self, _: &mut CGContext) -> String {
        match *self {
            BinOp::Add => String::from("add"),
            BinOp::Sub => String::from("sub"),
            BinOp::Mul => String::from("mul"),
            BinOp::Div => String::from("div"),
            BinOp::Mod => String::from("urem"),
            BinOp::And => String::from("and"),
            BinOp::Or => String::from("or"),
            _ => unimplemented!()
        }
    }
}

impl BinOp {
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