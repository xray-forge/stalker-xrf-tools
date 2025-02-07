use crate::types::{
  TranslationCompiledXml, TranslationEntryCompiled, TranslationJson, TranslationVariant,
};
use crate::{ProjectBuildOptions, ProjectBuildResult, TranslationLanguage, TranslationProject};
use quick_xml::se::Serializer;
use serde::Serialize;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{copy, Write};
use std::path::{Display, Path};
use std::time::Instant;
use walkdir::{DirEntry, WalkDir};
use xray_error::{XRayError, XRayResult};
use xray_utils::encode_string_to_bytes;

impl TranslationProject {
  pub fn build_dir(dir: &Path, options: &ProjectBuildOptions) -> XRayResult<ProjectBuildResult> {
    log::info!("Building dir {}", dir.display());

    if options.is_logging_enabled() {
      println!("Building dir {}", dir.display());
    }

    let started_at: Instant = Instant::now();
    let mut result: ProjectBuildResult = ProjectBuildResult::new();

    // Filter all the entries that are not accessed by other files and represent entry points.
    for entry in WalkDir::new(dir) {
      let entry: DirEntry =
        entry.map_err(|error| XRayError::new_serialization_error(error.to_string()))?;

      if entry.path().is_file() {
        Self::build_file(&entry.path(), options)?;
      }
    }

    result.duration = started_at.elapsed().as_millis();

    log::info!(
      "Built dir {} in {} sec",
      dir.display(),
      (result.duration as f64) / 1_000.0
    );

    Ok(result)
  }

  pub fn build_file<P: AsRef<Path>>(
    path: &P,
    options: &ProjectBuildOptions,
  ) -> XRayResult<ProjectBuildResult> {
    let extension: Option<&OsStr> = path.as_ref().extension();
    let started_at: Instant = Instant::now();

    let mut result: ProjectBuildResult = ProjectBuildResult::new();

    if let Some(extension) = extension {
      if extension == "xml" {
        Self::build_xml_file(path, options)?;
      } else if extension == "json" {
        Self::build_json_file(path, options)?;
      } else {
        log::info!("Skip file {}", path.as_ref().display());

        if options.is_logging_enabled() {
          println!("Skip file {}", path.as_ref().display());
        }
      }
    }

    result.duration = started_at.elapsed().as_millis();

    log::info!(
      "Built file {} in {} sec",
      path.as_ref().display(),
      (result.duration as f64) / 1000.0
    );

    Ok(result)
  }

  pub fn build_xml_file<P: AsRef<Path>>(path: &P, options: &ProjectBuildOptions) -> XRayResult {
    let path_display: Display = path.as_ref().display();
    let locale: Option<TranslationLanguage> = Self::get_locale_from_path(path);

    if let Some(locale) = locale {
      if options.is_logging_enabled() {
        println!("Building XML based translations {}", path_display);
      }

      // All locales needed or file locale matches current one.
      if options.language == TranslationLanguage::All || locale == options.language {
        log::info!("Building dynamic XML file {} ({})", path_display, locale);

        copy(
          &mut File::open(path)?,
          &mut Self::prepare_target_xml_translation_file(path, &options.output, &locale, options)?,
        )?;
      } else {
        log::info!("Skip dynamic XML file {}", path_display);
      }
    } else {
      log::info!("Building static XML file {}", path.as_ref().display());

      // Just plain XML to copy from one place to another.
      if options.is_logging_enabled() {
        println!("Copy static XML translations {}", path_display);
      }

      if options.language == TranslationLanguage::All {
        for language in TranslationLanguage::get_all() {
          copy(
            &mut File::open(path)?,
            &mut Self::prepare_target_xml_translation_file(
              path,
              &options.output,
              &language,
              options,
            )?,
          )?;
        }
      } else {
        copy(
          &mut File::open(path)?,
          &mut Self::prepare_target_xml_translation_file(
            path,
            &options.output,
            &options.language,
            options,
          )?,
        )?;
      }
    }

    Ok(())
  }

  pub fn build_json_file<P: AsRef<Path>>(path: &P, options: &ProjectBuildOptions) -> XRayResult {
    if options.is_logging_enabled() {
      println!(
        "Building JSON based translations {}",
        path.as_ref().display()
      );
    }

    let parsed: TranslationJson = Self::read_translation_json_by_path(path)?;

    if options.language == TranslationLanguage::All {
      for language in TranslationLanguage::get_all() {
        let data: Vec<u8> = encode_string_to_bytes(
          &Self::compile_translation_json_by_language(&parsed, &language, options)?,
          language.get_language_encoder(),
        )?;

        Self::prepare_target_xml_translation_file(path, &options.output, &language, options)?
          .write_all(&data)?;
      }
    } else {
      let data: Vec<u8> = encode_string_to_bytes(
        &Self::compile_translation_json_by_language(&parsed, &options.language, options)?,
        options.language.get_language_encoder(),
      )?;

      Self::prepare_target_xml_translation_file(path, &options.output, &options.language, options)?
        .write_all(&data)?;
    }

    Ok(())
  }

  fn compile_translation_json_by_language(
    source: &TranslationJson,
    language: &TranslationLanguage,
    options: &ProjectBuildOptions,
  ) -> XRayResult<String> {
    let mut buffer: String = format!(
      "<?xml version=\"1.0\" encoding=\"{}\" ?>\n\n",
      language.get_language_encoding()
    );
    let mut serializer: Serializer<String> = Serializer::new(&mut buffer);
    let mut compiled: TranslationCompiledXml = TranslationCompiledXml::default();

    let language: String = language.to_string();

    if options.is_verbose_logging_enabled() {
      println!(
        "Building json file with {} entries, language '{language}'",
        source.len(),
      )
    }

    for (key, entry) in source {
      match entry.get(&language) {
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

    compiled
      .serialize(serializer)
      .map_err(|error| XRayError::new_serialization_error(error.to_string()))?;

    Ok(buffer)
  }

  fn compile_translation_entry_by_ref(variant: &TranslationVariant) -> String {
    match variant {
      TranslationVariant::String(value) => value.clone(),
      TranslationVariant::MultiString(values) => values.join("\\n"),
    }
  }
}
