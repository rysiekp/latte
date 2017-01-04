use code_generation::generation_context::{CGContext, Val};
use std::fs::File;
use ast::*;

pub fn generate(out: &mut File, stmt: &Stmt) {
    let mut context = CGContext::new();
    stmt.generate(&mut context);
    context.write(out);
}

trait Generator<T> {
    fn generate(&self, context: &mut CGContext) -> T;
}

impl Generator<()> for Stmt {
    fn generate(&self, context: &mut CGContext) -> () {
        match *self {
            Stmt::SAss(ref id, ref expr) => {
                let val = expr.generate(context);
                let rhs_type = expr.get_type(context);
                store_var(id, val, &rhs_type, context)
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
                let ret_type = expr.get_type(context);
                context.add_code(format!("ret {} {}", ret_type, val));
            },
            Stmt::SIf(ref expr, ref block) => {
                let expr_val = expr.generate(context);
                let if_label = context.next_label();
                let after_label = context.next_label();
                context.add_code(format!("br i1 {}, label {}, label {}", expr_val, if_label, after_label));
                context.add_label(if_label);
                block.generate(context);
                context.add_code(format!("br label {}", after_label));
                context.add_label(after_label);
            }
            Stmt::SIfElse(ref expr, ref block1, ref block2) => unimplemented!(),
            Stmt::SWhile(ref expr, ref block) => unimplemented!(),
        }
    }
}

fn manipulate_variable(var: &String, operation: String, context: &mut CGContext) -> () {
    let ptr = read_var(var, context);
    let val = context.next_register();
    let var_type = context.get_type(var);
    context.add_code(format!("{} = {}, {} {}", val, operation, var_type, ptr));
    store_var(var, val, &var_type, context)
}

fn read_var(var: &String, context: &mut CGContext) -> Val {
    let var_reg = context.get_register(var);
    let res_reg = context.next_register();
    let var_type = context.get_type(var);
    context.add_code(format!("{} = load {}, {}* {}", res_reg, var_type, var_type, var_reg));
    res_reg
}

fn store_var(var: &String, val: Val, var_type: &Type, context: &mut CGContext) -> () {
    let reg = context.get_register(var);
    context.add_code(format!("store {} {}, {}* {}", var_type, val, var_type, reg))
}

fn generate_item(item_type: &Type, item: &Item, context: &mut CGContext) -> () {
    let reg = context.add(&item.get_id(), item_type);
    context.add_code(format!("{} = alloca {}", reg, item_type));
    match *item {
        Item::Init(ref id, ref expr) => Stmt::SAss(id.clone(), expr.clone()).generate(context),
        Item::NoInit(ref id) => Stmt::SAss(id.clone(), item_type.default_value()).generate(context),
    }
}

impl Generator<()> for Block {
    fn generate(&self, context: &mut CGContext) -> () {
        let Block(ref stmts) = *self;
        for stmt in stmts {
            stmt.generate(context);
            if stmt.early_return() {
                break;
            }
        }
    }
}

impl Stmt {
    pub fn early_return(&self) -> bool {
        match *self {
            Stmt::SVRet | Stmt::SRet(_) => true,
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
                let res_reg = context.next_register();
                context.add_code(format!("{} = sub i32 0, i32 {}", res_reg, e));
                res_reg
            },
            Expr::ENot(ref expr) => {
                let e = expr.generate(context);
                let res_reg = context.next_register();
                context.add_code(format!("{} = sub i1 1, i1 {}", res_reg, e));
                res_reg
            },
            Expr::EOp(ref lhs, ref op, ref rhs) => {
                let rhs = rhs.generate(context);
                let lhs = lhs.generate(context);
                let op = op.generate(context);
                let reg = context.next_register();
                let types = self.get_type(context);
                context.add_code(format!("{} = {} {} {}, {} {}", reg,  op, types, lhs, types, rhs));
                reg
            },
            Expr::EApp(ref s, ref args) => {
                let llvm_args = args_to_llvm(args, context);
                let ret_type = context.get_type(s).get_return_type();
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
            BinOp::Div => Some(Type::TInt),
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