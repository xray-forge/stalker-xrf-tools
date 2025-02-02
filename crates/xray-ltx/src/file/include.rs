use crate::Ltx;
use std::io;
use std::path::{Path, PathBuf, MAIN_SEPARATOR_STR};
use xray_error::{XRayError, XRayResult};

/// Converter object to process and inject all child #include statements.
#[derive(Default)]
pub struct LtxIncludeConvertor {}

impl LtxIncludeConvertor {
  fn new() -> Self {
    Self {}
  }

  /// Cast LTX file to fully parsed with include sections.
  pub fn convert(ltx: Ltx) -> XRayResult<Ltx> {
    Self::new().convert_ltx(ltx)
  }

  /// Transform ltx statement to cross-platform path.
  pub fn statement_to_path(statement: &str) -> PathBuf {
    PathBuf::from(statement.replace('\\', MAIN_SEPARATOR_STR))
  }
}

impl LtxIncludeConvertor {
  /// Convert ltx file with inclusion of nested files.
  fn convert_ltx(&self, ltx: Ltx) -> XRayResult<Ltx> {
    if ltx.directory.is_none() {
      return Err(XRayError::new_convert_error(
        "Failed to parse ltx file, parent directory is not specified",
      ));
    }

    // Nothing to parse - no include statements.
    if ltx.includes.is_empty() {
      return Ok(ltx);
    }

    let mut result: Ltx = Ltx {
      path: ltx.path,
      directory: ltx.directory,
      includes: Default::default(),
      sections: Default::default(),
    };

    for included in &ltx.includes {
      let mut included_path: PathBuf = result.directory.as_ref().unwrap().clone();

      included_path.push(Self::statement_to_path(included));

      self.include_children(&mut result, &included_path)?;
    }

    for (key, value) in ltx.sections {
      match result.section_mut(&key) {
        None => {
          result.sections.insert(key, value);
        }
        Some(existing) => {
          // Handle cases with root declarations.
          if key.is_empty() {
            existing.merge(value);
          } else {
            return Err(XRayError::new_convert_error(format!(
              "Failed to equipment ltx file, duplicate section {key} found",
            )));
          }
        }
      }
    }

    Ok(result)
  }

  /// Include children ltx into provided ltx.
  fn include_children<P: AsRef<Path>>(&self, into: &mut Ltx, path: &P) -> XRayResult {
    let ltx: Ltx = match self.parse_nested_file(path) {
      Ok(value) => match value {
        Some(ltx) => ltx,
        None => return Ok(()),
      },
      Err(error) => {
        return Err(XRayError::new_convert_error(format!(
          "Failed to parse ltx file, nested file {} in {} error: {error}",
          path.as_ref().display(),
          into.path.as_ref().unwrap().display(),
        )))
      }
    };

    for (key, value) in ltx.into_included()?.sections {
      match into.section_mut(&key) {
        None => {
          into.sections.insert(key, value);
        }
        Some(existing) => {
          // Handle cases with root declarations.
          if key.is_empty() {
            existing.merge(value);
          } else {
            return Err(XRayError::new_convert_error(format!(
              "Failed to include ltx file '{}' in {}, duplicate section '{}' found",
              path.as_ref().display(),
              into.path.as_ref().unwrap().display(),
              key
            )));
          }
        }
      }
    }

    Ok(())
  }

  /// Open nested file for importing in current context.
  /// Skips '.ts' variant of configuration file as None.
  fn parse_nested_file<P: AsRef<Path>>(&self, path: &P) -> XRayResult<Option<Ltx>> {
    match Ltx::read_from_path(path.as_ref()) {
      Ok(ltx) => Ok(Some(ltx)),
      Err(error) => match error {
        XRayError::Io {
          ref kind,
          message: _,
        } => {
          if *kind == io::ErrorKind::NotFound {
            if self.is_raw_ts_variant_existing(path) {
              Ok(None)
            } else {
              Err(error)
            }
          } else {
            Err(error)
          }
        }
        _ => Err(error),
      },
    }
  }

  /// Check if similar TS counterpart exists for provided ltx path.
  fn is_raw_ts_variant_existing<P: AsRef<Path>>(&self, path: &P) -> bool {
    if path
      .as_ref()
      .extension()
      .is_some_and(|extension| extension == "ltx")
    {
      path.as_ref().with_extension("ts").exists()
    } else {
      false
    }
  }
}
