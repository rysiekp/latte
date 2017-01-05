mod generation_context;
mod code_generator;

use std::fs::File;
use ast::*;

pub fn run(out: &mut File, p: &Program) {
    code_generator::generate(out, p);
}