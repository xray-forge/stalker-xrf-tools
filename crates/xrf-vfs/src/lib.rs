//! Indexes X-Ray assets and layers their physical sources in a virtual file system.
//!
//! [`XrayVfs::open`] is the front door: a mode and a path become something you can resolve and read against. The crate
//! is grouped by the question each part answers:
//!
//! - [`path`] — what an engine identity is, and the only place separators and case are decided.
//! - [`asset`] — what a resolved asset is, plus the per-kind rules that turn a reference into one.
//! - [`source`] — the mountable surface, and the two sources the engine itself has.
//! - [`mount`] — composing sources into a searchable order, and planning one from a path.
//! - [`vfs`] — resolving and reading through that order.
//! - [`fsgame`] — the declaration file an installation describes its own layout with.
//!
//! The `.db` volume format the archive source reads lives below this crate, in `xrf-volume`.
//!
//! Everything a consumer needs is re-exported here, so `use xrf_vfs::XrayVfs` stays the import regardless of how the inside
//! is arranged. The root exports only types; helpers and constants hang off the type that owns their concept.

pub mod asset;
pub mod fsgame;
pub mod mount;
pub mod path;
pub mod source;
pub mod vfs;

pub use asset::{XrayAsset, XrayAssetContainer, XrayAssetRules, XrayAssetType};
pub use fsgame::{FsgameDeclaration, FsgameFile};
pub use mount::{XrayMount, XrayMountId, XrayMountMode, XrayMountPlan, XrayPlannedMount, XraySkippedMount};
pub use path::{XrayLogicalPath, XrayPathCollision};
pub use source::{XrayArchiveSource, XrayAssetSource, XraySourceKind};
pub use vfs::{XrayDirectoryListing, XrayLookupScope, XrayScopedVfs, XrayVfs};
