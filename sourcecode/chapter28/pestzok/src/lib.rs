
pub mod ast;
pub mod compiler;
pub mod parser;

// pub use crate::ast;
// pub use crate::compiler::interpreter::Interpreter;
pub use crate::compiler::jit::Jit;
// pub use crate::compiler::vm::{self, vm::VM};

pub type Result<T> = anyhow::Result<T>;

// ANCHOR: compile_trait
pub trait Compile {
    type Output;

    fn from_ast(ast: ast::File) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        println!("Compiling the source: {}", source);
        let ast = ast::generate_ast(source).unwrap();
        println!("{:?}", ast);
        Self::from_ast(ast)
    }
}