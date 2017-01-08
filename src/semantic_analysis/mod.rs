pub mod type_checker;
mod errors;
mod type_context;

use semantic_analysis::errors::*;
use utils::print_err;
use ast::Program;
use std::fmt;

pub enum Error {
    Type(ErrStack),
    Return(String)
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Type(ref stack) => write!(fmt, "{}", stack),
            Error::Return(ref error) => write!(fmt, "{}", error),
        }
    }
}

pub fn check_types(program: &Program) {
    match type_checker::check(program).map_err(|err| Error::Type(err)) {
        Ok(_) => (),
        Err(err) => print_err(format!("{}", err)),
    }
}

pub fn check_returns(program: &Program) {
    match type_checker::check_return(program).map_err(|err| Error::Return(err)) {
        Ok(_) => (),
        Err(err) => print_err(format!("{}", err)),
    }
}