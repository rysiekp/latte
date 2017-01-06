extern crate lalrpop_util;

pub mod ast;
mod ast_printer;
mod semantic_analysis;
mod parser;
mod code_generation;

use std::fs::File;

fn main() {
    let input = "\
                /* asdasdasdasd\n\
                asdasdasdasd */\n\
                // asdasdasdasd\n\
                int f(int x, bool y) {\n\
                    int z = x + 1;\n\
                    if (y) {\n\
                        z = 0;\n\
                    }\n\
                    \n
                    return z;\n
                }\n\
                bool g(int x, bool y) {\n
                    bool z = true;\n
                    if (y) {\n
                        z = false;\n
                    }\n
                    \n
                    return z;\n
                }\n\
                int main() {\n
                    1 + 1;\n
                    return 0;\n
                }";
    let filename = "test.bc";
    let mut output = File::create(filename).unwrap();
    match parser::parse(String::from(input)) {
        Some(program) => match semantic_analysis::check(&program) {
            Ok(_) => code_generation::run(&mut output, &program),
            Err(err) => println!("{}", err),
        },
        None => (),
    }
}
