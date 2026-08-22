use xrf_test_utils::utils::{build_absolute_generated_test_resource_path, write_generated_test_resource};

use crate::project::descriptor::TranslationProjectMode;
use crate::project::layout::detect_mode;

#[test]
fn language_directories_look_like_gamedata() {
  let root: &str = "layout/gamedata";

  write_generated_test_resource(&format!("{root}/rus/st_test.xml"), "<string_table></string_table>")
    .expect("Expected a written test file");

  assert_eq!(
    detect_mode(&build_absolute_generated_test_resource_path(root)),
    TranslationProjectMode::Gamedata
  );
}

#[test]
fn a_json_map_looks_like_a_source_tree() {
  let root: &str = "layout/source_json";

  write_generated_test_resource(&format!("{root}/st_test.json"), r#"{"st_hello":{"eng":"Hello"}}"#)
    .expect("Expected a written test file");

  assert_eq!(
    detect_mode(&build_absolute_generated_test_resource_path(root)),
    TranslationProjectMode::Source
  );
}

#[test]
fn a_language_suffixed_xml_looks_like_a_source_tree() {
  let root: &str = "layout/source_xml";

  write_generated_test_resource(&format!("{root}/dialogs.eng.xml"), "<string_table></string_table>")
    .expect("Expected a written test file");

  assert_eq!(
    detect_mode(&build_absolute_generated_test_resource_path(root)),
    TranslationProjectMode::Source
  );
}

#[test]
fn an_unreadable_directory_falls_back_to_source() {
  // Guessing gamedata for something unreadable would preselect the mode that rewrites shipped files.
  assert_eq!(
    detect_mode(&build_absolute_generated_test_resource_path("layout/does_not_exist")),
    TranslationProjectMode::Source
  );
}
