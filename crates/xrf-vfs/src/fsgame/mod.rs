//! `fsgame.ltx`: the file an installation describes its own directory layout with, parsed and resolved to host paths.

mod declaration;
mod file;
#[cfg(test)]
mod tests;

pub use declaration::{FS_ROOT_ALIAS, FsgameDeclaration};
pub use file::{FSGAME_FILE_NAME, FsgameFile};
