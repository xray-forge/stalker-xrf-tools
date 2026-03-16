use crate::language::MULTILANGUAGE;
use crate::project::translation_project::TranslationProject;
use crate::project::translation_project_constants::ALLOWED_PROJECT_READ_EXTENSIONS;
use crate::types::{
  TranslationCompiledXml, TranslationJson, TranslationProjectJson, TranslationVariant,
};
use crate::{TranslationEntry, TranslationLanguage};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};
use xray_error::XRayResult;
use xray_utils::{XRayEncoding, decode_bytes_to_string};

impl TranslationProject {
  pub fn read_project<P: AsRef<Path>>(dir: P) -> XRayResult<TranslationProjectJson> {
    let mut project_json: TranslationProjectJson = Default::default();

    // Filter all the entries that are not accessed by other files and represent entry points.
    for entry in WalkDir::new(dir) {
      let entry: DirEntry = match entry {
        Ok(entry) => entry,
        Err(error) => {
          return Err(
            error
              .into_io_error()
              .expect("WalkError transformation")
              .into(),
          );
        }
      };

      let entry_path: &Path = entry.path();

      if entry_path.is_file()
        && let Some(extension) = entry_path.extension()
      {
        if extension == "json" {
          project_json.insert(
            entry_path
              .to_str()
              .expect("Entry path when read translation")
              .into(),
            Self::read_translation_json_by_path(&entry_path)?,
          );
        } else if extension == "xml" {
          let translations: TranslationJson = Self::read_translation_xml_by_path(&entry_path)?;

          let xml_entry_path: String =
            Self::get_xml_name_from_path(&entry_path).expect("Xml file entry path");

          // Merge or insert based on existing translation keys.
          Self::merge_translation_xml(&xml_entry_path, &mut project_json, translations);
        } else {
          log::warn!(
            "Skip unknown extension translation file {}",
            entry_path.display()
          );
        }
      }
    }

    // todo: Validate and de-duplicate?

    Ok(project_json)
  }

  pub fn read_translation_json_by_path<P: AsRef<Path>>(path: &P) -> XRayResult<TranslationJson> {
    let mut data: Vec<u8> = Vec::new();

    File::open(path)?.read_to_end(&mut data)?;

    Ok(serde_json::from_slice(&data).expect("Expected valid JSON source file with standard format"))
  }

  pub fn read_translation_xml_by_path<P: AsRef<Path>>(path: &P) -> XRayResult<TranslationJson> {
    let mut data: Vec<u8> = Vec::new();

    File::open(path)?.read_to_end(&mut data)?;

    let xml_language: TranslationLanguage =
      Self::get_locale_from_path(&path).unwrap_or(TranslationLanguage::English);
    let xml_encoding: XRayEncoding = xml_language.get_language_encoder();
    let xml_content: String = decode_bytes_to_string(&data, xml_encoding)?;
    let xml_data: TranslationCompiledXml = quick_xml::de::from_str(&xml_content)
      .expect("Expected valid XML source file with standard format");

    let mut json: TranslationJson = HashMap::new();

    for entry in xml_data.string {
      let mut translation_entry: TranslationEntry = HashMap::new();

      translation_entry.insert(
        xml_language.to_string(),
        Some(TranslationVariant::String(entry.text)),
      );

      json.insert(entry.id, translation_entry);
    }

    Ok(json)
  }

  /// Merge or insert XML -> JSON translation data based on existing translation keys.
  pub fn merge_translation_xml(key: &str, to: &mut TranslationProjectJson, from: TranslationJson) {
    if let Some(existing_translations) = to.get_mut(key) {
      for (id, lang_translations) in from {
        if let Some(existing_entry) = existing_translations.get_mut(&id) {
          for (lang, text) in lang_translations {
            existing_entry.insert(lang, text);
          }
        } else {
          existing_translations.insert(id, lang_translations);
        }
      }
    } else {
      to.insert(key.into(), from);
    }
  }

