use crate::project::project::TranslationProject;
use std::fs;
use std::fs::File;
use std::io::ErrorKind::AlreadyExists;

use crate::types::{TranslationJson, TranslationVariant};
use crate::{ProjectBuildOptions, TranslationLanguage};
use serde_json::{Map, Value};
use std::path::{Path, PathBuf};
use xray_error::XRayResult;

impl TranslationProject {
  pub fn prepare_target_xml_translation_file(
    path: &Path,
    destination: &Path,
    language: &TranslationLanguage,
    options: &ProjectBuildOptions,
  ) -> XRayResult<File> {
    let target: PathBuf = destination
      .join(language.to_string())
      .join(path.file_name().unwrap())
      .with_extension("xml");

    if options.is_verbose_logging_enabled() {
      println!("Writing file ({}) {}", language, target.display());
    }

    match fs::create_dir_all(target.parent().unwrap()) {
      Ok(_) => {}
      Err(error) if error.kind() == AlreadyExists => {}
      Err(error) => return Err(error.into()),
    }

    Ok(
      File::options()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(target)
        .expect("File can not be opened for writing"),
    )
  }

  pub fn prepare_target_json_translation_file(path: &Path) -> XRayResult<File> {
    Ok(
      File::options()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("File can not be opened for writing"),
    )
  }

  pub fn transform_translation_into_value(translation: &TranslationJson) -> Value {
    let mut root: Map<String, Value> = Map::new();

    for (key, value) in translation {
      let mut translations: Map<String, Value> = Map::new();

      for (language, translation) in value {
        translations.insert(
          language.clone(),
          match translation {
            None => Value::Null,
            Some(value) => match value {
              TranslationVariant::String(str) => Value::String(str.clone()),
              TranslationVariant::MultiString(vector) => Value::Array(
                vector
                  .iter()
                  .map(|string| Value::String(string.clone()))
                  .collect(),
              ),
            },
          },
        );
      }

      root.insert(key.clone(), Value::Object(translations));
    }

    Value::Object(root)
  }
}
