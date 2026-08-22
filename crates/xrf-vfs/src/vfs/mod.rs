//! The virtual file system: ordered mounts searched first-hit-wins, and the lookup scope that narrows a search to some of
//! them.

mod directory_listing;
#[cfg(test)]
mod tests;
mod xray_lookup_scope;
mod xray_scoped_vfs;
mod xray_vfs;

pub use directory_listing::XrayDirectoryListing;
pub use xray_lookup_scope::XrayLookupScope;
pub use xray_scoped_vfs::XrayScopedVfs;
pub use xray_vfs::XrayVfs;
