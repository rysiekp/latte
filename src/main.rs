extern crate lalrpop_util;

pub mod ast;
mod ast_printer;
mod semantic_analysis;
mod parser;

use parser::parser::*;
use parser::parser_errors::*;

fn main() {
    let input = "int main() {\n\
                    int __c123__ = readInt();\n\
                    printInt(__c123__);
                    __c123__--;\n\
                    return 0;\n\
                }\n\
                int f(int x) {\n\
                    if (x < 1) {\n\
                        return 1;\n\
                    } else { \n\
                        return f(x - 1) + f(x - 2);\n\
                    }\n\
                }\n\
                void y() { return; }";
    match parse_Program(input) {
        Ok(program) => match semantic_analysis::check(&program) {
            Ok(_) => println!("Ok"),
            Err(err) => println!("{}", err),
        },
        Err(err) => print_error(err, input),
    }
}
