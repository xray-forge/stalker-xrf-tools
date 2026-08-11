use std::ffi::OsStr;
use std::fs::File;
use std::io::{Write, copy};
use std::path::{Display, Path};
use std::time::Instant;

use walkdir::{DirEntry, WalkDir};
use xray_error::{XRayError, XRayResult};
use xray_utils::{XRayEncoding, encode_string_to_bytes};
use xray_xml::serialize_xml;

use crate::types::{TranslationCompiledXml, TranslationEntryCompiled, TranslationJson, TranslationVariant};
use crate::{ProjectBuildOptions, ProjectBuildResult, TranslationLanguage, TranslationProject};

impl TranslationProject {
  pub fn build_dir(dir: &Path, options: &ProjectBuildOptions) -> XRayResult<ProjectBuildResult> {
    log::info!("Building dir {}", dir.display());
    xray_output::info!(options.output, "Building dir {}", dir.display());

    let started_at: Instant = Instant::now();
    let mut result: ProjectBuildResult = ProjectBuildResult::new();

    // Filter all the entries that are not accessed by other files and represent entry points.
    for entry in WalkDir::new(dir) {
      let entry: DirEntry = entry.map_err(|error| XRayError::new_serialization_error(error.to_string()))?;

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

  pub fn build_file<P: AsRef<Path>>(path: &P, options: &ProjectBuildOptions) -> XRayResult<ProjectBuildResult> {
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
        xray_output::info!(options.output, "Skip file {}", path.as_ref().display());
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
      xray_output::info!(options.output, "Building XML based translations {path_display}");

      // All locales needed or file locale matches current one.
      if options.language == TranslationLanguage::All || locale == options.language {
        log::info!("Building dynamic XML file {} ({})", path_display, locale);

        copy(
          &mut File::open(path)?,
          &mut Self::prepare_target_xml_translation_file(path, &options.output_dir, &locale, options)?,
        )?;
      } else {
        log::info!("Skip dynamic XML file {}", path_display);
      }
    } else {
      log::info!("Building static XML file {}", path.as_ref().display());

      // Just plain XML to copy from one place to another.
      xray_output::info!(options.output, "Copy static XML translations {path_display}");

      if options.language == TranslationLanguage::All {
        for language in TranslationLanguage::get_all() {
          copy(
            &mut File::open(path)?,
            &mut Self::prepare_target_xml_translation_file(path, &options.output_dir, &language, options)?,
          )?;
        }
      } else {
        copy(
          &mut File::open(path)?,
          &mut Self::prepare_target_xml_translation_file(path, &options.output_dir, &options.language, options)?,
        )?;
      }
    }

    Ok(())
  }

  pub fn build_json_file<P: AsRef<Path>>(path: &P, options: &ProjectBuildOptions) -> XRayResult {
    xray_output::info!(
      options.output,
      "Building JSON based translations {}",
      path.as_ref().display()
    );

    let parsed: TranslationJson = Self::read_translation_json_by_path(path)?;

    if options.language == TranslationLanguage::All {
      for language in TranslationLanguage::get_all() {
        Self::build_translation_json_by_language(path.as_ref(), &parsed, &language, options)?;
      }
    } else {
      Self::build_translation_json_by_language(path.as_ref(), &parsed, &options.language, options)?;
    }

    Ok(())
  }

  fn build_translation_json_by_language(
    path: &Path,
    source: &TranslationJson,
    language: &TranslationLanguage,
    options: &ProjectBuildOptions,
  ) -> XRayResult {
    let data: Vec<u8> = encode_string_to_bytes(
      &Self::compile_translation_json_by_language(path, source, language, options)?,
      language.get_language_encoder(),
    )?;

    Self::prepare_target_xml_translation_file(&path, &options.output_dir, language, options)?.write_all(&data)?;

    Ok(())
  }

  fn compile_translation_json_by_language(
    path: &Path,
    source: &TranslationJson,
    language: &TranslationLanguage,
    options: &ProjectBuildOptions,
  ) -> XRayResult<String> {
    let mut buffer: String = format!(
      "<?xml version=\"1.0\" encoding=\"{}\" ?>\n\n",
      language.get_language_encoding()
    );
    let mut compiled: TranslationCompiledXml = TranslationCompiledXml::default();

    let language_key: String = language.to_string();

    xray_output::verbose!(
      options.output,
      "Building json file with {} entries, language '{language_key}'",
      source.len(),
    );

    for (key, entry) in source {
      let text: String = entry.get(&language_key).map_or(key.clone(), |value| {
        value
          .as_ref()
          .map_or(key.clone(), Self::compile_translation_entry_by_ref)
      });

      Self::validate_translation_entry_encoding(path, language, key, &text)?;

      compiled.string.push(TranslationEntryCompiled { id: key.clone(), text });
    }

    if options.is_sorted {
      compiled.string.sort_by(|first, second| first.id.cmp(&second.id))
    }

    buffer.push_str(&serialize_xml(&compiled)?);

    Ok(buffer)
  }

  fn validate_translation_entry_encoding(
    path: &Path,
    language: &TranslationLanguage,
    id: &str,
    text: &str,
  ) -> XRayResult {
    for (field, value) in [("id", id), ("text", text)] {
      if let Some(character) = Self::find_unencodable_character(value, language.get_language_encoder()) {
        return Err(XRayError::new_encoding_error(format!(
          "Translation '{}' entry '{}' {} cannot be encoded as {}: '{}' (U+{:04X})",
          path.display(),
          id,
          field,
          language.get_language_encoding(),
          character,
          character as u32,
        )));
      }
    }

    Ok(())
  }

  fn find_unencodable_character(value: &str, encoding: XRayEncoding) -> Option<char> {
    value
      .chars()
      .find(|character| encoding.encode(&String::from(*character)).2)
  }

  fn compile_translation_entry_by_ref(variant: &TranslationVariant) -> String {
    match variant {
      TranslationVariant::String(value) => value.clone(),
      TranslationVariant::MultiString(values) => values.join("\\n"),
    }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use std::path::PathBuf;

  use super::*;

  fn build_options(language: TranslationLanguage) -> ProjectBuildOptions {
    ProjectBuildOptions {
      is_sorted: false,
      path: PathBuf::from("translations"),
      output: xray_output::OutputOptions::default(),
      output_dir: PathBuf::from("output"),
      language,
    }
  }

  #[test]
  fn compiles_windows_1252_translations() {
    let source: TranslationJson = HashMap::from([(
      String::from("st_test"),
      HashMap::from([(
        String::from("fra"),
        Some(TranslationVariant::String(String::from("À bientôt, José !"))),
      )]),
    )]);
    let options: ProjectBuildOptions = build_options(TranslationLanguage::French);

    let compiled: String = TranslationProject::compile_translation_json_by_language(
      Path::new("translations/example.json"),
      &source,
      &TranslationLanguage::French,
      &options,
    )
    .unwrap();

    assert!(compiled.contains("encoding=\"windows-1252\""));
    assert!(encode_string_to_bytes(&compiled, TranslationLanguage::French.get_language_encoder()).is_ok());
  }

  #[test]
  fn reports_unencodable_translation_entries_with_context() {
    let source: TranslationJson = HashMap::from([(
      String::from("st_test"),
      HashMap::from([(String::from("pol"), Some(TranslationVariant::String(String::from("Й"))))]),
    )]);
    let options: ProjectBuildOptions = build_options(TranslationLanguage::Polish);

    let error = TranslationProject::compile_translation_json_by_language(
      Path::new("translations/example.json"),
      &source,
      &TranslationLanguage::Polish,
      &options,
    )
    .unwrap_err();

    assert_eq!(
      error.to_string(),
      "Encoding error: Translation 'translations/example.json' entry 'st_test' text cannot be encoded as windows-1250: 'Й' (U+0419)"
    );
  }
}
