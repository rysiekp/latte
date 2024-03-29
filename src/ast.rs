#[derive(Debug, Clone)]
pub struct Program(pub Vec<Def>);

#[derive(Debug, Clone)]
pub enum Def {
    DFun(Type, String, Vec<Arg>, Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub struct Arg(pub Type, pub String);

#[derive(Debug, Clone)]
pub enum Stmt {
    Empty,
    SDecl(Type, Vec<Item>),
    SAss(String, Expr),
    SInc(String),
    SDecr(String),
    SRet(Expr),
    SVRet,
    SIf(Expr, Box<Stmt>),
    SIfElse(Expr, Box<Stmt>, Box<Stmt>),
    SWhile(Expr, Box<Stmt>),
    SExpr(Expr),
    SBlock(Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub enum Item {
    NoInit(String),
    Init(String, Expr),
}

impl Item {
    pub fn get_id(&self) -> String {
        match *self {
            Item::NoInit(ref id) => id.clone(),
            Item::Init(ref id, _) => id.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Predef {
    PrintInt(Box<Expr>),
    PrintString(Box<Expr>),
    Error,
    ReadInt,
    ReadString,
}

impl Predef {
    pub fn get_type(&self) -> Type {
        match *self {
            Predef::ReadInt => Type::TInt,
            Predef::ReadString => Type::TString,
            _ => Type::TVoid,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
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


