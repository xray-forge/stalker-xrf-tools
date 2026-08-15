use std::path::Path;

use indexmap::IndexMap;
use xrf_error::XrfResult;
use xrf_test_utils::utils::write_generated_test_resource;

use crate::constants::LANGUAGE_NEUTRAL;
use crate::edit::TranslationEdit;
use crate::json::read::read_json;
use crate::project::descriptor::TranslationProjectDescriptor;
use crate::project::edit::{apply_edits, find_unwritable_character};
use crate::types::TranslationVariant;
use crate::xml::read::read_string_table;

fn descriptor(encodings: &[(&str, &str)]) -> TranslationProjectDescriptor {
  TranslationProjectDescriptor {
    encodings: encodings
      .iter()
      .map(|(language, encoding)| (String::from(*language), String::from(*encoding)))
      .collect::<IndexMap<String, String>>(),
    ..Default::default()
  }
}

fn set(id: &str, text: &str) -> TranslationEdit {
  TranslationEdit::Set {
    id: String::from(id),
    value: TranslationVariant::String(String::from(text)),
  }
}

#[test]
fn routes_a_json_source_to_the_json_writer() -> XrfResult {
  let path = write_generated_test_resource("project_edit/dispatch.json", r#"{"st_a":{"eng":"A"}}"#)?;

  apply_edits(&path, "eng", &[set("st_a", "B")])?;

  assert_eq!(
    read_json(&path)?["st_a"]["eng"],
    Some(TranslationVariant::String(String::from("B")))
  );

  Ok(())
}

#[test]
fn routes_an_xml_source_to_the_splice_writer() -> XrfResult {
  let path = write_generated_test_resource(
    "project_edit/dispatch.xml",
    "<string_table><string id=\"st_a\"><text>A</text></string></string_table>",
  )?;

  apply_edits(&path, "eng", &[set("st_a", "B")])?;

  assert_eq!(
    read_string_table(&path)?,
    vec![(String::from("st_a"), String::from("B"))]
  );

  Ok(())
}

#[test]
fn refuses_a_file_it_has_no_writer_for() {
  let error = apply_edits(Path::new("translations/notes.txt"), "eng", &[]).unwrap_err();

  assert!(error.to_string().contains("not a file this can write"));
}

#[test]
fn accepts_text_the_target_code_page_can_hold() -> XrfResult {
  let project = descriptor(&[("rus", "windows-1251")]);

  assert_eq!(find_unwritable_character(&project, "rus", "Привет")?, None);

  Ok(())
}

#[test]
fn names_the_character_a_target_cannot_hold() -> XrfResult {
  let project = descriptor(&[("fra", "windows-1252")]);
  let reported = find_unwritable_character(&project, "fra", "Привет")?.expect("Expected a refusal");

  assert!(reported.contains("U+041F"));
  assert!(reported.contains("fra"));

  Ok(())
}

#[test]
fn neutral_text_must_survive_every_language() -> XrfResult {
  let project = descriptor(&[]);

  // Copied into all of them by the build, so passing in one code page is not enough.
  assert!(find_unwritable_character(&project, LANGUAGE_NEUTRAL, "Привет")?.is_some());
  assert_eq!(find_unwritable_character(&project, LANGUAGE_NEUTRAL, "plain")?, None);

  Ok(())
}

#[test]
fn a_language_with_no_recorded_encoding_is_not_second_guessed() -> XrfResult {
  let project = descriptor(&[]);

  // Nothing was read for it, so there is no code page to judge against and nothing to report.
  assert_eq!(find_unwritable_character(&project, "jpn", "日本語")?, None);

  Ok(())
}
