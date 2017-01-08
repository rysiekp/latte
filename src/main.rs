extern crate lalrpop_util;

pub mod ast;
mod ast_printer;
mod semantic_analysis;
mod parser;
mod code_generation;
mod optimizer;
mod utils;

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
    let input = utils::get_input();
    let filename = utils::get_output_filename(".ll");
    let mut output = File::create(filename).unwrap();
    let program = parser::parse(String::from(input));
    semantic_analysis::check_types(&program);
    let program = optimizer::optimize(program);
    println!("{}", program);
    semantic_analysis::check_returns(&program);
    code_generation::run(&mut output, &program);
    use_llvm(utils::get_output_filename(".ll"), utils::get_output_filename(".bc"));
    println!("OK");
}
