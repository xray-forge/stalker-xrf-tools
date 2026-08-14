use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{Write, copy};
use std::path::{Component, Display, Path, PathBuf};
use std::time::Instant;

use walkdir::{DirEntry, WalkDir};
use xrf_error::{XrfError, XrfResult};
use xrf_utils::{XRayEncoding, encode_string_to_bytes};
use xrf_xml::serialize_xml;

use crate::types::{TranslationCompiledXml, TranslationEntryCompiled, TranslationJson, TranslationVariant};
use crate::{ProjectBuildOptions, ProjectBuildResult, TranslationLanguage, TranslationProject};

impl TranslationProject {
  pub fn build_dir(dir: &Path, options: &ProjectBuildOptions) -> XrfResult<ProjectBuildResult> {
    log::info!("Building dir {}", dir.display());
    xrf_output::info!(options.output, "Building dir {}", dir.display());

    let started_at: Instant = Instant::now();
    let mut result: ProjectBuildResult = ProjectBuildResult::new();
    let mut source_files: Vec<PathBuf> = Vec::new();

    Self::ensure_output_outside_source(dir, &options.output_dir)?;

    // Filter all the entries that are not accessed by other files and represent entry points.
    for entry in WalkDir::new(dir).sort_by_file_name() {
      let entry: DirEntry = entry.map_err(|error| {
        XrfError::new_read_error(format!(
          "Failed to walk translation directory '{}': {error}",
          dir.display()
        ))
      })?;

      if entry.path().is_file() {
        source_files.push(entry.into_path());
      }
    }

    Self::validate_build_targets(&source_files, options)?;

    for source_file in source_files {
      Self::build_file(&source_file, options)?;
    }

    result.duration = started_at.elapsed().as_millis();

    log::info!(
      "Built dir {} in {} sec",
      dir.display(),
      (result.duration as f64) / 1_000.0
    );

    Ok(result)
  }

