use crate::file::constants::LTX_EXTENSION;
use crate::file::error::LtxConvertError;
use crate::{EscapePolicy, Ltx, LtxError, ParseOptions, WriteOptions};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

/// Handler of LTX configs root.
/// Iteration and filtering of de-duplicated ini files.
#[derive(Debug)]
pub struct LtxProject {
  pub root: PathBuf,
  pub files: Vec<DirEntry>,
  pub entries: Vec<DirEntry>,
}

impl LtxProject {
  /// Initialize project on provided root.
  pub fn on_root(root: &Path) -> Result<LtxProject, LtxError> {
    let mut files: Vec<DirEntry> = Vec::new();
    let mut included: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(root) {
      let entry: DirEntry = match entry {
        Ok(entry) => entry,
        Err(error) => return Err(LtxError::Io(error.into_io_error().unwrap())),
      };

      let entry_path: &Path = entry.path();

      if entry_path
        .extension()
        .is_some_and(|extension| extension == LTX_EXTENSION)
      {
        let parent: &Path = match entry_path.parent() {
          Some(parent) => parent,
          None => {
            return Err(LtxConvertError::new_ltx_error(
              "Failed to parse parent directory of ltx file.",
            ))
          }
        };

        for include in &Ltx::load_includes_from_file_opt(
          entry.path(),
          ParseOptions {
            enabled_quote: false,
            enabled_escape: false,
          },
        )? {
          let mut included_path: PathBuf = PathBuf::from(parent);

          included_path.push(include);

          included.push(included_path);
        }

        files.push(entry);
      }
    }

    let entries: Vec<DirEntry> = files
      .iter()
      .filter_map(|it| {
        if included.contains(&PathBuf::from(it.path())) {
          None
        } else {
          Some(it.clone())
        }
      })
      .collect();

    Ok(LtxProject {
      root: PathBuf::from(root),
      files,
      entries,
    })
  }

  pub fn verify_entries(&self) -> Result<(), LtxError> {
    for entry in &self.entries {
      println!("Verify: {:?}", entry.path());

      let ltx: Ltx = Ltx::load_from_file_full_inherited_opt(
        entry.path(),
        ParseOptions {
          enabled_escape: false,
          enabled_quote: false,
        },
      )?;

      let mut destination: PathBuf = PathBuf::from("target/assets");

      destination.push(entry.file_name());

      ltx
        .write_to_file_opt(
          destination,
          WriteOptions {
            escape_policy: EscapePolicy::Nothing,
            ..Default::default()
          },
        )
        .unwrap();
    }

    println!();
    println!("Verified {} ltx files", self.entries.len());

    Ok(())
  }
}
