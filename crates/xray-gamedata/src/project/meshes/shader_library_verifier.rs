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
    let path: PathBuf = self.project.get_shader_library_path();

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
