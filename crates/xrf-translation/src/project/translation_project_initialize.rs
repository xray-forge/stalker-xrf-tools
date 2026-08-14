use std::ffi::OsStr;
use std::path::{Display, Path};
use std::time::Instant;

use walkdir::{DirEntry, WalkDir};
use xrf_error::{XrfError, XrfResult};

use crate::project::staged_write::write_file_staged;
use crate::types::TranslationJson;
use crate::{ProjectInitializeOptions, ProjectInitializeResult, TranslationLanguage, TranslationProject};

impl TranslationProject {
  pub fn initialize_dir<P: AsRef<Path>>(
    dir: &P,
    options: &ProjectInitializeOptions,
  ) -> XrfResult<ProjectInitializeResult> {
    xrf_output::info!(options.output, "Initializing dir {}", dir.as_ref().display());

    let started_at: Instant = Instant::now();
    let mut result: ProjectInitializeResult = ProjectInitializeResult::new();

    // Filter all the entries that are not accessed by other files and represent entry points.
    for entry in WalkDir::new(dir).sort_by_file_name() {
      let entry: DirEntry = entry.map_err(|error| {
        XrfError::new_read_error(format!(
          "Failed to walk translation directory '{}': {error}",
          dir.as_ref().display()
        ))
      })?;

      if entry.path().is_file() {
        Self::initialize_file(&entry.path(), options)?;
      }
    }

    result.duration = started_at.elapsed().as_millis();

    log::info!(
      "Initialize dir {} in {} sec",
      dir.as_ref().display(),
      (result.duration as f64) / 1000.0
    );

    Ok(result)
  }

  pub fn initialize_file<P: AsRef<Path>>(
    path: &P,
    options: &ProjectInitializeOptions,
  ) -> XrfResult<ProjectInitializeResult> {
    let extension: Option<&OsStr> = path.as_ref().extension();

    if let Some(extension) = extension {
      if extension == "json" {
        return Self::initialize_json_file(path, options);
      } else {
        log::info!("Skip file {}", path.as_ref().display());
        xrf_output::info!(options.output, "Skip file {}", path.as_ref().display());
      }
    }

    Ok(ProjectInitializeResult::new())
  }

  pub fn initialize_json_file<P: AsRef<Path>>(
    path: &P,
    options: &ProjectInitializeOptions,
  ) -> XrfResult<ProjectInitializeResult> {
    let path_display: Display = path.as_ref().display();

    let mut result: ProjectInitializeResult = ProjectInitializeResult::new();
    let mut initialized_count: u32 = 0;

    log::info!("Initializing dynamic JSON file {}", path_display);

    let started_at: Instant = Instant::now();
    let mut parsed: TranslationJson = Self::read_translation_json_by_path(path.as_ref())?;

    let all_languages: Vec<String> = TranslationLanguage::get_all_strings();

    for (key, value) in &mut parsed {
      for language in &all_languages {
        match value.get_mut(language) {
          None => {
            initialized_count += 1;

            log::info!("Initializing missing key: {key} - {language}");
            xrf_output::info!(options.output, "Initializing missing key: {key} - {language}");

            value.insert(String::from(language), None);
          }
          _ => {
            // Nothing.
          }
        }

        if !value.contains_key(language) {
          value.insert(String::from(language), None);
        }
      }
    }

    if initialized_count > 0 {
      let serialized = serde_json::to_vec_pretty(&parsed).map_err(|error| {
        XrfError::new_serialization_error(format!(
          "Failed to serialize initialized translation JSON '{}': {error}",
          path_display
        ))
      })?;

      write_file_staged(path.as_ref(), &serialized)?;
    }

    result.duration = started_at.elapsed().as_millis();

    if initialized_count > 0 {
      log::info!(
        "Initialized file {} in {} sec, {} keys added",
        path_display,
        (result.duration as f64) / 1000.0,
        initialized_count
      );
    } else {
      log::info!(
        "Skip file {}, checked in {} sec",
        path_display,
        (result.duration as f64) / 1000.0
      );
    }

    Ok(result)
  }
}

#[cfg(test)]
mod tests {
  use std::io::Write;

  use xrf_error::XrfResult;
  use xrf_test_utils::utils::{get_absolute_generated_test_resource_path, overwrite_generated_test_resource_as_file};

  use crate::{ProjectInitializeOptions, TranslationLanguage, TranslationProject, TranslationVariant};

  #[test]
  fn initialization_replaces_json_only_after_writing_a_valid_document() -> XrfResult {
    let relative_path = "translation_project_initialize/transactional.json";
    let mut file = overwrite_generated_test_resource_as_file(relative_path)?;

    file.write_all(br#"{"st_test":{"eng":"original"}}"#)?;

    drop(file);

    let path = get_absolute_generated_test_resource_path(relative_path);
    let options = ProjectInitializeOptions {
      output: xrf_output::OutputOptions::default(),
      path: path.clone(),
    };

    TranslationProject::initialize_json_file(&path, &options)?;

    let initialized = TranslationProject::read_translation_json_by_path(&path)?;

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
}
