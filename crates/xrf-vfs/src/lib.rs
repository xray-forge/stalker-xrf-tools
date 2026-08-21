//! Indexes X-Ray assets and layers their physical sources in a virtual file system.
//!
//! The crate is grouped by the question each part answers:
//!
//! - [`path`] — what an engine identity is, and the only place separators and case are decided.
//! - [`asset`] — what a resolved asset is, plus the per-kind rules that turn a reference into one.
//! - [`source`] — the mountable surface, and the two sources the engine itself has.
//! - [`mount`] — composing sources into a searchable order, and planning one from a path.
//! - [`vfs`] — resolving and reading through that order.
//! - [`fsgame`] — the declaration file an installation describes its own layout with.
//! - `archive` — reading `.db` volume sets, which the archive source above is built on.
//!
//! Everything a consumer needs is re-exported here, so `use xrf_vfs::XrayVfs` stays the import regardless of how the inside
//! is arranged.

pub mod asset;
pub mod fsgame;
pub mod mount;
pub mod path;
pub mod source;
pub mod vfs;

pub(crate) mod archive;
#[cfg(feature = "typescript-bindings")]
mod typescript_bindings;

#[cfg(feature = "typescript-bindings")]
pub use crate::typescript_bindings::typescript_bindings;

pub use asset::{
  SHADER_LIBRARY_LOGICAL_PATH, XrayAsset, XrayAssetContainer, XrayAssetRules, XrayAssetType, sound_reference_name,
};
pub use fsgame::{FS_ROOT_ALIAS, FSGAME_FILE_NAME, FsgameDeclaration, FsgameFile};
pub use mount::{
  XrayMount, XrayMountId, XrayMountMode, XrayMountPlan, XrayPlannedMount, XraySkippedMount, implied_asset_root,
  mount_plan, open_plan, open_vfs,
};
pub use path::{XrayPath, XrayPathCollision, is_component_prefix, normalize_logical};
pub use source::{ArchiveAssetSource, XrayAssetSource, XrayMountKind};
pub use vfs::{XrayDirectoryListing, XrayLookupScope, XrayVfs};

pub use crate::archive::{
  ArchiveDescriptor, ArchiveFileDescriptor, ArchiveProject, ArchiveProjectReadPolicy, ProjectReadResult,
};

// Format internals the archive tooling in `xrf-archive` needs to build and extract volumes. Reading a set is this crate's
// job; the two write directions are not, and they cannot be expressed without these.
pub use crate::archive::{CHUNK_ID_COMPRESSED_MASK, write_descriptor_contents};