  pub fn can_read_path<P: AsRef<Path>>(&self, path: &P) -> bool {
    if let Some(extension) = path.as_ref().extension() {
      for allowed in ALLOWED_PROJECT_READ_EXTENSIONS {
        if (*allowed).eq(extension) {
          return true;
        }
      }
    }

    false
  }

  pub fn get_locale_from_path<P: AsRef<Path>>(path: &P) -> Option<TranslationLanguage> {
    match path.as_ref().file_name() {
      Some(name) => {
        if let Some(name) = name.to_str() {
          let parts: Vec<&str> = name.split('.').collect();
          let parts_count: usize = parts.len();

          if parts_count > 2 {
            let lang_part = parts
              .get(parts_count - 2)
              .expect("Language path in translation file");

            return TranslationLanguage::from_str_single(lang_part).ok();
          }
        }

        None
      }
      None => None,
    }
  }

  /// Map multilang XML files in a format of `name.lang.xml` into single `name.multilang.xml` entries.
  pub fn get_xml_name_from_path<P: AsRef<Path>>(path: &P) -> Option<String> {
    let path: &Path = path.as_ref();

    if let Some(file_name) = path.file_name()
      && let Some(file_name) = file_name.to_str()
    {
      let parts: Vec<&str> = file_name.split('.').collect();

      // Confirm language is part of the file extension.
      if parts.len() > 2 && TranslationLanguage::from_str_single(parts[parts.len() - 2]).is_ok() {
        let base_name: String = format!(
          "{}.{}.xml",
          parts[..parts.len() - 2].join("."),
          MULTILANGUAGE
        );

        return path
          .parent()
          .unwrap_or_else(|| Path::new(""))
          .join(base_name)
          .to_str()
          .map(|it| it.to_string());
      }
    }

    path.to_str().map(|it| it.to_string())
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

#[cfg(test)]
mod tests {
  use crate::project::translation_project::TranslationProject;
  use crate::types::{TranslationProjectJson, TranslationVariant};
  use crate::{TranslationEntry, TranslationJson};
  use std::path::PathBuf;
  use xray_test_utils::utils::{
    get_absolute_test_resource_path, get_absolute_test_sample_file_path,
  };

  #[test]
  fn test_read_xml_project() {
    let base_xml_path: PathBuf =
      get_absolute_test_sample_file_path(file!(), "multilang.multilang.xml");

    let project_json: TranslationProjectJson =
      TranslationProject::read_project(base_xml_path.parent().expect("Parent dir expected"))
        .expect("Expected project data");

    assert_eq!(project_json.len(), 1);

    let translations: &TranslationJson = project_json.get(base_xml_path.to_str().unwrap()).unwrap();

    assert!(translations.contains_key("st_multilang_example"));

    let entry: &TranslationEntry = translations.get("st_multilang_example").unwrap();

    if let TranslationVariant::String(value) = entry.get("eng").unwrap().as_ref().unwrap() {
      assert_eq!(value, "eng text");
    } else {
      panic!("Expected String variant");
    }

    if let TranslationVariant::String(value) = entry.get("ukr").unwrap().as_ref().unwrap() {
      assert_eq!(value, "ukr text");
    } else {
      panic!("Expected String variant");
    }
  }

  #[test]
  fn test_get_xml_name_from_path() {
    let dir: PathBuf = get_absolute_test_resource_path(file!());
    let generic_xml_path: PathBuf = dir.join("some.path.xml");
    let eng_xml_path: PathBuf = dir.join("example.eng.xml");
    let ukr_xml_path: PathBuf = dir.join("example.ukr.xml");

    assert_eq!(
      TranslationProject::get_xml_name_from_path(&generic_xml_path).expect("Expected path"),
      dir.join("some.path.xml").to_str().expect("Expected path"),
    );
    assert_eq!(
      TranslationProject::get_xml_name_from_path(&eng_xml_path).expect("Expected path"),
      dir
        .join("example.multilang.xml")
        .to_str()
        .expect("Expected path"),
    );
    assert_eq!(
      TranslationProject::get_xml_name_from_path(&ukr_xml_path).expect("Expected path"),
      dir
        .join("example.multilang.xml")
        .to_str()
        .expect("Expected path"),
    );
  }
}
