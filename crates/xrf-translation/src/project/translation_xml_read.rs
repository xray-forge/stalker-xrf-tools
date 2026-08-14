use std::fs::File;
use std::io::Read;
use std::path::Path;

use xrf_error::{XrfError, XrfResult};
use xrf_utils::decode_bytes_to_string;
use xrf_xml::{declared_xml_encoding, deserialize_xml, deserialize_xml_bytes};

use crate::TranslationLanguage;
use crate::language::MULTILANGUAGE;
use crate::project::translation_project::TranslationProject;
use crate::types::{TranslationCompiledXml, TranslationEntry, TranslationJson, TranslationVariant};

impl TranslationProject {
  pub(crate) fn read_translation_xml_by_path(path: &Path) -> XrfResult<TranslationJson> {
    let mut data: Vec<u8> = Vec::new();

    File::open(path)?.read_to_end(&mut data)?;

    let xml_language = Self::get_locale_from_path(path).unwrap_or(TranslationLanguage::English);
    let declared_encoding = declared_xml_encoding(&data)?;

    if let Some(declared_encoding) = declared_encoding
      && Self::get_locale_from_path(path).is_some()
      && declared_encoding != xml_language.get_language_encoder()
    {
      log::warn!(
        "Translation XML '{}' declares {}, but the '{}' filename suffix requires {}",
        path.display(),
        declared_encoding.name(),
        xml_language,
        xml_language.get_language_encoding(),
      );
    }

    let xml_data: TranslationCompiledXml = if declared_encoding.is_some() {
      deserialize_xml_bytes(&data)?
    } else {
      deserialize_xml(&decode_bytes_to_string(&data, xml_language.get_language_encoder())?)?
    };

    let mut json: TranslationJson = Default::default();

    for entry in xml_data.string {
      if json.contains_key(&entry.id) {
        return Err(XrfError::new_parsing_error(format!(
          "Translation XML '{}' contains duplicate ID '{}'",
          path.display(),
          entry.id,
        )));
      }

      let mut translation_entry: TranslationEntry = Default::default();

      translation_entry.insert(xml_language.to_string(), Some(TranslationVariant::String(entry.text)));
      json.insert(entry.id, translation_entry);
    }

    Ok(json)
  }

  pub(crate) fn get_locale_from_path(path: &Path) -> Option<TranslationLanguage> {
    let file_name: &str = path.file_name()?.to_str()?;
    let mut parts = file_name.rsplit('.');

    parts.next()?;

    TranslationLanguage::from_str_single(parts.next()?).ok()
  }

  /// Map `name.lang.xml` files into a single `name.multilang.xml` project entry.
  pub(crate) fn get_xml_name_from_path(path: &Path) -> Option<String> {
    if let Some(file_name) = path.file_name().and_then(|file_name| file_name.to_str()) {
      let parts: Vec<&str> = file_name.split('.').collect();

      if parts.len() > 2 && TranslationLanguage::from_str_single(parts[parts.len() - 2]).is_ok() {
        let base_name = format!("{}.{}.xml", parts[..parts.len() - 2].join("."), MULTILANGUAGE);

        return path
          .parent()
          .unwrap_or_else(|| Path::new(""))
          .join(base_name)
          .to_str()
          .map(str::to_owned);
      }
    }

    path.to_str().map(str::to_owned)
  }
}

#[cfg(test)]
mod tests {
  use std::io::Write;
  use std::path::PathBuf;

  use xrf_error::XrfResult;
  use xrf_test_utils::utils::{
    get_absolute_generated_test_resource_path, get_absolute_test_resource_path,
    overwrite_generated_test_resource_as_file,
  };
  use xrf_utils::{encode_string_to_bytes, get_windows1251_encoder, get_windows1252_encoder};

  use crate::project::translation_project::TranslationProject;
  use crate::types::TranslationVariant;

