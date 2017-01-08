pub mod type_checker;
mod errors;
mod type_context;

use semantic_analysis::errors::*;
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

pub fn check(program: &Program) -> Result<(), Error> {
    type_checker::check(program).map_err(|err| Error::Type(err))?;
    type_checker::check_return(program).map_err(|err| Error::Return(err))
}