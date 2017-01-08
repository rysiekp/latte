use ast::Program;
use optimizer::constant_folding::Fold;

mod constant_folding;

pub fn optimize(program: Program) -> Program {
    program.fold()
}