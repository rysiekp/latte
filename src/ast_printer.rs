use std::fmt::Display;
use std::fmt;
use ast::*;

fn print_list<T>(items: &Vec<T>) -> String where T: Display{
    let mut s = String::new();
    for item in items {
        if !s.is_empty() {
            s.push_str(", ");
        }
        s.push_str(format!("{}", item).as_str());
    }
    s
}

pub trait Print {
    fn print_first(&self, fmt: &mut fmt::Formatter) {
        self.print(&String::from(""), fmt);
    }
    fn print(&self, indent: &String, fmt: &mut fmt::Formatter);
}

impl<T> Print for Vec<T> where T: Print {
    fn print(&self, indent: &String, fmt: &mut fmt::Formatter) {
        self.into_iter().map(|item| item.print(indent, fmt)).collect::<Vec<()>>();
    }
}

impl Print for Program {
    fn print(&self, indent: &String, fmt: &mut fmt::Formatter) {
        let Program(ref definitions) = *self;
        definitions.print(indent, fmt);
    }
}

impl fmt::Display for Program {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.print_first(fmt);
        Ok(())
    }
}

impl Print for Def {
    fn print(&self, indent: &String, fmt: &mut fmt::Formatter) {
        match *self {
            Def::DFun(ref t, ref f, ref args, ref block) => {
                writeln!(fmt, "{}{} {}({})", indent, t, f, print_list(&args)).unwrap();
                block.print(indent, fmt);
            }
        }
    }
}

impl fmt::Display for Def {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.print_first(fmt);
        Ok(())
    }
}

impl Print for Block {
    fn print(&self, indent: &String, fmt: &mut fmt::Formatter) {
        writeln!(fmt, "{}{}", indent, '{').unwrap();
        let Block(ref stmts) = *self;
        stmts.print(&format!("\t{}", indent), fmt);
        writeln!(fmt, "{}{}", indent, '}').unwrap();
    }
}

impl fmt::Display for Block {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.print_first(fmt);
        Ok(())
    }
}

impl Print for Stmt {
    fn print(&self, indent: &String, fmt: &mut fmt::Formatter) {
        write!(fmt, "{}", indent).unwrap();
        match *self {
            Stmt::SDecl(ref t, ref items) => writeln!(fmt, "{} {};", t, print_list(&items)).unwrap(),
            Stmt::SAss(ref var, ref expr) => writeln!(fmt, "{} = {};", var, *expr).unwrap(),
            Stmt::SInc(ref var) => writeln!(fmt, "{}++;", var).unwrap(),
            Stmt::SDecr(ref var) => writeln!(fmt, "{}--;", var).unwrap(),
            Stmt::SExpr(ref expr) => writeln!(fmt, "{};", *expr).unwrap(),
            Stmt::SIf(ref expr, ref block) => {
                writeln!(fmt, "if ({})", *expr).unwrap();
                block.print(indent, fmt);
            },
            Stmt::SIfElse(ref expr, ref if_block, ref else_block) => {
                writeln!(fmt, "if ({})", *expr).unwrap();
                if_block.print(indent, fmt);
                writeln!(fmt, "{}else", indent).unwrap();
                else_block.print(indent, fmt);
            },
            Stmt::SWhile(ref expr, ref block) => {
                writeln!(fmt, "while ({})", *expr).unwrap();
                block.print(indent, fmt);
            },
            Stmt::SVRet => writeln!(fmt, "return;").unwrap(),
            Stmt::SRet(ref expr) => writeln!(fmt, "return {};", *expr).unwrap(),
        };
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.print_first(fmt);
        Ok(())
    }
}

impl Print for Predef {
    fn print(&self, _: &String, fmt: &mut fmt::Formatter) {
        match *self {
            Predef::Error => write!(fmt, "error()").unwrap(),
            Predef::PrintInt(ref expr) => write!(fmt, "printInt({})", *expr).unwrap(),
            Predef::PrintString(ref expr) => write!(fmt, "printString({})", *expr).unwrap(),
            Predef::ReadInt => write!(fmt, "readInt()").unwrap(),
            Predef::ReadString => write!(fmt, "readString()").unwrap(),
        };
    }
}

impl fmt::Display for Predef {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.print_first(fmt);
        Ok(())
    }
}

impl Display for Expr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Expr::EVar(ref i) => format!("{}", i),
            Expr::EIntLit(ref i) => format!("{}", i),
            Expr::EBoolLit(ref b) => format!("{}", b),
            Expr::EStringLit(ref s) => format!("\"{}\"", s),
            Expr::EApp(ref f, ref args) => format!("{}({})", f, print_list(args)),
            Expr::ENeg(ref e) => format!("-{}", *e),
            Expr::ENot(ref e) => format!("!{}", *e),
            Expr::EOp(ref lhs, ref op, ref rhs) => format!("({} {} {})", *lhs, op, *rhs),
            Expr::EPredef(ref predef) => format!("{}", predef),
        };
        write!(fmt, "{}", s)
    }
}

impl Display for Item {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Item::Init(ref var, ref e) => format!("{} = {}", var, e),
            Item::NoInit(ref var) => format!("{}", var),
        };
        write!(fmt, "{}", s)
    }
}

impl Display for Arg {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let Arg(ref t, ref id) = *self;
        write!(fmt, "{} {}", t, id)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Type::TInt => "i32",
            Type::TString => "string",
            Type::TBool => "i1",
            Type::TVoid => "void",
            _ => "function",
        };
        write!(fmt, "{}", s)
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::LT => "<",
            BinOp::GT => ">",
            BinOp::LE => "<=",
            BinOp::GE => ">=",
            BinOp::EQ => "==",
            BinOp::NEQ => "!=",
            BinOp::And => "&&",
            BinOp::Or => "||",
        };
        write!(fmt, "{}", s)
    }
}