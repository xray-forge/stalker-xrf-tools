use crate::project::project::TranslationProject;
use crate::project::project_constants::ALLOWED_PROJECT_READ_EXTENSIONS;
use std::fs::File;
use std::io::Read;

use crate::types::{TranslationJson, TranslationProjectJson};
use crate::{TranslationError, TranslationLanguage, TranslationResult};
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

impl TranslationProject {
  pub fn read_project(dir: &Path) -> TranslationResult<TranslationProjectJson> {
    let mut project_json: TranslationProjectJson = Default::default();

    // Filter all the entries that are not accessed by other files and represent entry points.
    for entry in WalkDir::new(dir) {
      let entry: DirEntry = match entry {
        Ok(entry) => entry,
        Err(error) => return Err(TranslationError::Io(error.into_io_error().unwrap())),
      };

      let entry_path: &Path = entry.path();

      if entry_path.is_file() {
        if let Some(extension) = entry_path.extension() {
          if extension == "json" {
            project_json.insert(
              entry_path.to_str().unwrap().into(),
              Self::read_translation_json_by_path(entry_path)?,
            );
          } else {
            log::warn!("Skip non json translation file {:?}", entry_path);
          }
        }
      }
    }

    // todo: Validate and de-duplicate?

    Ok(project_json)
  }

  pub fn read_translation_json_by_path(path: &Path) -> TranslationResult<TranslationJson> {
    let mut data: Vec<u8> = Vec::new();

    File::open(path)?.read_to_end(&mut data)?;

    Ok(serde_json::from_slice(&data).expect("Expected valid JSON source file with standard format"))
  }

  pub fn can_read_path(&self, path: &Path) -> bool {
    if let Some(extension) = path.extension() {
      for allowed in ALLOWED_PROJECT_READ_EXTENSIONS {
        if (*allowed).eq(extension) {
          return true;
        }
      }
    }

    false
  }

  pub fn get_locale_from_path(path: &Path) -> Option<TranslationLanguage> {
    match path.file_name() {
      Some(name) => {
        if let Some(name) = name.to_str() {
          let parts: Vec<&str> = name.split('.').collect();
          let parts_count: usize = parts.len();

          if parts_count > 2 {
            return match TranslationLanguage::from_str_single(parts.get(parts_count - 2).unwrap()) {
              Ok(locale) => Some(locale),
              Err(_) => None,
            };
          }
        }

        None
      }
      None => None,
    }
  }

  pub fn flatten(translation_project_json: &TranslationProjectJson) -> TranslationJson {
    let mut json: TranslationJson = Default::default();

    for nested in translation_project_json.values() {
      for (key, value) in nested {
        // todo: Duplicates check and error return?

        json.insert(key.clone(), value.clone());
      }
    }

    json
  }
}
