use crate::project::project::TranslationProject;
use std::fs;
use std::fs::File;
use std::io::ErrorKind::AlreadyExists;

use crate::{ProjectBuildOptions, TranslationError, TranslationLanguage};
use std::path::{Path, PathBuf};

impl TranslationProject {
  pub fn prepare_target_file(
    path: &Path,
    destination: &Path,
    language: &TranslationLanguage,
    options: &ProjectBuildOptions,
  ) -> Result<File, TranslationError> {
    let target: PathBuf = destination
      .join(language.as_str())
      .join(path.file_name().unwrap())
      .with_extension("xml");

    if !options.is_silent && options.is_verbose {
      println!("Writing file ({:?}) {:?}", language, target);
    }

    match fs::create_dir_all(target.parent().unwrap()) {
      Ok(_) => {}
      Err(error) if error.kind() == AlreadyExists => {}
      Err(error) => return Err(error.into()),
    }

    Ok(
      File::options()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(target)
        .expect("File can not be opened for writing"),
    )
  }
}
