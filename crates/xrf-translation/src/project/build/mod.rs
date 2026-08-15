//! Compiling translation sources into the per-language string tables the game loads.

pub(crate) mod compile;
pub(crate) mod options;
pub(crate) mod result;
pub(crate) mod run;
pub(crate) mod targets;

#[cfg(test)]
mod tests;
