use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::de::{Error as _, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use xrf_error::{XrfError, XrfResult};

use crate::project::translation_project::TranslationProject;
use crate::types::{TranslationEntry, TranslationJson};

impl TranslationProject {
  pub(crate) fn read_translation_json_by_path(path: &Path) -> XrfResult<TranslationJson> {
    let mut data: Vec<u8> = Vec::new();

    File::open(path)?.read_to_end(&mut data)?;

    serde_json::from_slice::<UniqueTranslationJson>(&data)
      .map(|json| json.0)
      .map_err(|error| {
        XrfError::new_parsing_error(format!(
          "Failed to parse translation JSON '{}': {error}",
          path.display()
        ))
      })
  }
}

struct UniqueTranslationJson(TranslationJson);

impl<'de> Deserialize<'de> for UniqueTranslationJson {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_map(UniqueTranslationJsonVisitor)
  }
}

struct UniqueTranslationJsonVisitor;

impl<'de> Visitor<'de> for UniqueTranslationJsonVisitor {
  type Value = UniqueTranslationJson;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str("a translation object with unique IDs")
  }

  fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
  where
    A: MapAccess<'de>,
  {
    let mut translations: TranslationJson = Default::default();

    while let Some(id) = map.next_key::<String>()? {
      if translations.contains_key(&id) {
        return Err(A::Error::custom(format!("duplicate translation ID '{id}'")));
      }

      let entry: UniqueTranslationEntry = map.next_value()?;
      translations.insert(id, entry.0);
    }

    Ok(UniqueTranslationJson(translations))
  }
}

struct UniqueTranslationEntry(TranslationEntry);

impl<'de> Deserialize<'de> for UniqueTranslationEntry {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_map(UniqueTranslationEntryVisitor)
  }
}

struct UniqueTranslationEntryVisitor;

impl<'de> Visitor<'de> for UniqueTranslationEntryVisitor {
  type Value = UniqueTranslationEntry;

  fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str("a translation entry with unique language keys")
  }

  fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
  where
    A: MapAccess<'de>,
  {
    let mut entry: TranslationEntry = Default::default();

    while let Some(language) = map.next_key::<String>()? {
      if entry.contains_key(&language) {
        return Err(A::Error::custom(format!("duplicate translation language '{language}'")));
      }

      entry.insert(language, map.next_value()?);
    }

    Ok(UniqueTranslationEntry(entry))
  }
}

#[cfg(test)]
mod tests {
  use std::io::Write;

  use xrf_error::XrfResult;
  use xrf_test_utils::utils::{get_absolute_generated_test_resource_path, overwrite_generated_test_resource_as_file};

  use crate::project::translation_project::TranslationProject;

  #[test]
  fn malformed_json_returns_a_parsing_error() -> XrfResult {
    let relative_path = "translation_project_read/malformed.json";
    let mut file = overwrite_generated_test_resource_as_file(relative_path)?;

    file.write_all(b"{")?;
    drop(file);

    let path = get_absolute_generated_test_resource_path(relative_path);
    let error = TranslationProject::read_translation_json_by_path(&path).unwrap_err();

    assert!(error.to_string().contains("Failed to parse translation JSON"));
    assert!(error.to_string().contains(&path.display().to_string()));

    Ok(())
  }

  #[test]
  fn duplicate_json_ids_return_a_parsing_error() -> XrfResult {
    let relative_path = "translation_project_read/duplicate_ids.json";
    let mut file = overwrite_generated_test_resource_as_file(relative_path)?;

    file.write_all(br#"{"st_test":{"eng":"first"},"st_test":{"eng":"second"}}"#)?;
    drop(file);

    let path = get_absolute_generated_test_resource_path(relative_path);
    let error = TranslationProject::read_translation_json_by_path(&path).unwrap_err();

    assert!(error.to_string().contains("duplicate translation ID 'st_test'"));

    Ok(())
  }

  #[test]
  fn duplicate_json_languages_return_a_parsing_error() -> XrfResult {
    let relative_path = "translation_project_read/duplicate_languages.json";
    let mut file = overwrite_generated_test_resource_as_file(relative_path)?;

    file.write_all(br#"{"st_test":{"eng":"first","eng":"second"}}"#)?;
    drop(file);

    let path = get_absolute_generated_test_resource_path(relative_path);
    let error = TranslationProject::read_translation_json_by_path(&path).unwrap_err();

    assert!(error.to_string().contains("duplicate translation language 'eng'"));

    Ok(())
  }
}
