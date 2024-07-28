use crate::error::translation_error::TranslationError;
use crate::types::{
  TranslationCompiledXml, TranslationEntryCompiled, TranslationJson, TranslationVariant,
};
use crate::{ProjectBuildOptions, ProjectBuildResult, TranslationLanguage, TranslationProject};
use quick_xml::se::Serializer;
use serde::Serialize;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{copy, Write};
use std::path::Path;
use std::time::Instant;
use walkdir::{DirEntry, WalkDir};

impl TranslationProject {
  pub fn build_dir(
    dir: &Path,
    options: &ProjectBuildOptions,
  ) -> Result<ProjectBuildResult, TranslationError> {
    log::info!("Building dir {:?}", dir);

    if options.is_logging_enabled() {
      println!("Building dir {:?}", dir);
    }

    let started_at: Instant = Instant::now();
    let mut result: ProjectBuildResult = ProjectBuildResult::new();

    // Filter all the entries that are not accessed by other files and represent entry points.
    for entry in WalkDir::new(dir) {
      let entry: DirEntry = match entry {
        Ok(entry) => entry,
        Err(error) => return Err(TranslationError::Io(error.into_io_error().unwrap())),
      };

      let entry_path: &Path = entry.path();

      if entry_path.is_file() {
        TranslationProject::build_file(entry_path, options)?;
      }
    }

    result.duration = started_at.elapsed().as_millis();

    log::info!(
      "Built dir {:?} in {} sec",
      dir,
      (result.duration as f64) / 1000.0
    );

    Ok(result)
  }

  pub fn build_file(
    path: &Path,
    options: &ProjectBuildOptions,
  ) -> Result<ProjectBuildResult, TranslationError> {
    let extension: Option<&OsStr> = path.extension();
    let started_at: Instant = Instant::now();

    let mut result: ProjectBuildResult = ProjectBuildResult::new();

    if let Some(extension) = extension {
      if extension == "xml" {
        Self::build_xml_file(path, options)?;
      } else if extension == "json" {
        Self::build_json_file(path, options)?;
      } else {
        log::info!("Skip file {:?}", path);

        if options.is_logging_enabled() {
          println!("Skip file {:?}", path);
        }
      }
    }

    result.duration = started_at.elapsed().as_millis();

    log::info!(
      "Built file {:?} in {} sec",
      path,
      (result.duration as f64) / 1000.0
    );

    Ok(result)
  }

  pub fn build_xml_file(
    path: &Path,
    options: &ProjectBuildOptions,
  ) -> Result<(), TranslationError> {
    let locale = Self::get_locale_from_path(path);

    if let Some(locale) = locale {
      if options.is_logging_enabled() {
        println!("Building XML based translations {:?}", path);
      }

      // All locales needed or file locale matches current one.
      if options.language == TranslationLanguage::All || locale == options.language {
        log::info!("Building dynamic XML file {:?} ({:?})", path, locale);

        copy(
          &mut File::open(path)?,
          &mut Self::prepare_target_file(path, &options.output, &locale, options)?,
        )?;
      } else {
        log::info!("Skip dynamic XML file {:?}", path);
      }
    } else {
      log::info!("Building static XML file {:?}", path);

      // Just plain XML to copy from one place to another.
      if options.is_logging_enabled() {
        println!("Copy static XML translations {:?}", path);
      }

      if options.language == TranslationLanguage::All {
        for language in TranslationLanguage::get_all() {
          copy(
            &mut File::open(path)?,
            &mut Self::prepare_target_file(path, &options.output, &language, options)?,
          )?;
        }
      } else {
        copy(
          &mut File::open(path)?,
          &mut Self::prepare_target_file(path, &options.output, &options.language, options)?,
        )?;
      }
    }

    Ok(())
  }

  pub fn build_json_file(
    path: &Path,
    options: &ProjectBuildOptions,
  ) -> Result<(), TranslationError> {
    log::info!("Building dynamic JSON file {:?}", path);

    if options.is_logging_enabled() {
      println!("Building JSON based translations {:?}", path);
    }

    let parsed: TranslationJson = Self::read_translation_json_by_path(path)?;

    if options.language == TranslationLanguage::All {
      for language in TranslationLanguage::get_all() {
        Self::prepare_target_file(path, &options.output, &language, options)?.write_all(
          TranslationProject::compile_translation_json_by_language(&parsed, &language, options)
            .as_bytes(),
        )?;
      }
    } else {
      Self::prepare_target_file(path, &options.output, &options.language, options)?.write_all(
        TranslationProject::compile_translation_json_by_language(
          &parsed,
          &options.language,
          options,
        )
        .as_bytes(),
      )?;
    }

    Ok(())
  }

  fn compile_translation_json_by_language(
    source: &TranslationJson,
    language: &TranslationLanguage,
    options: &ProjectBuildOptions,
  ) -> String {
    let mut buffer: String = format!(
      "<?xml version=\"1.0\" encoding=\"{}\" ?>\n\n",
      language.get_language_encoding()
    );
    let mut serializer: Serializer<String> = Serializer::new(&mut buffer);
    let mut compiled: TranslationCompiledXml = TranslationCompiledXml::default();

    let language: &str = language.as_str();

    if options.is_verbose_logging_enabled() {
      println!(
        "Building json file with {} entries, language '{language}'",
        source.len(),
      )
    }

    for (key, entry) in source {
      match entry.get(language) {
        None => {
          compiled.string.push(TranslationEntryCompiled {
            id: key.clone(),
            text: key.clone(),
          });
        }
        Some(value) => compiled.string.push(TranslationEntryCompiled {
          id: key.clone(),
          text: value
            .as_ref()
            .map_or(key.clone(), Self::compile_translation_entry_by_ref),
        }),
      }
    }

    if options.is_sorted {
      compiled
        .string
        .sort_by(|first, second| first.id.cmp(&second.id))
    }

    serializer.expand_empty_elements(true);
    serializer.indent(' ', 2);

    compiled.serialize(serializer).unwrap();

    buffer
  }

  fn compile_translation_entry_by_ref(variant: &TranslationVariant) -> String {
    match variant {
      TranslationVariant::String(value) => value.clone(),
      TranslationVariant::MultiString(values) => values.join("\\n"),
    }
  }
}
