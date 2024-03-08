use crate::file::configuration::constants::LTX_EXTENSION;
use crate::file::error::LtxConvertError;
use crate::{Ltx, LtxError, LtxFormatOptions};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

/// Handler of LTX configs root.
/// Iteration and filtering of de-duplicated ini files.
/// Parsing of validation schema and making sure LTX files are valid.
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

        for include in &Ltx::read_included_from_file(entry_path)? {
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

      let ltx: Ltx = Ltx::load_from_file_full(entry.path())?;
      let mut destination: PathBuf = PathBuf::from("target/assets");

      destination.push(entry.file_name());

      ltx.write_to_file(destination)?
    }

    println!();
    println!("Verified {} ltx files", self.ltx_entries.len());

    Ok(())
  }

  /// Format all LTX entries in current project.
  pub fn format_all_files_opt(&self, options: LtxFormatOptions) -> Result<bool, LtxError> {
    let mut count: usize = 0;

    if !options.is_silent {
      println!("Formatting path: {:?}", self.root);
    }

    for entry in &self.ltx_files {
      if Ltx::format_file(entry.path(), true)? {
        count += 1;

        if !options.is_silent {
          println!("Formatted: {:?}", entry.path());
        }
      }
    }

    if !options.is_silent {
      println!("Formatted {count}/{} ltx files", self.ltx_entries.len());
    }

    Ok(count > 0)
  }

  /// Check format of all LTX entries in current project.
  pub fn check_all_files_opt(&self, options: LtxFormatOptions) -> Result<bool, LtxError> {
    let mut count: usize = 0;

    if !options.is_silent {
      println!("Checking path: {:?}", self.root);
    }

    for entry in &self.ltx_files {
      if Ltx::format_file(entry.path(), false)? {
        count += 1;

        if !options.is_silent {
          println!("Not formatted: {:?}", entry.path());
        }
      }
    }

    if !options.is_silent {
      if count > 0 {
        println!(
          "Format issues with {count}/{} ltx files",
          self.ltx_entries.len()
        );
      } else {
        println!("All {} ltx files are formatted", self.ltx_entries.len());
      }
    }

    Ok(count > 0)
  }

  /// Format all LTX entries in current project.
  pub fn format_all_files(&self) -> Result<bool, LtxError> {
    self.format_all_files_opt(LtxFormatOptions::default())
  }

  /// Format all LTX entries in current project.
  pub fn check_all_files(&self) -> Result<bool, LtxError> {
    self.check_all_files_opt(LtxFormatOptions::default())
  }

  /// Format single LTX file by provided path
  pub fn verify_file(path: &Path) -> Result<(), LtxError> {
    let ltx: Ltx = Ltx::read_from_file(path)?;

    ltx
      .into_included()?
      .into_inherited()?
      .write_to_file("target/assets/test.ltx")?;

    Ok(())
  }
}
