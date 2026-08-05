use crate::GamedataFindingFactory;
use crate::project::meshes::shader_library_verification_result::GamedataShaderLibraryVerificationResult;
use crate::{GamedataProject, GamedataVerificationRule};
use std::path::PathBuf;
use xray_db::ShaderLibraryFile;

pub(crate) struct ShaderLibraryVerifier<'a> {
  project: &'a GamedataProject,
}

impl<'a> ShaderLibraryVerifier<'a> {
  pub(crate) fn new(project: &'a GamedataProject) -> Self {
    Self { project }
  }

  pub(crate) fn verify(&self) -> GamedataShaderLibraryVerificationResult {
    let path: PathBuf = self
      .project
      .assets
      .expected_absolute_path(xray_assets::shader::SHADER_LIBRARY_LOGICAL_PATH)
      .expect("fixed shader library path is valid");

    match ShaderLibraryFile::read_from_path(&path) {
      Ok(library) => GamedataShaderLibraryVerificationResult::passed(library),
      Err(error) => {
        GamedataShaderLibraryVerificationResult::failed(GamedataFindingFactory::for_asset(
          GamedataVerificationRule::MeshesShaderLibrary,
          path,
          format!("Failed to read shader library: {error}"),
        ))
      }
    }
  }
}
