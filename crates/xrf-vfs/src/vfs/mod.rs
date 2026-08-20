//! The virtual file system: ordered mounts searched first-hit-wins, and the scope that narrows them.

mod directory_listing;
#[cfg(test)]
mod tests;
mod xray_scope;
mod xray_vfs;

pub use directory_listing::XrayDirectoryListing;
pub use xray_scope::XrayScope;
pub use xray_vfs::XrayVfs;
