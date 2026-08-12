use std::path::{Path, PathBuf};

use serde::Serialize;

#[cfg_attr(
  feature = "typescript-bindings",
  derive(ts_rs::TS),
  ts(export, export_to = "xray-archive.ts")
)]
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveFileDescriptor {
  pub crc: u32,
  pub source: PathBuf,
  pub destination: PathBuf,
  pub extension: String,
  pub name: String,
  pub offset: u32,
  pub size_compressed: u32,
  pub size_real: u32,
}

impl ArchiveFileDescriptor {
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
