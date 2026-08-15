use std::path::Path;

use xrf_report::Status;

use crate::project::verify::result::ProjectVerifyResult;

#[test]
fn reports_missing_translations_as_findings() {
  let mut result: ProjectVerifyResult = ProjectVerifyResult::new();

  result.record_missing_translation(Path::new("translations/dialogs.json"), "st_dialog_hello", "ukr");

  let report = result.to_report();

  assert_eq!(result.status(), Status::Failed);
  assert_eq!(report.status(), Status::Failed);
  assert_eq!(report.checks()[0].id().as_str(), "translations");
  assert_eq!(
    report.checks()[0].findings()[0].rule_id().as_str(),
    "translations.missing"
  );
  assert_eq!(
    report.checks()[0].findings()[0].subject(),
    Some("translations/dialogs.json")
  );
}

#[test]
fn a_project_with_nothing_missing_passes() {
  let result: ProjectVerifyResult = ProjectVerifyResult::new();

  assert_eq!(result.status(), Status::Passed);
  assert!(result.findings().is_empty());
}

#[test]
fn merging_accumulates_counts_and_findings() {
  let mut first: ProjectVerifyResult = ProjectVerifyResult::new();
  let mut second: ProjectVerifyResult = ProjectVerifyResult::new();

  first.checked_translations_count = 2;
  first.record_missing_translation(Path::new("a.json"), "st_a", "ukr");

  second.checked_translations_count = 3;
  second.record_missing_translation(Path::new("b.json"), "st_b", "pol");

  first.merge(second);

  assert_eq!(first.checked_translations_count, 5);
  assert_eq!(first.missing_translations_count, 2);
  assert_eq!(first.findings().len(), 2);
}
