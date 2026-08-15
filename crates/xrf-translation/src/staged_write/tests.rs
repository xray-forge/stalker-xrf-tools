use std::fs;

use xrf_error::XrfResult;
use xrf_test_utils::utils::write_generated_test_resource;

use crate::staged_write::write_file_staged;

#[test]
fn replaces_an_existing_file_after_staging_succeeds() -> XrfResult {
  let path = write_generated_test_resource("staged_write/replaced.txt", "original")?;

  write_file_staged(&path, b"replacement")?;

  assert_eq!(fs::read(path)?, b"replacement");

  Ok(())
}

#[test]
fn leaves_no_staging_files_behind() -> XrfResult {
  // Its own directory, because the assertion scans one and the sibling test writes here too.
  let path = write_generated_test_resource("staged_write/leftovers/no_leftovers.txt", "original")?;

  write_file_staged(&path, b"replacement")?;

  let leftovers: Vec<String> = fs::read_dir(path.parent().expect("Expected a parent"))?
    .filter_map(Result::ok)
    .map(|entry| entry.file_name().to_string_lossy().into_owned())
    .filter(|name| name.contains("xrf-tmp") || name.contains("xrf-backup"))
    .collect();

  assert!(leftovers.is_empty(), "left behind: {leftovers:?}");

  Ok(())
}
