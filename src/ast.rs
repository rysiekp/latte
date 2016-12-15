#[derive(Debug)]
pub struct Program(pub Vec<Def>);

#[derive(Debug)]
pub enum Def {
    DFun(Type, String, Vec<Arg>, Block),
}

#[derive(Debug)]
pub struct Arg(pub Type, pub String);

#[derive(Debug)]
pub struct Block(pub Vec<Stmt>);

#[derive(Debug)]
pub enum Stmt {
    SDecl(Type, Vec<Item>),
    SAss(String, Expr),
    SInc(String),
    SDecr(String),
    SRet(Expr),
    SVRet,
    SIf(Expr, Block),
    SIfElse(Expr, Block, Block),
    SWhile(Expr, Block),
    SExpr(Expr),
}

#[derive(Debug)]
pub enum Item {
    NoInit(String),
    Init(String, Expr),
}

#[derive(Debug)]
pub enum Expr {
    EVar(String),
    EIntLit(i32),
    EBoolLit(bool),
    EStringLit(String),
    EApp(String, Vec<Expr>),
    ENeg(Box<Expr>),
    ENot(Box<Expr>),
    EPredef(Predef),
    EOp(Box<Expr>, BinOp, Box<Expr>),
}

#[derive(Debug)]
pub enum Predef {
    PrintInt(Box<Expr>),
    PrintString(Box<Expr>),
    Error,
    ReadInt,
    ReadString,
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    LT,
    LE,
    EQ,
    NEQ,
    GT,
    GE,
    And,
    Or,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    TInt,
    TString,
    TBool,
    TVoid,
    TFunc(Box<Type>, Vec<Type>)
}


