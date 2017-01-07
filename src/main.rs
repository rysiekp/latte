extern crate lalrpop_util;

pub mod ast;
mod ast_printer;
mod semantic_analysis;
mod parser;
mod code_generation;

use std::fs::File;
use std::process::Command;

fn use_llvm(ll_path: String, bc_path: String) {
    let ref tmp_bc_path = "TMP.bc";
    Command::new("llvm-as")
        .arg(ll_path)
        .arg("-o")
        .arg(tmp_bc_path)
        .status()
        .expect("Couldn't generate bc file");

    Command::new("llvm-link")
        .arg("-o")
        .arg(bc_path)
        .arg(tmp_bc_path)
        .arg("lib/runtime.bc")
        .status()
        .expect("Couldn't link with runtime.bc");

    Command::new("rm").arg(tmp_bc_path).status().expect("Unable to remove temporary bc file");
}

fn main() {
    let input = "\
                int fib1(int n) {\n\
                    if (n < 2) {\n\
                        return n;\n\
                    }\n\
                    return fib1(n - 1) + fib1(n - 2);\n\
                }\n\
                int fib2(int n) {\n\
                    if (n < 2) {\n\
                        return n;\n\
                    }
                    int prev1 = 0, prev2 = 1;\n\
                    int i = 2;\n\
                    int res = 0;\n\
                    while (i <= n) {\n\
                        i++;\n\
                        res = prev1 + prev2;\n\
                        prev1 = prev2;\n\
                        prev2 = res;\n\
                    }\n\
                    return res;

                }\n\
                int main() {\n\
                    int x = 5;\n\
                    printInt(fib1(x));\n\
                    printInt(fib2(x));\n\
                    return 0;\n\
                }";
    let filename = "test.ll";
    let mut output = File::create(filename).unwrap();
    match parser::parse(String::from(input)) {
        Some(program) => match semantic_analysis::check(&program) {
            Ok(_) => {
                code_generation::run(&mut output, &program);
                use_llvm(String::from("test.ll"), String::from("test.bc"));
            },
            Err(err) => println!("{}", err),
        },
        None => (),
    }
}
