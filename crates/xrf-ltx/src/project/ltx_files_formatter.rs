use std::path::PathBuf;
use std::time::Instant;

use xrf_error::XrfResult;

use crate::Ltx;
use crate::project::ltx_format_options::LtxFormatOptions;
use crate::project::ltx_project_format_result::LtxProjectFormatResult;

/// Formatter of arbitrary sets of LTX files.
pub struct LtxFilesFormatter {}

impl LtxFilesFormatter {
  /// Format provided LTX files, rewriting the ones that are not formatted yet.
  pub fn format_opt(files: &[PathBuf], options: LtxFormatOptions) -> XrfResult<LtxProjectFormatResult> {
    Self::process(files, options, true)
  }

  /// Check format of provided LTX files without rewriting any of them.
  pub fn check_format_opt(files: &[PathBuf], options: LtxFormatOptions) -> XrfResult<LtxProjectFormatResult> {
    Self::process(files, options, false)
  }

  /// Format provided LTX files with default options.
  pub fn format(files: &[PathBuf]) -> XrfResult<LtxProjectFormatResult> {
    Self::format_opt(files, LtxFormatOptions::default())
  }

  /// Check format of provided LTX files with default options.
  pub fn check_format(files: &[PathBuf]) -> XrfResult<LtxProjectFormatResult> {
    Self::check_format_opt(files, LtxFormatOptions::default())
  }
}

impl LtxFilesFormatter {
  /// Format or check provided LTX files, writing the formatted output only when requested.
  fn process(files: &[PathBuf], options: LtxFormatOptions, is_write: bool) -> XrfResult<LtxProjectFormatResult> {
    let mut result: LtxProjectFormatResult = LtxProjectFormatResult::new();
    let started_at: Instant = Instant::now();

    if is_write {
      xrf_output::heading!(options.output, "Formatting {} file(s)", files.len());
    } else {
      xrf_output::heading!(options.output, "Checking {} file(s)", files.len());
    }

    for file in files {
      if Ltx::format_file(file, is_write)? {
        result.invalid_files += 1;
        result.to_format.push(file.clone());

        xrf_output::info!(
          options.output,
          "{}: {}",
          if is_write { "Formatted" } else { "Not formatted" },
          file.display()
        );
      } else {
        result.valid_files += 1;
      }

      result.total_files += 1;
    }

    result.duration = started_at.elapsed().as_millis();

    Self::report(&result, &options, is_write);

    Ok(result)
  }

  /// Report resulting statistics of format or check run.
  fn report(result: &LtxProjectFormatResult, options: &LtxFormatOptions, is_write: bool) {
    let duration: f64 = (result.duration as f64) / 1000.0;

    if is_write {
      xrf_output::info!(
        options.output,
        "Formatted {}/{} files in {} sec",
        result.invalid_files,
        result.total_files,
        duration
      );
    } else if result.invalid_files == 0 {
      xrf_output::success!(
        options.output,
        "All {} files are formatted, checked in {} sec",
        result.total_files,
        duration
      );
    } else {
      xrf_output::warning!(
        options.output,
        "Format issues with {}/{} files in {} sec",
        result.invalid_files,
        result.total_files,
        duration
      );
    }
  }
}

#[cfg(test)]
mod tests {
  use std::fs;
  use std::path::PathBuf;

  use xrf_error::XrfResult;

  use crate::project::ltx_files_formatter::LtxFilesFormatter;
  use crate::project::ltx_project_format_result::LtxProjectFormatResult;

  fn create_root(name: &str) -> XrfResult<PathBuf> {
    let root: PathBuf = std::env::temp_dir().join(format!("xrf-ltx-format-{name}-{}", std::process::id()));

    if root.exists() {
      fs::remove_dir_all(&root)?;
    }

    fs::create_dir_all(&root)?;

    Ok(root)
  }

  #[test]
  fn formats_only_provided_files() -> XrfResult {
    let root: PathBuf = create_root("provided")?;
    let first: PathBuf = root.join("first.ltx");
    let second: PathBuf = root.join("second.ltx");

    fs::write(&first, "[a]\nkey=value\n")?;
    fs::write(&second, "[b]\nkey=value\n")?;

    let result: LtxProjectFormatResult = LtxFilesFormatter::format(std::slice::from_ref(&first))?;

    assert_eq!(result.total_files, 1);
    assert_eq!(result.invalid_files, 1);
    assert_eq!(result.to_format, vec![first.clone()]);

    assert_eq!(fs::read_to_string(&first)?, "[a]\r\nkey = value\r\n");
    assert_eq!(fs::read_to_string(&second)?, "[b]\nkey=value\n");

    fs::remove_dir_all(root)?;

    Ok(())
  }

  #[test]
  fn reports_already_formatted_files_as_valid() -> XrfResult {
    let root: PathBuf = create_root("valid")?;
    let file: PathBuf = root.join("formatted.ltx");

    fs::write(&file, "[a]\r\nkey = value\r\n")?;

    let result: LtxProjectFormatResult = LtxFilesFormatter::format(&[file])?;

    assert_eq!(result.total_files, 1);
    assert_eq!(result.valid_files, 1);
    assert_eq!(result.invalid_files, 0);
    assert!(result.to_format.is_empty());

    fs::remove_dir_all(root)?;

    Ok(())
  }

  #[test]
  fn check_does_not_write_files() -> XrfResult {
    let root: PathBuf = create_root("check")?;
    let file: PathBuf = root.join("unformatted.ltx");

    fs::write(&file, "[a]\nkey=value\n")?;

    let result: LtxProjectFormatResult = LtxFilesFormatter::check_format(std::slice::from_ref(&file))?;

    assert_eq!(result.total_files, 1);
    assert_eq!(result.invalid_files, 1);
    assert_eq!(result.to_format, vec![file.clone()]);
    assert_eq!(fs::read_to_string(&file)?, "[a]\nkey=value\n");

    fs::remove_dir_all(root)?;

    Ok(())
  }

  #[test]
  fn formats_file_with_unresolvable_inherit_and_include() -> XrfResult {
    let root: PathBuf = create_root("standalone")?;
    let file: PathBuf = root.join("standalone.ltx");

    fs::write(
      &file,
      "#include \"missing\\absent.ltx\"\n[af_custom]:af_base\ncost=100\n",
    )?;

    let result: LtxProjectFormatResult = LtxFilesFormatter::format(std::slice::from_ref(&file))?;

    assert_eq!(result.total_files, 1);
    assert_eq!(
      fs::read_to_string(&file)?,
      "#include \"missing\\absent.ltx\"\r\n\r\n[af_custom]:af_base\r\ncost = 100\r\n"
    );

    fs::remove_dir_all(root)?;

    Ok(())
  }

  #[test]
  fn handles_empty_file_list() -> XrfResult {
    let result: LtxProjectFormatResult = LtxFilesFormatter::format(&[])?;

    assert_eq!(result.total_files, 0);
    assert_eq!(result.invalid_files, 0);
    assert!(result.to_format.is_empty());

    Ok(())
  }
}
