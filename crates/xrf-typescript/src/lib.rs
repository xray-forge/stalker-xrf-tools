pub mod ast;
pub mod parser;
pub mod symbol_resolver;

pub use parser::*;
pub use swc_common;
pub use swc_ecma_ast;
pub use symbol_resolver::*;
