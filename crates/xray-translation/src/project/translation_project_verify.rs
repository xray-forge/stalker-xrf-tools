use crate::project::translation_project_verify_result::ProjectVerifyResult;
use crate::types::TranslationJson;
use crate::{ProjectVerifyOptions, TranslationLanguage, TranslationProject};
use std::ffi::OsStr;
use std::path::Path;
use std::time::Instant;
use walkdir::{DirEntry, WalkDir};
use xray_error::XRayResult;

impl TranslationProject {
  pub fn verify_dir(dir: &Path, options: &ProjectVerifyOptions) -> XRayResult<ProjectVerifyResult> {
    log::info!("Verifying dir {}", dir.display());

    if options.is_logging_enabled() {
      println!("Verifying dir {}", dir.display());
    }

    let started_at: Instant = Instant::now();
    let mut result: ProjectVerifyResult = ProjectVerifyResult::new();

    // Filter all the entries that are not accessed by other files and represent entry points.
    for entry in WalkDir::new(dir) {
      let entry: DirEntry = match entry {
        Ok(entry) => entry,
        Err(error) => return Err(error.into_io_error().unwrap().into()),
      };

      let entry_path: &Path = entry.path();

      if entry_path.is_file() {
        let file_result: ProjectVerifyResult = Self::verify_file(entry_path, options)?;

        result.missing_translations_count += file_result.missing_translations_count;
        result.checked_translations_count += file_result.checked_translations_count;
      }
    }

    result.duration = started_at.elapsed().as_millis();

    log::info!(
      "Verified dir {} in {} sec",
      dir.display(),
      (result.duration as f64) / 1000.0
    );

    Ok(result)
  }

  pub fn verify_file(
    path: &Path,
    options: &ProjectVerifyOptions,
  ) -> XRayResult<ProjectVerifyResult> {
    let extension: Option<&OsStr> = path.extension();

    if let Some(extension) = extension {
      if extension == "json" {
        return Self::verify_json_file(path, options);
      } else {
        log::info!("Skip file {}", path.display());

        if options.is_logging_enabled() {
          println!("Skip file {}", path.display());
        }
      }
    }

    Ok(ProjectVerifyResult::new())
  }

  pub fn verify_json_file(
    path: &Path,
    options: &ProjectVerifyOptions,
  ) -> XRayResult<ProjectVerifyResult> {
    let mut result: ProjectVerifyResult = ProjectVerifyResult::new();

    log::info!("Verifying dynamic JSON file {}", path.display());

    let started_at: Instant = Instant::now();
    let parsed: TranslationJson = Self::read_translation_json_by_path(path)?;

    let languages: Vec<String> = if options.language == TranslationLanguage::All {
      TranslationLanguage::get_all_strings()
    } else {
      vec![options.language.to_string()]
    };

    for language in languages {
      for (key, entry) in &parsed {
        if let Some(possible_translation) = entry.get(&language) {
          if possible_translation.is_none() {
            println!(
              "Translation key missing: {} {} in {}",
              key,
              language,
              path.display()
            );

            result.missing_translations_count += 1;
          }
        } else {
          println!(
            "Translation key missing: {} {} in {}",
            key,
            language,
            path.display()
          );

          result.missing_translations_count += 1;
        }
      }
    }

    result.checked_translations_count = parsed.len() as u32;
    result.duration = started_at.elapsed().as_millis();

    log::info!(
      "Verified file {} in {} sec",
      path.display(),
      (result.duration as f64) / 1000.0
    );

    Ok(result)
  }
}
