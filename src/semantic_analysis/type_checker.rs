use std::fmt;
use std::marker;
use ast::*;
use semantic_analysis::errors::{TError, RError, ErrStack, missing_return};
use semantic_analysis::type_context::{TCContext};

pub fn check(program: &Program) -> TError<()> {
    program.do_check(&mut TCContext::new())
}

trait TypeCheck<T> where Self: fmt::Display + marker::Sized {
    fn check(&self, context: &mut TCContext) -> TError<T> {
        self.do_check(context).map_err(|err| err.add_to_stack(self))
    }

    fn do_check(&self, context: &mut TCContext) -> TError<T>;
}

pub trait Returns {
    fn check_return(&self) -> bool;
}

impl Returns for Stmt {
    fn check_return(&self) -> bool {
        match *self {
            Stmt::SVRet |
            Stmt::SRet(_) => true,
            Stmt::SIfElse(_, ref b1, ref b2) => b1.check_return() && b2.check_return(),
            Stmt::SBlock(ref stmts) => stmts.check_return(),
            _ => false
        }
    }
}

impl Returns for Vec<Stmt> {
    fn check_return(&self) -> bool {
        self.iter().any(Stmt::check_return)
    }
}

pub fn check_return(program: &Program) -> RError {
    for def in &program.0 {
        match *def {
            Def::DFun(ref t, ref name, _, ref body) =>
                if t != &Type::TVoid && !body.check_return() {
                    return Err(missing_return(name));
                }
        }
    }
    Ok(())
}

impl TypeCheck<()> for Program {
    fn do_check(&self, context: &mut TCContext) -> TError<()> {
        let Program(ref defs) = *self;

        for def in defs {
            match *def {
                Def::DFun(_, ref name, _, _) => {
                    context.add(name, &def.get_type())?;
                }
            }
        }

        for def in defs {
            match *def {
                Def::DFun(ref ret_type, _, _, _) =>
                    context.in_new_function(ret_type, |mut ctx| def.check(&mut ctx))?
            };
        }

        check_main_exists(context)
    }
}

fn check_main_exists(context: &mut TCContext) -> TError<()> {
    if let Ok(main_type) = context.get(&String::from("main")) {
        if main_type == Type::TFunc(Box::new(Type::TInt), vec![]) {
            Ok(())
        } else {
            Err(ErrStack::invalid_main_type())
        }
    } else {
        Err(ErrStack::missing_main())
    }
}

impl TypeCheck<()> for Def {
    fn do_check(&self, context: &mut TCContext) -> TError<()> {
        match *self {
            Def::DFun(_, _, ref args, ref block) => {
                for arg in args {
                    arg.do_check(context)?;
                    context.add(&arg.1, &arg.0)?;
                }
                for stmt in block {
                    stmt.check(context)?;
                }
                Ok(())
            }
        }
    }
}

impl TypeCheck<()> for Arg {
    fn do_check(&self, _: &mut TCContext) -> TError<()> {
        let &Arg(ref t, _) = self;
        if *t == Type::TVoid {
            Err(ErrStack::void_argument())
        } else {
            Ok(())
        }
    }
}

impl Def {
    fn get_type(&self) -> Type {
        match *self {
            Def::DFun(ref ret_type, _, ref args, _) => {
                Type::TFunc(Box::new(ret_type.clone()), args.into_iter().map(|arg| arg.0.clone()).collect())
            }
        }
    }
}

impl TypeCheck<()> for Stmt {
    fn do_check(&self, context: &mut TCContext) -> TError<()> {
        match *self {
            Stmt::SExpr(ref expr) => {
                expr.check(context)?;
            },
            Stmt::SAss(ref var, ref expr) => {
                expect_one_of(context.get(var)?, expr.check(context)?, vec![Type::TInt, Type::TString, Type::TBool])?;
            },
            Stmt::SDecl(ref decl_type, ref decls) => {
                if decl_type == &Type::TVoid {
                    return Err(ErrStack::void_declaration())
                }
                for item in decls {
                    check_decl(item, decl_type, context)?;
                }
            },
            Stmt::SInc(ref var) |
            Stmt::SDecr(ref var) => {
                expect(context.get(var)?, Type::TInt)?;
            },
            Stmt::SIf(ref cond, ref block) |
            Stmt::SWhile(ref cond, ref block) => {
                expect(cond.check(context)?, Type::TBool)?;
                context.in_new_scope(|mut ctx| block.do_check(&mut ctx))?;
            },
            Stmt::SIfElse(ref cond, ref if_block, ref else_block) => {
                expect(cond.check(context)?, Type::TBool)?;
                context.in_new_scope(|mut ctx| if_block.do_check(&mut ctx))?;
                context.in_new_scope(|mut ctx| else_block.do_check(&mut ctx))?;
            },
            Stmt::SVRet => {
                expect(context.return_type(), Type::TVoid)?;
            },
            Stmt::SRet(ref expr) => {
                if context.return_type() == Type::TVoid {
                    return Err(ErrStack::void_return_value())
                }
                expect(expr.check(context)?, context.return_type())?;
            },
            Stmt::SBlock(ref stmts) => {
                context.in_new_scope(|mut ctx| {
                    for stmt in stmts {
                        stmt.check(ctx)?;
                    };
                    Ok(())
                })?;
            },
            Stmt::Empty => (),
        };
        Ok(())
    }
}

