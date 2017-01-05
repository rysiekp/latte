use std::collections::HashMap;
use std::fs::File;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::io::Write;
use ast::Type;

type Vars = HashMap<String, i32>;
type Types = HashMap<String, Type>;

#[derive(Clone)]
pub enum Val {
    IConst(i32),
    SConst(String),
    BConst(i8),
    Register(i32),
    Label(i32),
}

impl Val {
    pub fn unwrap_register(&self) -> i32 {
        match *self {
            Val::Register(x) |
            Val::Label(x) => x,
            _ => unreachable!(),
        }
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Val::IConst(x) => write!(f, "{}", x),
            Val::SConst(ref x) => write!(f, "{}", x),
            Val::BConst(x) => write!(f, "{}", x),
            Val::Register(x) => write!(f, "%V{}", x),
            Val::Label(x) => write!(f, "%L{}", x),
        }
    }
}

#[derive(Debug)]
pub struct CGContext {
    output: Vec<String>,
    vars: Vars,
    types: Types,
    register: i32,
    label: i32,
    current_fun: i32,
}

impl CGContext {
    pub fn new() -> Self {
        CGContext {
            vars: Vars::new(),
            types: Types::new(),
            register: 1,
            label: 1,
            output: vec![],
            current_fun: 1,
        }
    }

    pub fn get_register(&self, id: &String) -> Val {
        Val::Register(self.vars.get(id).unwrap().clone())
    }

    pub fn get_type(&self, id: &String) -> Type {
        self.types.get(id).unwrap().clone()
    }

    pub fn add(&mut self, id: &String, t: &Type) -> Val {
        let reg = self.next_register();
        self.vars.insert(id.clone(), reg.unwrap_register());
        self.types.insert(id.clone(), t.clone());
        reg
    }

    pub fn switch_reg(&mut self, id: &String, reg: &Val) {
        self.vars.insert(id.clone(), reg.unwrap_register());
    }

    pub fn add_function(&mut self, id: &String, t: &Type) {
        self.types.insert(id.clone(), t.clone());
    }

    pub fn next_register(&mut self) -> Val {
        self.register = self.register + 1;
        Val::Register(self.register - 1)
    }

    pub fn next_label(&mut self) -> Val {
        self.label = self.label + 1;
        Val::Label(self.label - 1)
    }

    pub fn add_code(&mut self, code: String) {
        self.output.push(code);
    }

    pub fn add_label(&mut self, label: &Val) {
        self.output.push(format!("L{}:", label.unwrap_register()));
    }

    pub fn write(self, file: &mut File) -> io::Result<()> {
        for line in self.output {
            file.write_fmt(format_args!("{}\n", line))?
        }
        Ok(())
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
        self.next_fun();
        let res = fun(self);
        self.vars = old_vars;
        self.types = old_types;
        res
    }

    fn next_fun(&mut self) {
        self.current_fun = self.current_fun + 1;
    }
}