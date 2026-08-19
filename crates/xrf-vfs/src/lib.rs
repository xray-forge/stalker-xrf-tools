//! Indexes X-Ray assets and layers their physical sources in a virtual file system.

mod directory_asset;
mod directory_asset_index;
mod fsgame;
mod open;
pub mod shader;
pub mod sound;
pub mod texture;
#[cfg(feature = "typescript-bindings")]
mod typescript_bindings;
mod xray_asset;
mod xray_asset_index;
mod xray_asset_location;
mod xray_asset_source;
mod xray_asset_type;
mod xray_directory_source;
mod xray_mount;
mod xray_mount_mode;
mod xray_mount_plan;
pub mod xray_path;
mod xray_root;
mod xray_scope;
mod xray_vfs;

#[cfg(feature = "typescript-bindings")]
pub use crate::typescript_bindings::typescript_bindings;
pub use directory_asset::DirectoryAsset;
pub use directory_asset_index::DirectoryAssetIndex;
pub use fsgame::{FS_ROOT_ALIAS, FSGAME_FILE_NAME, FsgameDeclaration, FsgameFile};
pub use open::open_vfs;
pub use xray_asset::XrayAsset;
pub use xray_asset_index::XrayAssetIndex;
pub use xray_asset_location::{XrayAssetContainer, XrayAssetLocation};
pub use xray_asset_source::{XrayAssetSource, XrayMountKind};
pub use xray_asset_type::XrayAssetType;
pub use xray_directory_source::XrayDirectorySource;
pub use xray_mount::{XrayMount, XrayMountId};
pub use xray_mount_mode::XrayMountMode;
pub use xray_mount_plan::{XrayMountPlan, XrayPlannedMount};
pub use xray_path::XrayPath;
pub use xray_root::{MESHES_DIRECTORY, TEXTURES_DIRECTORY, implied_asset_root, implied_install_root};
pub use xray_scope::{XrayMountSelection, XrayScope};
pub use xray_vfs::XrayVfs;

pub(crate) mod archive;
pub(crate) mod project;
pub(crate) mod types;

pub use crate::archive::archive_descriptor::ArchiveDescriptor;
pub use crate::archive::archive_file_descriptor::ArchiveFileDescriptor;
pub use crate::project::archive_asset_source::ArchiveAssetSource;
pub use crate::project::archive_project::ArchiveProject;
pub use crate::project::archive_project_read_policy::ArchiveProjectReadPolicy;
pub use crate::project::archive_project_read_result::ProjectReadResult;
pub use crate::project::plan_mount::mount_plan;

// Format internals the archive tooling in `xrf-archive` needs to build and extract volumes. Reading a set is this crate's
// job; the two write directions are not, and they cannot be expressed without these.
pub use crate::archive::archive_constants::CHUNK_ID_COMPRESSED_MASK;
pub use crate::archive::archive_file_io::write_descriptor_contents;
