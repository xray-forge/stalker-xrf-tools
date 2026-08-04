//! Immutable, deterministic reports produced by XRF commands.

mod check_report;
mod finding;
mod identifier;
mod report;
mod status;

pub use check_report::CheckReport;
pub use finding::Finding;
pub use identifier::{CheckId, IdentifierError, RuleId};
pub use report::Report;
pub use status::Status;
