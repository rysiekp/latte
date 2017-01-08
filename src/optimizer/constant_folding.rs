use ast::*;

pub trait Fold {
    fn fold(self) -> Self;
}

trait IsConstant {
    fn is_constant(&self) -> bool;
}

impl Fold for Program {
    fn fold(self) -> Program {
        Program(self.0.into_iter().map(Def::fold).collect())
    }
}

impl Fold for Def {
    fn fold(self) -> Def {
        match self {
            Def::DFun(t, name, args, stmts) => Def::DFun(t, name, args, stmts.fold()),
        }
    }
}

impl Fold for Stmt {
    fn fold(self) -> Stmt {
        match self {
            Stmt::SAss(stmt, expr) => Stmt::SAss(stmt, expr.fold()),
            Stmt::SBlock(stmts) => Stmt::SBlock(stmts.fold()),
            Stmt::SDecl(t, items) => Stmt::SDecl(t, items.into_iter().map(Item::fold).collect()),
            Stmt::SExpr(expr) => Stmt::SExpr(expr.fold()),
            Stmt::SRet(expr) => Stmt::SRet(expr.fold()),
            Stmt::SIf(cond, block) => {
                let cond = cond.fold();
                match cond {
                    Expr::EBoolLit(true) => Stmt::SBlock(vec![*block]),
                    Expr::EBoolLit(false) => Stmt::Empty,
                    _ => Stmt::SIf(cond, block),
                }
            },
            Stmt::SWhile(cond, block) => {
                let cond = cond.fold();
                match cond {
                    Expr::EBoolLit(false) => Stmt::Empty,
                    _ => Stmt::SWhile(cond, block),
                }
            },
            Stmt::SIfElse(cond, if_block, else_block) => {
                let cond = cond.fold();
                match cond {
                    Expr::EBoolLit(true) => Stmt::SBlock(vec![*if_block]),
                    Expr::EBoolLit(false) => Stmt::SBlock(vec![*else_block]),
                    _ => Stmt::SIfElse(cond, if_block, else_block),
                }
            },
            _ => self
        }
    }
}

impl Fold for Vec<Stmt> {
    fn fold(self) -> Vec<Stmt> {
        self.into_iter().map(Stmt::fold).collect()
    }
}

impl Fold for Item {
    fn fold(self) -> Item {
        match self {
            Item::Init(s, e) => Item::Init(s, e.fold()),
            _ => self,
        }
    }
}

impl IsConstant for Expr {
    fn is_constant(&self) -> bool {
        match *self {
            Expr::EVar(_) |
            Expr::EPredef(_) |
            Expr::EApp(_, _)  => false,
            Expr::EBoolLit(_) |
            Expr::EIntLit(_) |
            Expr::EStringLit(_) => true,
            Expr::ENeg(ref e) |
            Expr::ENot(ref e) => e.is_constant(),
            Expr::EOp(ref lhs, _, ref rhs) => lhs.is_constant() && rhs.is_constant(),
        }
    }
}

impl Fold for Expr {
    fn fold(self) -> Expr {
        match self {
            Expr::ENot(expr) => {
                let expr = expr.fold();
                match expr {
                    Expr::EBoolLit(b) => Expr::EBoolLit(!b),
                    _ => Expr::ENot(Box::new(expr))
                }
            },
            Expr::ENeg(expr) => {
                let expr = expr.fold();
                match expr {
                    Expr::EIntLit(x) => Expr::EIntLit(-x),
                    _ => Expr::ENeg(Box::new(expr))
                }
            },
            Expr::EOp(lhs, op, rhs) => {
                let lhs = lhs.fold();
                let rhs = rhs.fold();
                op.apply(lhs, rhs)
            },
            Expr::EApp(s, args) => Expr::EApp(s, args.into_iter().map(Expr::fold).collect()),
            Expr::EPredef(p) => Expr::EPredef(p.fold()),
            _ => self
        }
    }
}

impl Fold for Predef {
    fn fold(self) -> Predef {
        match self {
            Predef::Error |
            Predef::ReadInt |
            Predef::ReadString => self,
            Predef::PrintInt(e) => Predef::PrintInt(Box::new(e.fold())),
            Predef::PrintString(e) => Predef::PrintString(Box::new(e.fold())),
        }
    }
}

impl BinOp {
    fn apply(self, lhs: Expr, rhs: Expr) -> Expr {
        let op = Expr::EOp(Box::new(lhs.clone()), self, Box::new(rhs.clone()));
        match self {
            BinOp::Add => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EIntLit(x + y),
                (Expr::EStringLit(x), Expr::EStringLit(y)) => Expr::EStringLit(format!("{}{}", x, y)),
                _ => op
            },
            BinOp::Sub => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EIntLit(x - y),
                _ => op
            },
            BinOp::Mul => match (lhs, rhs) {
                (Expr::EIntLit(0), _) |
                (_, Expr::EIntLit(0)) => Expr::EIntLit(0),
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EIntLit(x * y),
                _ => op
            },
            BinOp::Div => match (lhs, rhs) {
                (e, Expr::EIntLit(1)) => e,
                (Expr::EIntLit(x), Expr::EIntLit(y)) if y != 0 => Expr::EIntLit(x / y),
                _ => op
            },
            BinOp::Mod => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) if y != 0 => Expr::EIntLit(x % y),
                _ => op
            },
            BinOp::And => match (lhs, rhs) {
                (Expr::EBoolLit(false), _) |
                (_, Expr::EBoolLit(false)) => Expr::EBoolLit(false),
                (Expr::EBoolLit(x), Expr::EBoolLit(y)) => Expr::EBoolLit(x && y),
                _ => op
            },
            BinOp::Or => match (lhs, rhs) {
                (Expr::EBoolLit(true), _) |
                (_, Expr::EBoolLit(true)) => Expr::EBoolLit(true),
                (Expr::EBoolLit(x), Expr::EBoolLit(y)) => Expr::EBoolLit(x || y),
                _ => op
            },
            BinOp::EQ => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EBoolLit(x == y),
                (Expr::EStringLit(x), Expr::EStringLit(y)) => Expr::EBoolLit(x == y),
                (Expr::EBoolLit(x), Expr::EBoolLit(y)) => Expr::EBoolLit(x == y),
                _ => op
            },
            BinOp::NEQ => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EBoolLit(x != y),
                (Expr::EStringLit(x), Expr::EStringLit(y)) => Expr::EBoolLit(x != y),
                (Expr::EBoolLit(x), Expr::EBoolLit(y)) => Expr::EBoolLit(x != y),
                _ => op
            },
            BinOp::LT => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EBoolLit(x < y),
                _ => op
            },
            BinOp::LE => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EBoolLit(x <= y),
                _ => op
            },
            BinOp::GT => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EBoolLit(x > y),
                _ => op
            },
            BinOp::GE => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EBoolLit(x >= y),
                _ => op
            },
        }
    }
}