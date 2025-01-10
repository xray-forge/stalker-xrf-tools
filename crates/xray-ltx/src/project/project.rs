use crate::file::configuration::constants::{
  LTX_EXTENSION, LTX_SCHEME_EXTENSION, LTX_SCHEME_LTX_FILENAME,
};
use crate::file::include::LtxIncludeConvertor;
use crate::file::types::LtxSectionSchemes;
use crate::project::project_options::LtxProjectOptions;
use crate::scheme::parser::LtxSchemeParser;
use crate::{Ltx, LtxError, LtxResult};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

/// Handler of LTX configs root.
/// Iteration and filtering of de-duplicated ltx files.
/// Parsing of validation schema and making sure LTX files are valid.
#[derive(Debug)]
pub struct LtxProject {
  /// Root path of the project.
  pub root: PathBuf,
  /// List of entry LTX files in the project, entry points that are not included in any file.
  pub ltx_file_entries: Vec<DirEntry>,
  /// List of all LTX files in the project.
  pub ltx_files: Vec<DirEntry>,
  /// List of all LTX scheme files in the project.
  pub ltx_scheme_files: Vec<DirEntry>,
  /// List of all LTX scheme files in the project.
  pub ltx_scheme_file_entries: Vec<DirEntry>,
  /// Map of section schemes declared in the project.
  pub ltx_scheme_declarations: LtxSectionSchemes,
}

impl LtxProject {
  /// Initialize project on provided root.
  pub fn open_at_path_opt(root: &Path, options: LtxProjectOptions) -> LtxResult<Self> {
    let mut ltx_files: Vec<DirEntry> = Vec::new();
    let mut ltx_scheme_files: Vec<DirEntry> = Vec::new();
    let mut included: Vec<PathBuf> = Vec::new();

    // Filter all the entries that are not accessed by other files and represent entry points.
    for entry in WalkDir::new(root) {
      let entry: DirEntry = entry.map_err(|error| LtxError::Io(error.into_io_error().unwrap()))?;
      let entry_path: &Path = entry.path();

      if let Some(extension) = entry_path.extension() {
        if extension == LTX_EXTENSION {
          let parent: &Path = match entry_path.parent() {
            Some(parent) => parent,
            None => {
              return Err(LtxError::new_convert_error(
                "Failed to parse parent directory of ltx file.",
              ))
            }
          };

          for include in &Ltx::read_included_from_file(entry_path)? {
            let mut included_path: PathBuf = PathBuf::from(parent);

            included_path.push(LtxIncludeConvertor::statement_to_path(include));

            included.push(included_path);
          }

          if options.is_with_schemes_check && Self::is_ltx_scheme_path(entry_path) {
            ltx_scheme_files.push(entry.clone())
          }

          ltx_files.push(entry);
        }
      }
    }

    // Filter our entries not included in other files.
    let ltx_file_entries: Vec<DirEntry> = ltx_files
      .iter()
      .filter_map(|it| {
        if included.contains(&PathBuf::from(it.path())) {
          None
        } else {
          Some(it.clone())
        }
      })
      .collect();

    // Filter our entries not included in other files.
    let ltx_scheme_file_entries: Vec<DirEntry> = if options.is_with_schemes_check {
      ltx_scheme_files
        .iter()
        .filter_map(|it| {
          if included.contains(&PathBuf::from(it.path())) {
            None
          } else {
            Some(it.clone())
          }
        })
        .collect()
    } else {
      Default::default()
    };

    Ok(Self {
      root: PathBuf::from(root),
      ltx_files,
      ltx_file_entries,
      ltx_scheme_declarations: if options.is_with_schemes_check {
        LtxSchemeParser::parse_from_files(&ltx_scheme_file_entries)?
      } else {
        Default::default()
      },
      ltx_scheme_file_entries,
      ltx_scheme_files,
    })
  }

  /// Initialize project on provided root with default options.
  pub fn open_at_path(root: &Path) -> LtxResult<Self> {
    Self::open_at_path_opt(root, Default::default())
  }
}

impl LtxProject {
  /// Check if provided LTX file is scheme definition file.
  pub fn is_ltx_scheme_path(path: &Path) -> bool {
    path
      .file_name()
      .and_then(|name| name.to_str())
      .map_or(false, |name| {
        name == LTX_SCHEME_LTX_FILENAME || name.ends_with(LTX_SCHEME_EXTENSION)
      })
  }

  pub fn get_system_ltx(&self) -> LtxResult<Ltx> {
    Ltx::read_from_file_full(self.root.join("system.ltx"))
  }
}
