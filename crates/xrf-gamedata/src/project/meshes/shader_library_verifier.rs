use xrf_db::ShaderLibraryFile;

use crate::GamedataFindingFactory;
use crate::project::meshes::shader_library_verification_result::GamedataShaderLibraryVerificationResult;
use crate::{GamedataProject, GamedataVerificationRule};

pub(crate) struct ShaderLibraryVerifier<'a> {
  project: &'a GamedataProject,
}

impl<'a> ShaderLibraryVerifier<'a> {
  pub(crate) fn new(project: &'a GamedataProject) -> Self {
    Self { project }
  }

  pub(crate) fn verify(&self) -> GamedataShaderLibraryVerificationResult {
    // The logical path is the identity to report, since an archived library has no file to name.
    let path: &str = xrf_vfs::SHADER_LIBRARY_LOGICAL_PATH;

    match self
      .project
      .read_asset_chunks(path)
      .and_then(|mut chunks| ShaderLibraryFile::read_from_chunk(&mut chunks))
    {
      Ok(library) => GamedataShaderLibraryVerificationResult::passed(library),
      Err(error) => GamedataShaderLibraryVerificationResult::failed(GamedataFindingFactory::for_asset(
        GamedataVerificationRule::MeshesShaderLibrary,
        path,
        format!("Failed to read shader library: {error}"),
      )),
    }
  }
}
