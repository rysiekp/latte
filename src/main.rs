extern crate lalrpop_util;

pub mod ast;
mod ast_printer;
mod semantic_analysis;
mod parser;
mod code_generation;

use parser::parser::*;
use parser::parser_errors::*;
use std::fs::File;

fn main() {
    let input = "int f(int x, bool y) {\
                    int z = x + 1;\
                    if (y) {\
                        z = 0;\
                    }\
                    \
                    return z;\
                }\
                bool g(int x, bool y) {\
                    bool z = true;\
                    if (y) {\
                        z = false;\
                    }\
                    \
                    return z;\
                }\
                int main() {\
                    return 0;\
                }";
    let filename = "test.bc";
    let mut output = File::create(filename).unwrap();
    match parse_Program(input) {
        Ok(program) => match semantic_analysis::check(&program) {
            Ok(_) => code_generation::run(&mut output, &program),
            Err(err) => println!("{}", err),
        },
        Err(err) => print_error(err, input),
    }
}
