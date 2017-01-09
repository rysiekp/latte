use std::collections::HashMap;
use std::fs::File;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::io::Write;
use ast::Type;

type Vars = HashMap<String, Register>;
type Consts = HashMap<String, Const>;
type Types = HashMap<String, Type>;

#[derive(Clone)]
pub enum Val {
    Const(Const),
    Register(Register),
}

impl Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Val::Const(c) => write!(f, "{}", c),
            Val::Register(r) => write!(f, "{}", r),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Const {
    IConst(i32),
    SConst(i32),
    BConst(bool),
}

impl Display for Const {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Const::IConst(x) => write!(f, "{}", x),
            Const::SConst(x) => write!(f, "@.str{}", x),
            Const::BConst(x) => write!(f, "{}", x as u8),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Register {
    Var(i32),
    Label(i32),
}

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Register::Var(x) => write!(f, "%v{}", x),
            Register::Label(x) => write!(f, "%L{}", x),
        }
    }
}

impl Register {
    pub fn unwrap(&self) -> i32 {
        match *self {
            Register::Var(x) |
            Register::Label(x) => x,
        }
    }

    pub fn next(&self) -> Self {
        match *self {
            Register::Var(x) => Register::Var(x + 1),
            Register::Label(x) => Register::Label(x + 1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CGContext {
    output: Vec<String>,
    vars: Vars,
    types: Types,
    consts: Consts,
    register: Register,
    label: Register,
    last_label: Register,
    next_const: i32,
}

impl CGContext {
    pub fn new() -> Self {
        CGContext {
            vars: Vars::new(),
            types: Types::new(),
            register: Register::Var(1),
            label: Register::Label(1),
            last_label: Register::Label(1),
            output: vec![],
            consts: Consts::new(),
            next_const: 0,
        }
    }

    pub fn get_register(&self, id: &String) -> Register {
        *(self.vars.get(id).unwrap())
    }

    pub fn get_type(&self, id: &String) -> Type {
        self.types.get(id).unwrap().clone()
    }

    pub fn add(&mut self, id: &String, t: &Type) -> Register {
        let reg = self.next_register();
        self.vars.insert(id.clone(), reg);
        self.types.insert(id.clone(), t.clone());
        reg
    }

    pub fn switch_reg(&mut self, id: &String, reg: Register) {
        self.vars.insert(id.clone(), reg);
    }

    pub fn add_function(&mut self, id: &String, t: &Type) {
        self.types.insert(id.clone(), t.clone());
    }

    pub fn next_register(&mut self) -> Register {
        let res = self.register;
        self.register = self.register.next();
        res
    }

    pub fn next_label(&mut self) -> Register {
        let res = self.label;
        self.label = self.label.next();
        res
    }

    pub fn last_label(&self) -> Register {
        self.last_label
    }

    pub fn add_code(&mut self, code: String) {
        self.output.push(code);
    }

    pub fn add_label(&mut self, label: &Register) {
        self.last_label = *label;
        self.add_code(format!("L{}:", label.unwrap()));
    }

    pub fn write(self, file: &mut File) -> io::Result<()> {
        for line in self.output {
            file.write_fmt(format_args!("{}\n", line))?
        }
        Ok(())
    }

    pub fn get_const(&mut self, s: &String) -> Const {
        match self.clone().consts.get(s) {
            Some(c) => *c,
            None => self.add_const(s),
        }
    }

    fn add_const(&mut self, s: &String) -> Const {
        let const_no = Const::SConst(self.next_const);
        self.next_const = self.next_const + 1;

        self.consts.insert(s.clone(), const_no);
        let hex = s.bytes().fold(String::new(), |acc, char| format!("{}\\{:X}", acc, char));
        let code = format!("{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"", const_no, s.len() + 1, hex);
        self.output.insert(0, code);
        const_no
    }

    pub fn in_new_scope<T, F>(&mut self, fun: F) -> T
        where F: Fn(&mut CGContext) -> T {
        let old_vars = self.vars.clone();
        let old_types = self.types.clone();
        let res = fun(self);
        self.vars = old_vars;
        self.types = old_types;
        res
    }

    pub fn in_new_function_scope<T, F>(&mut self, fun: F) -> T
        where F: Fn(&mut CGContext) -> T {
        let old_vars = self.vars.clone();
        let old_types = self.types.clone();
        let res = fun(self);
        self.vars = old_vars;
        self.types = old_types;
        res
    }

}