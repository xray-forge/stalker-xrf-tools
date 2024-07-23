use crate::project::project::TranslationProject;
use crate::project::project_constants::ALLOWED_PROJECT_READ_EXTENSIONS;
use std::fs::File;
use std::io::Read;

use crate::types::TranslationJson;
use crate::TranslationError;
use std::path::Path;

impl TranslationProject {
  pub fn read_translation_json_by_path(path: &Path) -> Result<TranslationJson, TranslationError> {
    let mut data: Vec<u8> = Vec::new();

    File::open(path)?.read_to_end(&mut data)?;

    Ok(serde_json::from_slice(&data).unwrap())
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
}
