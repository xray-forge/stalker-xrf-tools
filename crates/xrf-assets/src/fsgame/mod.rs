pub(crate) mod declaration;
pub(crate) mod file;
#[cfg(test)]
mod tests;

pub use declaration::{FS_ROOT_ALIAS, FsgameDeclaration};
pub use file::{FSGAME_FILE_NAME, FsgameFile};