  #[test]
  fn reads_xml_using_its_declared_windows_1251_encoding() -> XrfResult {
    let relative_path = "translation_project_read/declared_encoding.ukr.xml";
    let source = "<?xml version=\"1.0\" encoding=\"windows-1251\"?><string_table><string id=\"st_test\"><text>\u{041f}\u{0440}\u{0438}\u{0432}\u{0456}\u{0442}</text></string></string_table>";
    let encoded = encode_string_to_bytes(source, get_windows1251_encoder())?;
    let mut file = overwrite_generated_test_resource_as_file(relative_path)?;
    file.write_all(&encoded)?;
    drop(file);

    let translations =
      TranslationProject::read_translation_xml_by_path(&get_absolute_generated_test_resource_path(relative_path))?;

    assert_eq!(
      translations["st_test"]["ukr"],
      Some(TranslationVariant::String(String::from(
        "\u{041f}\u{0440}\u{0438}\u{0432}\u{0456}\u{0442}"
      )))
    );

    Ok(())
  }

  #[test]
  fn declarationless_xml_uses_the_language_code_page_fallback() -> XrfResult {
    let relative_path = "translation_project_read/declarationless.xml";
    let source = "<string_table><string id=\"st_test\"><text>\u{00c0} bient\u{00f4}t</text></string></string_table>";
    let encoded = encode_string_to_bytes(source, get_windows1252_encoder())?;
    let mut file = overwrite_generated_test_resource_as_file(relative_path)?;
    file.write_all(&encoded)?;
    drop(file);

    let translations =
      TranslationProject::read_translation_xml_by_path(&get_absolute_generated_test_resource_path(relative_path))?;

    assert_eq!(
      translations["st_test"]["eng"],
      Some(TranslationVariant::String(String::from("\u{00c0} bient\u{00f4}t")))
    );

    Ok(())
  }

  #[test]
  fn declaration_overrides_the_language_suffix_encoding() -> XrfResult {
    let relative_path = "translation_project_read/mismatched_encoding.eng.xml";
    let source = "<?xml version=\"1.0\" encoding=\"windows-1251\"?><string_table><string id=\"st_test\"><text>\u{041f}\u{0440}\u{0438}\u{0432}\u{0435}\u{0442}</text></string></string_table>";
    let encoded = encode_string_to_bytes(source, get_windows1251_encoder())?;
    let mut file = overwrite_generated_test_resource_as_file(relative_path)?;
    file.write_all(&encoded)?;
    drop(file);

    let path = get_absolute_generated_test_resource_path(relative_path);
    let translations = TranslationProject::read_translation_xml_by_path(&path)?;

    assert_eq!(
      translations["st_test"]["eng"],
      Some(TranslationVariant::String(String::from(
        "\u{041f}\u{0440}\u{0438}\u{0432}\u{0435}\u{0442}"
      )))
    );

    Ok(())
  }

  #[test]
  fn duplicate_xml_ids_return_a_parsing_error() -> XrfResult {
    let relative_path = "translation_project_read/duplicate_ids.xml";
    let mut file = overwrite_generated_test_resource_as_file(relative_path)?;
    file.write_all(
      br#"<string_table><string id="st_test"><text>first</text></string><string id="st_test"><text>second</text></string></string_table>"#,
    )?;
    drop(file);

    let path = get_absolute_generated_test_resource_path(relative_path);
    let error = TranslationProject::read_translation_xml_by_path(&path).unwrap_err();

    assert!(error.to_string().contains("contains duplicate ID 'st_test'"));

    Ok(())
  }

  #[test]
  fn maps_language_specific_xml_names_to_a_multilanguage_entry() {
    let dir: PathBuf = get_absolute_test_resource_path(file!());
    let generic_xml_path = dir.join("some.path.xml");
    let eng_xml_path = dir.join("example.eng.xml");
    let ukr_xml_path = dir.join("example.ukr.xml");

    assert_eq!(
      TranslationProject::get_xml_name_from_path(&generic_xml_path).expect("Expected path"),
      dir.join("some.path.xml").to_str().expect("Expected path"),
    );
    assert_eq!(
      TranslationProject::get_xml_name_from_path(&eng_xml_path).expect("Expected path"),
      dir.join("example.multilang.xml").to_str().expect("Expected path"),
    );
    assert_eq!(
      TranslationProject::get_xml_name_from_path(&ukr_xml_path).expect("Expected path"),
      dir.join("example.multilang.xml").to_str().expect("Expected path"),
    );
  }
}
