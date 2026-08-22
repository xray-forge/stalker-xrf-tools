use std::path::Path;

use xrf_error::XrfResult;
use xrf_ltx::Ltx;

use crate::pack::archive_pack_config::{ArchivePackConfig, ArchivePackDirectory};

/// Section holding the extension patterns that keep a file out.
const SECTION_OPTIONS: &str = "options";

/// Section listing files by name rather than by directory.
const SECTION_INCLUDE_FILES: &str = "include_files";

/// Section names keep the engine's `folders` spelling: they are the xrCompress dialect, not ours to rename.
const SECTION_INCLUDE_DIRECTORIES: &str = "include_folders";
const SECTION_EXCLUDE_DIRECTORIES: &str = "exclude_folders";

/// Section copied into the archive verbatim, which is what tells the engine where to mount it.
const SECTION_HEADER: &str = "header";

impl ArchivePackConfig {
  /// Write the selection rules back out as an xrCompress configuration.
  ///
  /// The inverse of [`ArchivePackConfig::with_ltx`], covering the same sections and no others: source,
  /// destination, name, mode, and volume size stay with the caller, because a configuration file never
  /// carried them. A file written here reads back through `with_ltx` unchanged.
  pub fn write_ltx_to_path<P: AsRef<Path>>(&self, path: P) -> XrfResult {
    self.to_ltx().write_to_path(path)
  }

  pub fn to_ltx(&self) -> Ltx {
    let mut ltx: Ltx = Ltx::new();

    if !self.exclude_extensions.is_empty() {
      Self::set_entry(
        &mut ltx,
        SECTION_OPTIONS,
        "exclude_exts",
        &self.exclude_extensions.join(","),
      );
    }

    // Listed files carry no value, matching how xrCompress reads the section as bare names.
    for name in &self.include_files {
      Self::set_entry(&mut ltx, SECTION_INCLUDE_FILES, name, "");
    }

    Self::write_directories(&mut ltx, SECTION_INCLUDE_DIRECTORIES, &self.include_directories);
    Self::write_directories(&mut ltx, SECTION_EXCLUDE_DIRECTORIES, &self.exclude_directories);

    if let Some(header) = &self.header {
      for (key, value) in Self::header_entries(header) {
        Self::set_entry(&mut ltx, SECTION_HEADER, &key, &value);
      }
    }

    ltx
  }

  fn write_directories(ltx: &mut Ltx, section_name: &str, directories: &[ArchivePackDirectory]) {
    for directory in directories {
      // An empty path names the packed root, which the dialect spells `.\`.
      let path: &str = if directory.path.is_empty() {
        ".\\"
      } else {
        &directory.path
      };

      Self::set_entry(
        ltx,
        section_name,
        path,
        if directory.is_recursive { "true" } else { "false" },
      );
    }
  }

  /// Set one key in a section, creating the section on first use.
  ///
  /// Written per entry rather than through a section setter, which borrows for its whole lifetime and so
  /// cannot be driven from a loop.
  fn set_entry(ltx: &mut Ltx, section_name: &str, key: &str, value: &str) {
    ltx
      .entry(section_name.into())
      .or_insert_with(Default::default)
      .insert(key, value);
  }

  /// Split the stored header text back into the pairs it was built from.
  ///
  /// The header is kept as text because the archive stores it verbatim; this reads it only well enough
  /// to round trip through a configuration file.
  fn header_entries(header: &str) -> Vec<(String, String)> {
    header
      .lines()
      .filter_map(|line| line.split_once('='))
      .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
      .collect()
  }
}
