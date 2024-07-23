use crate::project::project::TranslationProject;
use crate::project::project_constants::ALLOWED_PROJECT_READ_EXTENSIONS;
use std::fs::File;

use crate::types::TranslationJson;
use std::path::Path;

impl TranslationProject {
  pub fn read_translation_json_by_path(path: &Path) -> TranslationJson {
    let file: File = File::open(path).unwrap();

    serde_json::from_reader(file).unwrap()
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
