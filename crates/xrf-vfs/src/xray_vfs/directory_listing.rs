use crate::XrayAssetLocation;

/// What sits directly inside one logical directory.
///
/// Directories are names rather than located assets, because a directory is not an asset the VFS can resolve — it is
/// inferred from the paths of the entries below it.
#[derive(Clone, Debug, Default)]
pub struct XrayDirectoryListing {
  /// Names of the directories directly inside, sorted, without their parent path.
  pub directories: Vec<String>,
  /// Winning entries sitting directly inside, sorted by logical path.
  pub files: Vec<XrayAssetLocation>,
}

impl XrayDirectoryListing {
  /// Whether the directory holds nothing the current scope can see.
  pub fn is_empty(&self) -> bool {
    self.directories.is_empty() && self.files.is_empty()
  }
}
