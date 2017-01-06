use std::collections::HashMap;
use std::fs::File;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::io::Write;
use ast::Type;
use std::ops::Deref;

type Vars = HashMap<String, Register>;
type Types = HashMap<String, Type>;

#[derive(Clone)]
pub enum Val {
    IConst(i32),
    SConst(String),
    BConst(i8),
    Register(Register),
}

#[derive(Clone, Copy, Debug)]
pub enum Register {
    Var(i32),
    Label(i32),
    FuncRet(i32),
    RetVal(i32),
    RetAddr(i32),
}

impl Register {
    pub fn unwrap(&self) -> i32 {
        match *self {
            Register::Var(x) |
            Register::Label(x) |
            Register::FuncRet(x) |
            Register::RetVal(x) |
            Register::RetAddr(x) => x,
        }
    }

    pub fn next(&self) -> Self {
        match *self {
            Register::Var(x) => Register::Var(x + 1),
            Register::Label(x) => Register::Label(x + 1),
            Register::FuncRet(x) => Register::FuncRet(x + 1),
            Register::RetVal(x) => Register::RetVal(x + 1),
            Register::RetAddr(x) => Register::RetAddr(x + 1),
        }
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Val::IConst(ref x) => write!(f, "{}", x),
            Val::SConst(ref x) => write!(f, "{}", x),
            Val::BConst(ref x) => write!(f, "{}", x),
            Val::Register(r) => write!(f, "{}", r),
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Register::Var(x) => write!(f, "%v{}", x),
            Register::Label(x) => write!(f, "%L{}", x),
            Register::FuncRet(x) => write!(f, "%FR{}", x),
            Register::RetVal(x) => write!(f, "%rv{}", x),
            Register::RetAddr(x) => write!(f, "%ra{}", x),
        }
    }
}

#[derive(Debug)]
pub struct CGContext {
    output: Vec<String>,
    vars: Vars,
    types: Types,
    register: Register,
    label: Register,
    current_fun: i32,
}

impl CGContext {
    pub fn new() -> Self {
        CGContext {
            vars: Vars::new(),
            types: Types::new(),
            register: Register::Var(1),
            label: Register::Label(1),
            output: vec![],
            current_fun: 1,
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

    pub fn add_code(&mut self, code: String) {
        self.output.push(code);
    }

    pub fn add_label(&mut self, label: &Register) {
        self.add_code(format!("L{}:", label.unwrap()));
    }

    pub fn add_fun_prologue(&mut self, func_type: &Type) {
        let current_fun = self.current_fun;
        match func_type {
            &Type::TVoid => (),
            _ => self.add_code(format!("{} = alloca {}", Register::RetAddr(current_fun), func_type.to_llvm()))
        }
    }

    pub fn add_fun_epilogue(&mut self, func_type: &Type) {
        let current_fun = self.current_fun;
        self.add_code(format!("br label {}", Register::FuncRet(current_fun)));
        self.add_code(format!("FR{}:", current_fun));
        match func_type {
            &Type::TVoid => self.add_code(format!("ret void")),
            _ => {
                self.add_code(format!("{} = load {}, {}* {}",
                                      Register::RetVal(current_fun),
                                      func_type.to_llvm(),
                                      func_type.to_llvm(),
                                      Register::RetAddr(current_fun)));
                self.add_code(format!("ret {} {}", func_type.to_llvm(), Register::RetVal(current_fun)));
            }
        }
    }

    pub fn current_func_ret_addr(&self) -> Register {
        Register::RetAddr(self.current_fun)
    }

    pub fn current_func_ret_label(&self) -> Register {
        Register::FuncRet(self.current_fun)
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