  pub fn build_file<P: AsRef<Path>>(path: &P, options: &ProjectBuildOptions) -> XrfResult<ProjectBuildResult> {
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
        xrf_output::info!(options.output, "Skip file {}", path.as_ref().display());
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

  pub fn build_xml_file<P: AsRef<Path>>(path: &P, options: &ProjectBuildOptions) -> XrfResult {
    let path_display: Display = path.as_ref().display();
    let locale: Option<TranslationLanguage> = Self::get_locale_from_path(path.as_ref());

    if let Some(locale) = locale {
      xrf_output::info!(options.output, "Building XML based translations {path_display}");

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
      xrf_output::info!(options.output, "Copy static XML translations {path_display}");

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

  pub fn build_json_file<P: AsRef<Path>>(path: &P, options: &ProjectBuildOptions) -> XrfResult {
    xrf_output::info!(
      options.output,
      "Building JSON based translations {}",
      path.as_ref().display()
    );

    let parsed: TranslationJson = Self::read_translation_json_by_path(path.as_ref())?;

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
  ) -> XrfResult {
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
  ) -> XrfResult<String> {
    let mut buffer: String = format!(
      "<?xml version=\"1.0\" encoding=\"{}\" ?>\n\n",
      language.get_language_encoding()
    );
    let mut compiled: TranslationCompiledXml = TranslationCompiledXml::default();

    let language_key: String = language.to_string();

    xrf_output::verbose!(
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
  ) -> XrfResult {
    for (field, value) in [("id", id), ("text", text)] {
      if let Some(character) = Self::find_unencodable_character(value, language.get_language_encoder()) {
        return Err(XrfError::new_encoding_error(format!(
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

  fn validate_build_targets(source_files: &[PathBuf], options: &ProjectBuildOptions) -> XrfResult {
    let mut target_sources: HashMap<String, &Path> = HashMap::new();

    for source in source_files {
      for language in Self::target_languages_for_source(source, options) {
        let target = Self::target_xml_translation_path(source, &options.output_dir, &language, options)?;
        let target_key = target.to_string_lossy().replace('\\', "/").to_lowercase();

        if let Some(existing_source) = target_sources.insert(target_key, source)
          && existing_source != source
        {
          return Err(XrfError::new_invalid_error(format!(
            "Translation sources '{}' and '{}' both build to '{}'",
            existing_source.display(),
            source.display(),
            target.display(),
          )));
        }
      }
    }

    Ok(())
  }

  fn target_languages_for_source(path: &Path, options: &ProjectBuildOptions) -> Vec<TranslationLanguage> {
    match path.extension().and_then(OsStr::to_str) {
      Some("json") => {
        if options.language == TranslationLanguage::All {
          TranslationLanguage::get_all()
        } else {
          vec![options.language]
        }
      }
      Some("xml") => match Self::get_locale_from_path(path) {
        Some(locale) if options.language == TranslationLanguage::All || options.language == locale => vec![locale],
        Some(_) => Vec::new(),
        None if options.language == TranslationLanguage::All => TranslationLanguage::get_all(),
        None => vec![options.language],
      },
      _ => Vec::new(),
    }
  }

  fn ensure_output_outside_source(source: &Path, output: &Path) -> XrfResult {
    let source_lexical = Self::normalize_absolute_path(source)?;
    let output_lexical = Self::normalize_absolute_path(output)?;
    let source_resolved = fs::canonicalize(source).unwrap_or_else(|_| source_lexical.clone());
    let output_resolved = fs::canonicalize(output).unwrap_or_else(|_| output_lexical.clone());

    if Self::path_is_within(&output_lexical, &source_lexical)
      || Self::path_is_within(&output_resolved, &source_resolved)
    {
      return Err(XrfError::new_invalid_error(format!(
        "Translation output '{}' must be outside source directory '{}'",
        output.display(),
        source.display(),
      )));
    }

    Ok(())
  }

  fn normalize_absolute_path(path: &Path) -> XrfResult<PathBuf> {
    let absolute = if path.is_absolute() {
      path.to_path_buf()
    } else {
      std::env::current_dir()?.join(path)
    };
    let mut normalized = PathBuf::new();

    for component in absolute.components() {
      match component {
        Component::CurDir => {}
        Component::ParentDir => {
          normalized.pop();
        }
        _ => normalized.push(component.as_os_str()),
      }
    }

    Ok(normalized)
  }

  fn path_is_within(path: &Path, parent: &Path) -> bool {
    let path_components: Vec<String> = path
      .components()
      .map(|component| component.as_os_str().to_string_lossy().to_lowercase())
      .collect();
    let parent_components: Vec<String> = parent
      .components()
      .map(|component| component.as_os_str().to_string_lossy().to_lowercase())
      .collect();

    path_components.len() >= parent_components.len()
      && path_components
        .iter()
        .zip(parent_components.iter())
        .all(|(path_component, parent_component)| path_component == parent_component)
  }
}

#[cfg(test)]
mod tests {
  use std::fs;
  use std::io::Write as _;
  use std::path::PathBuf;

  use indexmap::IndexMap;
  use xrf_test_utils::utils::{get_absolute_generated_test_resource_path, overwrite_generated_test_resource_as_file};

  use super::*;

  fn build_options(language: TranslationLanguage) -> ProjectBuildOptions {
    ProjectBuildOptions {
      is_sorted: false,
      path: PathBuf::from("translations"),
      output: xrf_output::OutputOptions::default(),
      output_dir: PathBuf::from("output"),
      language,
    }
  }

  #[test]
  fn compiles_windows_1252_translations() {
    let source: TranslationJson = IndexMap::from([(
      String::from("st_test"),
      IndexMap::from([(
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
    let source: TranslationJson = IndexMap::from([(
      String::from("st_test"),
      IndexMap::from([(String::from("pol"), Some(TranslationVariant::String(String::from("Й"))))]),
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

  #[test]
  fn preserves_source_order_unless_sorting_is_requested() {
    let source = TranslationJson::from([
      (
        String::from("st_second"),
        crate::TranslationEntry::from([(
          String::from("eng"),
          Some(TranslationVariant::String(String::from("second"))),
        )]),
      ),
      (
        String::from("st_first"),
        crate::TranslationEntry::from([(
          String::from("eng"),
          Some(TranslationVariant::String(String::from("first"))),
        )]),
      ),
    ]);
    let mut options = build_options(TranslationLanguage::English);

    let source_order = TranslationProject::compile_translation_json_by_language(
      Path::new("translations/example.json"),
      &source,
      &TranslationLanguage::English,
      &options,
    )
    .unwrap();

    options.is_sorted = true;
    let sorted = TranslationProject::compile_translation_json_by_language(
      Path::new("translations/example.json"),
      &source,
      &TranslationLanguage::English,
      &options,
    )
    .unwrap();

    assert!(source_order.find("st_second").unwrap() < source_order.find("st_first").unwrap());
    assert!(sorted.find("st_first").unwrap() < sorted.find("st_second").unwrap());
  }

  #[test]
  fn directory_builds_preserve_relative_source_paths() -> XrfResult {
    let test_root = "translation_project_build/relative_paths";
    let source_root = get_absolute_generated_test_resource_path(&format!("{test_root}/source"));
    let output_root = get_absolute_generated_test_resource_path(&format!("{test_root}/output"));

    if output_root.exists() {
      fs::remove_dir_all(&output_root)?;
    }

    for relative_path in ["one/common.json", "two/common.json"] {
      let mut file = overwrite_generated_test_resource_as_file(&format!("{test_root}/source/{relative_path}"))?;
      file.write_all(br#"{"st_test":{"eng":"text"}}"#)?;
    }

    let options = ProjectBuildOptions {
      path: source_root.clone(),
      output_dir: output_root.clone(),
      ..build_options(TranslationLanguage::English)
    };

    TranslationProject::build_dir(&source_root, &options)?;

    assert!(output_root.join("eng/one/common.xml").is_file());
    assert!(output_root.join("eng/two/common.xml").is_file());

    Ok(())
  }

  #[test]
  fn directory_builds_reject_colliding_json_and_xml_targets() -> XrfResult {
    let test_root = "translation_project_build/colliding_targets";
    let source_root = get_absolute_generated_test_resource_path(&format!("{test_root}/source"));
    let output_root = get_absolute_generated_test_resource_path(&format!("{test_root}/output"));

    let mut json = overwrite_generated_test_resource_as_file(&format!("{test_root}/source/common.json"))?;
    json.write_all(br#"{"st_test":{"eng":"text"}}"#)?;

    let mut xml = overwrite_generated_test_resource_as_file(&format!("{test_root}/source/common.xml"))?;
    xml.write_all(br#"<string_table></string_table>"#)?;

    let options = ProjectBuildOptions {
      path: source_root.clone(),
      output_dir: output_root,
      ..build_options(TranslationLanguage::English)
    };

    let error = TranslationProject::build_dir(&source_root, &options).unwrap_err();

    assert!(error.to_string().contains("both build to"));

    Ok(())
  }

  #[test]
  fn directory_builds_reject_output_inside_the_source_tree() -> XrfResult {
    let test_root = "translation_project_build/output_inside_source";
    let source_root = get_absolute_generated_test_resource_path(&format!("{test_root}/source"));
    let mut json = overwrite_generated_test_resource_as_file(&format!("{test_root}/source/common.json"))?;

    json.write_all(br#"{"st_test":{"eng":"text"}}"#)?;

    let options = ProjectBuildOptions {
      path: source_root.clone(),
      output_dir: source_root.join("built"),
      ..build_options(TranslationLanguage::English)
    };

    let error = TranslationProject::build_dir(&source_root, &options).unwrap_err();

    assert!(error.to_string().contains("must be outside source directory"));

    Ok(())
  }
}
