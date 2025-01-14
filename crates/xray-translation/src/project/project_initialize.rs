use crate::types::TranslationJson;
use crate::{
  ProjectInitializeOptions, ProjectInitializeResult, TranslationError, TranslationLanguage,
  TranslationProject, TranslationResult,
};
use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use std::time::Instant;
use walkdir::{DirEntry, WalkDir};

impl TranslationProject {
  pub fn initialize_dir(
    dir: &Path,
    options: &ProjectInitializeOptions,
  ) -> TranslationResult<ProjectInitializeResult> {
    log::info!("Initializing dir {:?}", dir);

    if options.is_logging_enabled() {
      println!("Initializing dir {:?}", dir);
    }

    let started_at: Instant = Instant::now();
    let mut result: ProjectInitializeResult = ProjectInitializeResult::new();

    // Filter all the entries that are not accessed by other files and represent entry points.
    for entry in WalkDir::new(dir) {
      let entry: DirEntry = match entry {
        Ok(entry) => entry,
        Err(error) => return Err(TranslationError::Io(error.into_io_error().unwrap())),
      };

      let entry_path: &Path = entry.path();

      if entry_path.is_file() {
        Self::initialize_file(entry_path, options)?;
      }
    }

    result.duration = started_at.elapsed().as_millis();

    log::info!(
      "Initialize dir {:?} in {} sec",
      dir,
      (result.duration as f64) / 1000.0
    );

    Ok(result)
  }

  pub fn initialize_file(
    path: &Path,
    options: &ProjectInitializeOptions,
  ) -> TranslationResult<ProjectInitializeResult> {
    let extension: Option<&OsStr> = path.extension();

    if let Some(extension) = extension {
      if extension == "json" {
        return Self::initialize_json_file(path, options);
      } else {
        log::info!("Skip file {:?}", path);

        if options.is_logging_enabled() {
          println!("Skip file {:?}", path);
        }
      }
    }

    Ok(ProjectInitializeResult::new())
  }

  pub fn initialize_json_file(
    path: &Path,
    options: &ProjectInitializeOptions,
  ) -> TranslationResult<ProjectInitializeResult> {
    let mut result: ProjectInitializeResult = ProjectInitializeResult::new();
    let mut initialized_count: u32 = 0;

    log::info!("Initializing dynamic JSON file {:?}", path);

    let started_at: Instant = Instant::now();
    let mut parsed: TranslationJson = Self::read_translation_json_by_path(path)?;

    let all_languages: Vec<&str> = TranslationLanguage::get_all_str();

    for (key, value) in &mut parsed {
      for language in &all_languages {
        match value.get_mut(*language) {
          None => {
            initialized_count += 1;

            log::info!("Initializing missing key: {:?} - {:?}", key, language);

            if options.is_logging_enabled() {
              println!("Initializing missing key: {:?} - {:?}", key, language);
            }

            value.insert(String::from(*language), None);
          }
          _ => {
            // Nothing.
          }
        }

        if !value.contains_key(*language) {
          value.insert(String::from(*language), None);
        }
      }
    }

    if initialized_count > 0 {
      Self::prepare_target_json_translation_file(path)?.write_all(
        serde_json::to_string_pretty(&Self::transform_translation_into_value(&parsed))
          .expect("valid serializable JSON during init")
          .as_bytes(),
      )?;
    }

    result.duration = started_at.elapsed().as_millis();

    if initialized_count > 0 {
      log::info!(
        "Initialized file {:?} in {} sec, {initialized_count} keys added",
        path,
        (result.duration as f64) / 1000.0
      );
    } else {
      log::info!(
        "Skip file {:?}, checked in {} sec",
        path,
        (result.duration as f64) / 1000.0
      );
    }

    Ok(result)
  }
}
