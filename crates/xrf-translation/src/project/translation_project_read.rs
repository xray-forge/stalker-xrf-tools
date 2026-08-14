use std::collections::HashSet;
use std::path::Path;

use walkdir::{DirEntry, WalkDir};
use xrf_error::{XrfError, XrfResult};

use crate::project::translation_project::TranslationProject;
use crate::types::{TranslationJson, TranslationProjectJson};

impl TranslationProject {
  pub fn read_project<P: AsRef<Path>>(dir: P) -> XrfResult<TranslationProjectJson> {
    let dir: &Path = dir.as_ref();
    let mut project_json: TranslationProjectJson = Default::default();

    for entry in WalkDir::new(dir).sort_by_file_name() {
      let entry: DirEntry = entry.map_err(|error| {
        XrfError::new_read_error(format!(
          "Failed to walk translation directory '{}': {error}",
          dir.display()
        ))
      })?;
      let entry_path: &Path = entry.path();

      if !entry_path.is_file() {
        continue;
      }

      match entry_path.extension().and_then(|extension| extension.to_str()) {
        Some("json") => {
          let entry_key = entry_path.to_str().ok_or_else(|| {
            XrfError::new_invalid_error(format!(
              "Translation path cannot be represented as Unicode: {}",
              entry_path.display()
            ))
          })?;

          project_json.insert(entry_key.into(), Self::read_translation_json_by_path(entry_path)?);
        }
        Some("xml") => {
          let translations = Self::read_translation_xml_by_path(entry_path)?;
          let xml_entry_path = Self::get_xml_name_from_path(entry_path).ok_or_else(|| {
            XrfError::new_invalid_error(format!(
              "Translation XML path cannot be represented as Unicode: {}",
              entry_path.display()
            ))
          })?;

          Self::merge_translation_xml(&xml_entry_path, &mut project_json, translations)?;
        }
        Some(_) => log::warn!("Skip unknown extension translation file {}", entry_path.display()),
        None => {}
      }
    }

    Self::validate_project_duplicates(&project_json)?;

    Ok(project_json)
  }

  fn merge_translation_xml(key: &str, to: &mut TranslationProjectJson, from: TranslationJson) -> XrfResult {
    if let Some(existing_translations) = to.get_mut(key) {
      for (id, lang_translations) in from {
        if let Some(existing_entry) = existing_translations.get_mut(&id) {
          for (lang, text) in lang_translations {
            if existing_entry.contains_key(&lang) {
              return Err(XrfError::new_invalid_error(format!(
                "Translation source '{key}' contains duplicate language '{lang}' for ID '{id}'"
              )));
            }

            existing_entry.insert(lang, text);
          }
        } else {
          existing_translations.insert(id, lang_translations);
        }
      }
    } else {
      to.insert(key.into(), from);
    }

    Ok(())
  }

  fn validate_project_duplicates(project: &TranslationProjectJson) -> XrfResult {
    let mut seen_ids: HashSet<&str> = HashSet::new();

    for (source_path, translations) in project {
      for id in translations.keys() {
        if !seen_ids.insert(id) {
          return Err(XrfError::new_invalid_error(format!(
            "Translation project contains duplicate ID '{id}' in source '{source_path}'"
          )));
        }
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use std::io::Write;
  use std::path::PathBuf;

  use xrf_error::XrfResult;
  use xrf_test_utils::utils::{
    get_absolute_generated_test_resource_path, get_absolute_test_sample_file_path,
    overwrite_generated_test_resource_as_file,
  };

  use crate::project::translation_project::TranslationProject;
  use crate::types::{TranslationProjectJson, TranslationVariant};
  use crate::{TranslationEntry, TranslationJson};

  #[test]
  fn duplicate_ids_across_project_files_return_an_error() -> XrfResult {
    let test_root = "translation_project_read/duplicate_project_ids";
    let mut first = overwrite_generated_test_resource_as_file(&format!("{test_root}/first.json"))?;
    first.write_all(br#"{"st_test":{"eng":"first"}}"#)?;
    let mut second = overwrite_generated_test_resource_as_file(&format!("{test_root}/second.json"))?;
    second.write_all(br#"{"st_test":{"eng":"second"}}"#)?;

    drop(first);
    drop(second);

    let root = get_absolute_generated_test_resource_path(test_root);
    let error = TranslationProject::read_project(root).unwrap_err();

    assert!(error.to_string().contains("project contains duplicate ID 'st_test'"));

    Ok(())
  }

  #[test]
  fn reads_and_merges_a_multilanguage_xml_project() {
    let base_xml_path: PathBuf = get_absolute_test_sample_file_path(file!(), "multilang.multilang.xml");
    let project_json: TranslationProjectJson =
      TranslationProject::read_project(base_xml_path.parent().expect("Parent dir expected"))
        .expect("Expected project data");

    assert_eq!(project_json.len(), 1);

    let translations: &TranslationJson = project_json.get(base_xml_path.to_str().unwrap()).unwrap();
    let entry: &TranslationEntry = translations.get("st_multilang_example").unwrap();

    assert_eq!(
      entry.get("eng").unwrap().as_ref(),
      Some(&TranslationVariant::String(String::from("eng text")))
    );
    assert_eq!(
      entry.get("ukr").unwrap().as_ref(),
      Some(&TranslationVariant::String(String::from("ukr text")))
    );
  }
}
