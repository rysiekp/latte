use std::fmt;
use ast::Type;

pub type TError<T> = Result<T, ErrStack>;
pub type RError = Result<(), String>;

pub struct ErrStack {
    err: String,
    stack: Vec<String>,
}

impl ErrStack {
    pub fn new(err: String) -> ErrStack {
        ErrStack {
            err: err,
            stack: vec![],
        }
    }

    pub fn undeclared(id: &String) -> ErrStack {
        Self::new(format!("use of undeclared identifier {}", id))
    }

    pub fn redefinition(id: &String) -> ErrStack {
        Self::new(format!("redefinition of identifier {}", id))
    }

    pub fn op_not_defined(lhs: Type, rhs: Type) -> ErrStack {
        Self::new(format!("operation not defined for {} and {}", lhs, rhs))
    }

    pub fn incompatible(given: Type, expected: Type) -> ErrStack {
        Self::new(format!("incompatible types, cannot convert {} to {}", given, expected))
    }

    pub fn not_a_function(id: &String) -> ErrStack {
        Self::new(format!("{} is not a function", id))
    }

    pub fn invalid_call_type(fun: &String, arg_no: usize, given: Type, expected: Type) -> ErrStack {
        Self::new(format!("invalid argument type in call to function {}, parameter {} cannot be converted from {} to {}", fun, arg_no, given, expected))
    }

    pub fn invalid_argument_number(fun: &String, args: usize, expected: usize) -> ErrStack {
        Self::new(format!("invalid parameter count in call to function {}, expected {}, received {}", fun, expected, args))
    }

    pub fn invalid_main_type() -> ErrStack {
        Self::new(format!("invalid type of the main function"))
    }

    pub fn missing_main() -> ErrStack {
        Self::new(format!("main function is missing"))
    }

    pub fn void_return_value() -> ErrStack {
        Self::new(format!("void function cannot return value"))
    }

    pub fn void_argument() -> ErrStack {
        Self::new(format!("arguments cannot be of type void"))
    }

    pub fn void_declaration() -> ErrStack {
        Self::new(format!("cannot declare variable with type void"))
    }

    pub fn add_to_stack<T: fmt::Display>(mut self, within: &T) -> ErrStack {
        self.stack.push(format!("{}", within));
        self
    }
}

pub fn missing_return(function: &String) -> String {
    format!("not all execution paths yield value in function {}", function)
}

impl fmt::Display for ErrStack {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "err: {}", self.err)?;
        for item in &self.stack {
            writeln!(fmt, "in:\n{}", item)?;
        }
        Ok(())
    }
}