use crate::error::translation_error::TranslationError;
use crate::types::{
  TranslationCompiledXml, TranslationEntryCompiled, TranslationJson, TranslationVariant,
};
use crate::{ProjectBuildOptions, TranslationLanguage, TranslationProject};
use quick_xml::se::Serializer;
use serde::Serialize;
use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

impl TranslationProject {
  pub fn build_dir(dir: &Path, options: &ProjectBuildOptions) -> Result<(), TranslationError> {
    if options.is_logging_enabled() {
      println!("Building dir {:?}", dir);
    }

    // Filter all the entries that are not accessed by other files and represent entry points.
    for entry in WalkDir::new(dir) {
      let entry: DirEntry = match entry {
        Ok(entry) => entry,
        Err(error) => return Err(TranslationError::Io(error.into_io_error().unwrap())),
      };

      let entry_path: &Path = entry.path();

      if entry_path.is_file() {
        TranslationProject::build_file(entry_path, options)?
      }
    }

    Ok(())
  }

  pub fn build_file(path: &Path, options: &ProjectBuildOptions) -> Result<(), TranslationError> {
    let extension: Option<&OsStr> = path.extension();

    if let Some(extension) = extension {
      if extension == "xml" {
        if options.is_logging_enabled() {
          println!("Building todo XML based translations {:?}", path);
        }
      } else if extension == "json" {
        if options.is_logging_enabled() {
          println!("Building JSON based translations {:?}", path);
        }

        let parsed: TranslationJson = Self::read_translation_json_by_path(path);

        if options.language == TranslationLanguage::All {
          for language in TranslationLanguage::get_all() {
            Self::prepare_target_json_file(path, &options.output, &language, options)?.write_all(
              TranslationProject::compile_translation_json_by_language(
                &parsed,
                language.as_str(),
                options,
              )
              .as_bytes(),
            )?
          }
        } else {
          Self::prepare_target_json_file(path, &options.output, &options.language, options)?
            .write_all(
              TranslationProject::compile_translation_json_by_language(
                &parsed,
                options.language.as_str(),
                options,
              )
              .as_bytes(),
            )?
        }
      } else {
        // Just warn.
        if !options.is_silent {
          println!("Skip file {:?}", path);
        }
      }
    }

    Ok(())
  }

  fn compile_translation_json_by_language(
    source: &TranslationJson,
    language: &str,
    options: &ProjectBuildOptions,
  ) -> String {
    let mut buffer: String = String::from("<?xml version=\"1.0\" encoding=\"windows-1250\" ?>\n\n");
    let mut serializer: Serializer<String> = Serializer::new(&mut buffer);
    let mut compiled: TranslationCompiledXml = TranslationCompiledXml::default();

    if options.is_verbose_logging_enabled() {
      println!(
        "Building json file with {:?} entries, language '{language}'",
        source.len()
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
