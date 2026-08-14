use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use xrf_error::{XrfError, XrfResult};

use crate::project::translation_project::TranslationProject;
use crate::{ProjectBuildOptions, TranslationLanguage};

impl TranslationProject {
  pub(crate) fn prepare_target_xml_translation_file<P1: AsRef<Path>, P2: AsRef<Path>>(
    path: &P1,
    destination: &P2,
    language: &TranslationLanguage,
    options: &ProjectBuildOptions,
  ) -> XrfResult<File> {
    let target = Self::target_xml_translation_path(path.as_ref(), destination.as_ref(), language, options)?;

    xrf_output::verbose!(options.output, "Writing file ({}) {}", language, target.display());

    let target_parent = target.parent().ok_or_else(|| {
      XrfError::new_invalid_error(format!("Translation XML target has no parent: {}", target.display()))
    })?;

    fs::create_dir_all(target_parent)?;

    Ok(File::options().write(true).create(true).truncate(true).open(target)?)
  }

  pub(crate) fn target_xml_translation_path(
    source: &Path,
    destination: &Path,
    language: &TranslationLanguage,
    options: &ProjectBuildOptions,
  ) -> XrfResult<PathBuf> {
    let relative_source = if source == options.path {
      PathBuf::from(source.file_name().ok_or_else(|| {
        XrfError::new_invalid_error(format!("Translation source has no file name: {}", source.display()))
      })?)
    } else {
      source.strip_prefix(&options.path).map(PathBuf::from).map_err(|_| {
        XrfError::new_invalid_error(format!(
          "Translation source '{}' is outside project root '{}'",
          source.display(),
          options.path.display(),
        ))
      })?
    };

    Ok(
      destination
        .join(language.to_string())
        .join(relative_source)
        .with_extension("xml"),
    )
  }
}
