use std::path::{Path, PathBuf};

use serde::Serialize;

/// One entry of a volume's name table: where its payload sits and how to verify it.
///
/// Equal `size_real` and `size_compressed` is how the format says "stored uncompressed".
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveFileDescriptor {
  /// CRC32 of the unpacked payload, recorded by the packer and verified on decompression.
  pub crc: u32,
  /// The volume file holding the payload.
  pub source: PathBuf,
  /// Root the entry unpacks under, from its volume's header.
  pub destination: PathBuf,
  /// Lower-cased extension derived from [`Self::name`], empty when the name has none.
  pub extension: String,
  /// Entry name as authored, which the engine registers verbatim.
  pub name: String,
  /// Byte offset of the payload inside [`Self::source`].
  pub offset: u32,
  /// Payload bytes as stored in the volume.
  pub size_compressed: u32,
  /// Payload bytes once unpacked.
  pub size_real: u32,
}

impl ArchiveFileDescriptor {
  /// Creates a descriptor from name-table fields, deriving the extension; volume paths attach separately through
  /// [`Self::with_archive_paths`], because the table does not record them.
  pub fn new(crc: u32, name: String, offset: u32, size_compressed: u32, size_real: u32) -> Self {
    Self {
      crc,
      source: PathBuf::new(),
      destination: PathBuf::new(),
      extension: Self::extension_from_path(&name),
      name,
      offset,
      size_compressed,
      size_real,
    }
  }

  /// Attaches the volume the entry was read from and the root it unpacks under.
  pub fn with_archive_paths(mut self, source: &Path, destination: &Path) -> Self {
    self.source = source.into();
    self.destination = destination.into();
    self
  }

  fn extension_from_path(path: &str) -> String {
    let name: &str = path.rsplit(['\\', '/']).next().unwrap_or(path);

    name
      .rsplit_once('.')
      .map_or("", |(_, extension)| extension)
      .to_ascii_lowercase()
  }
}

#[cfg(test)]
mod tests {
  use std::path::Path;

  use super::ArchiveFileDescriptor;

  #[test]
  fn descriptor_includes_archive_paths_and_a_normalized_extension() {
    for (name, expected) in [
      ("configs\\system.LTX", "ltx"),
      ("scripts/actor.script", "script"),
      ("readme", ""),
    ] {
      let descriptor: ArchiveFileDescriptor = ArchiveFileDescriptor::new(0, name.into(), 0, 0, 0)
        .with_archive_paths(Path::new("database.db0"), Path::new("gamedata"));

      assert_eq!(descriptor.source, Path::new("database.db0"));
      assert_eq!(descriptor.destination, Path::new("gamedata"));
      assert_eq!(descriptor.extension, expected);
    }
  }
}
