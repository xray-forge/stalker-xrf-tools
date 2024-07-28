use crate::project::project_verify_result::ProjectVerifyResult;
use crate::types::TranslationJson;
use crate::{ProjectVerifyOptions, TranslationError, TranslationLanguage, TranslationProject};
use std::ffi::OsStr;
use std::path::Path;
use std::time::Instant;
use walkdir::{DirEntry, WalkDir};

impl TranslationProject {
  pub fn verify_dir(
    dir: &Path,
    options: &ProjectVerifyOptions,
  ) -> Result<ProjectVerifyResult, TranslationError> {
    log::info!("Verifying dir {:?}", dir);

    if options.is_logging_enabled() {
      println!("Verifying dir {:?}", dir);
    }

    let started_at: Instant = Instant::now();
    let mut result: ProjectVerifyResult = ProjectVerifyResult::new();

    // Filter all the entries that are not accessed by other files and represent entry points.
    for entry in WalkDir::new(dir) {
      let entry: DirEntry = match entry {
        Ok(entry) => entry,
        Err(error) => return Err(TranslationError::Io(error.into_io_error().unwrap())),
      };

      let entry_path: &Path = entry.path();

      if entry_path.is_file() {
        let file_result: ProjectVerifyResult =
          TranslationProject::verify_file(entry_path, options)?;

        result.missing_translations_count += file_result.missing_translations_count;
        result.checked_translations_count += file_result.checked_translations_count;
      }
    }

    result.duration = started_at.elapsed().as_millis();

    log::info!(
      "Verified dir {:?} in {} sec",
      dir,
      (result.duration as f64) / 1000.0
    );

    Ok(result)
  }

  pub fn verify_file(
    path: &Path,
    options: &ProjectVerifyOptions,
  ) -> Result<ProjectVerifyResult, TranslationError> {
    let extension: Option<&OsStr> = path.extension();

    if let Some(extension) = extension {
      if extension == "json" {
        return Self::verify_json_file(path, options);
      } else {
        log::info!("Skip file {:?}", path);

        if options.is_logging_enabled() {
          println!("Skip file {:?}", path);
        }
      }
    }

    Ok(ProjectVerifyResult::new())
  }

  pub fn verify_json_file(
    path: &Path,
    options: &ProjectVerifyOptions,
  ) -> Result<ProjectVerifyResult, TranslationError> {
    let mut result: ProjectVerifyResult = ProjectVerifyResult::new();

    log::info!("Verifying dynamic JSON file {:?}", path);

    let started_at: Instant = Instant::now();
    let parsed: TranslationJson = Self::read_translation_json_by_path(path)?;

    let languages: Vec<&str> = if options.language == TranslationLanguage::All {
      TranslationLanguage::get_all_str()
    } else {
      vec![options.language.as_str()]
    };

    for language in languages {
      for (key, entry) in &parsed {
        if let Some(possible_translation) = entry.get(language) {
          if possible_translation.is_none() {
            println!(
              "Translation key missing: {:?} {:?} in {:?}",
              key, language, path
            );

            result.missing_translations_count += 1;
          }
        } else {
          println!(
            "Translation key missing: {:?} {:?} in {:?}",
            key, language, path
          );

          result.missing_translations_count += 1;
        }
      }
    }

    result.checked_translations_count = parsed.len() as u32;
    result.duration = started_at.elapsed().as_millis();

    log::info!(
      "Verified file {:?} in {} sec",
      path,
      (result.duration as f64) / 1000.0
    );

    Ok(result)
  }
}