fn check_decl(item: &Item, decl_type: &Type, context: &mut TCContext) -> TError<()> {
    match *item {
        Item::NoInit(ref var) => context.add(var, decl_type),
        Item::Init(ref var, ref expr) => {
            expect(expr.check(context)?, decl_type.clone())?;
            context.add(var, decl_type)
        }
    }
}

impl TypeCheck<Type> for Expr {
    fn do_check(&self, context: &mut TCContext) -> TError<Type> {
        match *self {
            Expr::EVar(ref var) => Ok(context.get(var)?),
            Expr::EBoolLit(_) => Ok(Type::TBool),
            Expr::EIntLit(_) => Ok(Type::TInt),
            Expr::EStringLit(_) => Ok(Type::TString),
            Expr::ENeg(ref expr) => expect(expr.do_check(context)?, Type::TInt),
            Expr::ENot(ref expr) => expect(expr.do_check(context)?, Type::TBool),
            Expr::EOp(ref lhs, op, ref rhs) => {
                let lhs_type = lhs.do_check(context)?;
                let rhs_type = rhs.do_check(context)?;
                match op {
                    BinOp::Sub |
                    BinOp::Mul |
                    BinOp::Div |
                    BinOp::Mod =>
                        expect(lhs_type, Type::TInt).and(expect(rhs_type, Type::TInt)),
                    BinOp::GE |
                    BinOp::GT |
                    BinOp::LE |
                    BinOp::LT => {
                        expect(lhs_type, Type::TInt).and(expect(rhs_type, Type::TInt))?;
                        Ok(Type::TBool)
                    }
                    BinOp::Add =>
                        expect_one_of(lhs_type, rhs_type, vec![Type::TInt, Type::TString]),
                    BinOp::And | BinOp::Or =>
                        expect(lhs_type, Type::TBool).and(expect(rhs_type, Type::TBool)),
                    BinOp::EQ | BinOp::NEQ => {
                        expect_one_of(lhs_type, rhs_type, vec![Type::TInt, Type::TString, Type::TBool])?;
                        Ok(Type::TBool)
                    },
                }
            },
            Expr::EPredef(ref predef) => predef.do_check(context),
            Expr::EApp(ref fun, ref args) => check_function_call(fun, args, context),
        }
    }
}

fn check_function_call(fun: &String, args: &Vec<Expr>, context: &mut TCContext) -> TError<Type> {
    if let Type::TFunc(ret_type, expected_types) = context.get(fun)? {
        if args.len() != expected_types.len() {
            return Err(ErrStack::invalid_argument_number(fun, args.len(), expected_types.len()));
        }

        for (number, (ref actual_type, ref arg)) in expected_types.into_iter().zip(args).enumerate() {
            let arg_type = arg.check(context)?;
            if arg_type.clone() != actual_type.clone() {
                return Err(ErrStack::invalid_call_type(fun, number, arg_type, actual_type.clone()));
            }
        }
        Ok(*ret_type)
    } else {
        Err(ErrStack::not_a_function(fun))
    }
}

impl TypeCheck<Type> for Predef {
    fn do_check(&self, context: &mut TCContext) -> TError<Type> {
        match *self {
            Predef::Error => Ok(Type::TVoid),
            Predef::ReadInt => Ok(Type::TInt),
            Predef::ReadString => Ok(Type::TString),
            Predef::PrintInt(ref arg) => {
                expect(arg.check(context)?, Type::TInt)?;
                Ok(Type::TVoid)
            },
            Predef::PrintString(ref arg) => {
                expect(arg.check(context)?, Type::TString)?;
                Ok(Type::TVoid)
            },
        }
    }
}

fn expect_one_of(lhs: Type, rhs: Type, expected: Vec<Type>) -> TError<Type> {
    if lhs == rhs && expected.contains(&lhs) {
        Ok(lhs)
    } else {
        Err(ErrStack::op_not_defined(lhs, rhs))
    }
}

fn expect(given: Type, expected: Type) -> TError<Type> {
    if given == expected {
        Ok(expected)
    } else {
        Err(ErrStack::incompatible(given, expected))
    }
}