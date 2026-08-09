use std::ffi::OsStr;
use std::path::{Display, Path};
use std::time::Instant;

use walkdir::{DirEntry, WalkDir};
use xray_error::XRayResult;

use crate::project::translation_project_verify_result::ProjectVerifyResult;
use crate::types::TranslationJson;
use crate::{ProjectVerifyOptions, TranslationLanguage, TranslationProject};

impl TranslationProject {
  pub fn verify_dir(dir: &Path, options: &ProjectVerifyOptions) -> XRayResult<ProjectVerifyResult> {
    log::info!("Verifying dir {}", dir.display());
    xray_output::info!(options.output, "Verifying dir {}", dir.display());

    let started_at: Instant = Instant::now();
    let mut result: ProjectVerifyResult = ProjectVerifyResult::new();

    // Filter all the entries that are not accessed by other files and represent entry points.
    for entry in WalkDir::new(dir) {
      let entry: DirEntry = match entry {
        Ok(entry) => entry,
        Err(error) => {
          return Err(
            error
              .into_io_error()
              .expect("WalkDir error transformation")
              .into(),
          );
        }
      };

      let entry_path: &Path = entry.path();

      if entry_path.is_file() {
        let file_result: ProjectVerifyResult = Self::verify_file(&entry_path, options)?;

        result.merge(file_result);
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

  pub fn verify_file<P: AsRef<Path>>(
    path: &P,
    options: &ProjectVerifyOptions,
  ) -> XRayResult<ProjectVerifyResult> {
    let extension: Option<&OsStr> = path.as_ref().extension();

    if let Some(extension) = extension {
      if extension == "json" {
        return Self::verify_json_file(path, options);
      } else {
        log::info!("Skip file {}", path.as_ref().display());
        xray_output::info!(options.output, "Skip file {}", path.as_ref().display());
      }
    }

    Ok(ProjectVerifyResult::new())
  }

  pub fn verify_json_file<P: AsRef<Path>>(
    path: &P,
    options: &ProjectVerifyOptions,
  ) -> XRayResult<ProjectVerifyResult> {
    let path_display: Display = path.as_ref().display();
    let mut result: ProjectVerifyResult = ProjectVerifyResult::new();

    log::info!("Verifying dynamic JSON file {}", path_display);

    let started_at: Instant = Instant::now();
    let parsed: TranslationJson = Self::read_translation_json_by_path(path)?;

    let languages: Vec<String> = if options.language == TranslationLanguage::All {
      TranslationLanguage::get_all_strings()
    } else {
      vec![options.language.to_string()]
    };

    for language in languages {
      for (key, entry) in &parsed {
        let is_missing: bool = entry
          .get(&language)
          .is_none_or(|translation| translation.is_none());

        if is_missing {
          xray_output::error!(
            options.output,
            "Translation key missing: {} {} in {}",
            key,
            language,
            path_display
          );

          result.record_missing_translation(path.as_ref(), key, &language);
        }
      }
    }

    result.checked_translations_count = parsed.len() as u32;
    result.duration = started_at.elapsed().as_millis();

    log::info!(
      "Verified file {} in {} sec",
      path_display,
      (result.duration as f64) / 1000.0
    );

    Ok(result)
  }
}
