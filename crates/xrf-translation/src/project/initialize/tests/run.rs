use std::fs;

use xrf_error::XrfResult;
use xrf_test_utils::utils::write_generated_test_resource;

use crate::json::read::read_json;
use crate::language::TranslationLanguage;
use crate::project::initialize::options::ProjectInitializeOptions;
use crate::project::initialize::run::initialize_json_file;
use crate::types::TranslationVariant;

fn options(path: std::path::PathBuf) -> ProjectInitializeOptions {
  ProjectInitializeOptions {
    output: xrf_output::OutputOptions::default(),
    path,
  }
}

#[test]
fn initialization_replaces_json_only_after_writing_a_valid_document() -> XrfResult {
  let path = write_generated_test_resource("initialize/transactional.json", r#"{"st_test":{"eng":"original"}}"#)?;

  initialize_json_file(&path, &options(path.clone()))?;

  let initialized = read_json(&path)?;

  assert_eq!(
    initialized["st_test"]["eng"],
    Some(TranslationVariant::String(String::from("original")))
  );
  assert!(
    TranslationLanguage::get_all_strings()
      .iter()
      .all(|language| initialized["st_test"].contains_key(language))
  );

  Ok(())
}

#[test]
fn an_already_complete_file_is_left_untouched() -> XrfResult {
  let languages: String = TranslationLanguage::get_all_strings()
    .iter()
    .map(|language| format!("\"{language}\":null"))
    .collect::<Vec<_>>()
    .join(",");
  let path = write_generated_test_resource("initialize/complete.json", format!("{{\"st_test\":{{{languages}}}}}"))?;
  let before: Vec<u8> = fs::read(&path)?;

  initialize_json_file(&path, &options(path.clone()))?;

  // Nothing was added, so nothing is rewritten - a repeated run leaves the tree alone.
  assert_eq!(fs::read(&path)?, before);

  Ok(())
}
