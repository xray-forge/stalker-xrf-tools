use crate::file::constants::LTX_EXTENSION;
use crate::file::error::LtxConvertError;
use crate::{EscapePolicy, Ltx, LtxError, ParseOptions, WriteOptions};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

/// Handler of LTX configs root.
/// Iteration and filtering of de-duplicated ini files.
#[derive(Debug)]
pub struct LtxProject {
  /// Root path of the project.
  pub root: PathBuf,
  /// List of all LTX files in the project.
  pub ltx_files: Vec<DirEntry>,
  /// List of entry LTX files in the project, entry points that are not included in any file.
  pub ltx_entries: Vec<DirEntry>,
}

impl LtxProject {
  /// Initialize project on provided root.
  pub fn open_at_path(root: &Path) -> Result<LtxProject, LtxError> {
    let mut files: Vec<DirEntry> = Vec::new();
    let mut included: Vec<PathBuf> = Vec::new();

    // Filter all the entries that are not accessed by other files and represent entry points.
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

        for include in &Ltx::read_includes_from_file_opt(
          entry_path,
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

    // Filter our entries not included in other files.
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
      ltx_files: files,
      ltx_entries: entries,
    })
  }
}

impl LtxProject {
  /// Verify all the entries in current ltx project.
  /// Make sure that:
  /// - All included files exist or `.ts` counterpart is declared
  /// - All the inherited sections are valid and declared before inherit attempt
  pub fn verify_entries(&self) -> Result<(), LtxError> {
    for entry in &self.ltx_entries {
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
    println!("Verified {} ltx files", self.ltx_entries.len());

    Ok(())
  }

  /// Format all LTX entries in current project.
  pub fn format_all_files(&self) -> Result<(), LtxError> {
    for entry in &self.ltx_files {
      println!("Format: {:?}", entry.path());
      Self::format_file(entry.path())?;
    }

    println!();
    println!("Formatted {} ltx files", self.ltx_entries.len());

    Ok(())
  }

  /// Format single LTX file by provided path
  pub fn format_file(path: &Path) -> Result<(), LtxError> {
    let formatted: String = Ltx::format_from_file_opt(
      path,
      ParseOptions {
        enabled_escape: false,
        enabled_quote: false,
      },
      WriteOptions {
        escape_policy: EscapePolicy::Nothing,
        ..Default::default()
      },
    )?;

    fs::write(path, formatted).map_err(LtxError::Io)
  }

  /// Format single LTX file by provided path
  pub fn verify_file(path: &Path) -> Result<(), LtxError> {
    let ltx: Ltx = Ltx::load_from_file_opt(
      path,
      ParseOptions {
        enabled_escape: false,
        enabled_quote: false,
      },
    )?;

    let ltx: Ltx = ltx
      .into_included_opt(ParseOptions {
        enabled_escape: false,
        enabled_quote: false,
      })
      .unwrap()
      .into_inherited()
      .unwrap();

    ltx.write_to_file_opt(
      "target/assets/test.ltx",
      WriteOptions {
        escape_policy: EscapePolicy::Nothing,
        ..Default::default()
      },
    )?;

    Ok(())
  }
}
