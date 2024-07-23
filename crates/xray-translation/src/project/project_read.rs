use crate::project::project::TranslationProject;
use crate::project::project_constants::ALLOWED_PROJECT_READ_EXTENSIONS;
use std::fs::File;
use std::io::Read;

use crate::types::TranslationJson;
use crate::{TranslationError, TranslationLanguage};
use std::path::Path;

impl TranslationProject {
  pub fn read_translation_json_by_path(path: &Path) -> Result<TranslationJson, TranslationError> {
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
}
