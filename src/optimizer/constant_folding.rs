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

                if !lhs.is_constant() ||
                    !rhs.is_constant() ||
                    (rhs == Expr::EIntLit(0) && (op == BinOp::Div || op == BinOp::Mod)) {
                    Expr::EOp(Box::new(lhs), op, Box::new(rhs))
                } else {
                    op.apply(lhs, rhs)
                }
            }
            _ => self
        }
    }
}



impl BinOp {
    fn apply(self, lhs: Expr, rhs: Expr) -> Expr {
        match self {
            BinOp::Add => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EIntLit(x + y),
                (Expr::EStringLit(x), Expr::EStringLit(y)) => Expr::EStringLit(format!("{}{}", x, y)),
                _ => unreachable!()
            },
            BinOp::Sub => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EIntLit(x - y),
                _ => unreachable!(),
            },
            BinOp::Mul => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EIntLit(x * y),
                _ => unreachable!(),
            },
            BinOp::Div => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EIntLit(x / y),
                _ => unreachable!(),
            },
            BinOp::Mod => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EIntLit(x % y),
                _ => unreachable!(),
            },
            BinOp::And => match (lhs, rhs) {
                (Expr::EBoolLit(x), Expr::EBoolLit(y)) => Expr::EBoolLit(x && y),
                _ => unreachable!(),
            },
            BinOp::Or => match (lhs, rhs) {
                (Expr::EBoolLit(x), Expr::EBoolLit(y)) => Expr::EBoolLit(x || y),
                _ => unreachable!(),
            },
            BinOp::EQ => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EBoolLit(x == y),
                (Expr::EStringLit(x), Expr::EStringLit(y)) => Expr::EBoolLit(x == y),
                (Expr::EBoolLit(x), Expr::EBoolLit(y)) => Expr::EBoolLit(x == y),
                _ => unreachable!(),
            },
            BinOp::NEQ => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EBoolLit(x != y),
                (Expr::EStringLit(x), Expr::EStringLit(y)) => Expr::EBoolLit(x != y),
                (Expr::EBoolLit(x), Expr::EBoolLit(y)) => Expr::EBoolLit(x != y),
                _ => unreachable!(),
            },
            BinOp::LT => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EBoolLit(x < y),
                _ => unreachable!(),
            },
            BinOp::LE => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EBoolLit(x <= y),
                _ => unreachable!(),
            },
            BinOp::GT => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EBoolLit(x > y),
                _ => unreachable!(),
            },
            BinOp::GE => match (lhs, rhs) {
                (Expr::EIntLit(x), Expr::EIntLit(y)) => Expr::EBoolLit(x >= y),
                _ => unreachable!(),
            },
        }
    }
}