use std::fs;
use std::path::PathBuf;

use crate::GamedataProject;
use crate::project::levels::level_engine_constants::LEVELS_DIRECTORY;

/// One level bundle directory, addressed the way the engine addresses `$level$`.
///
/// Rules receive a bundle rather than a name so that finding subjects, on-disk lookups and
/// level-local asset resolution all derive from one place.
pub(crate) struct LevelBundle<'a> {
  project: &'a GamedataProject,
  name: &'a str,
}

impl<'a> LevelBundle<'a> {
  pub(crate) fn new(project: &'a GamedataProject, name: &'a str) -> Self {
    Self { project, name }
  }

  pub(crate) fn name(&self) -> &str {
    self.name
  }

  pub(crate) fn project(&self) -> &'a GamedataProject {
    self.project
  }

  /// Logical path of the bundle directory, used as a finding subject.
  pub(crate) fn path(&self) -> String {
    Self::path_of(self.name)
  }

  /// Logical path of a file inside the bundle, used as a finding subject.
  pub(crate) fn file_path(&self, file: &str) -> String {
    format!("{LEVELS_DIRECTORY}\\{}\\{file}", self.name)
  }

  /// Logical path of a bundle directory that does not need to exist.
  pub(crate) fn path_of(name: &str) -> String {
    format!("{LEVELS_DIRECTORY}\\{name}")
  }

  pub(crate) fn file_on_disk(&self, file: &str) -> Option<PathBuf> {
    self
      .project
      .assets
      .absolute_path(&self.file_path(file))
      .ok()
      .flatten()
  }

  pub(crate) fn file_size(&self, file: &str) -> Option<u64> {
    self
      .file_on_disk(file)
      .and_then(|path| fs::metadata(path).ok())
      .map(|metadata| metadata.len())
  }

  pub(crate) fn contains(&self, file: &str) -> bool {
    self.file_size(file).is_some()
  }

  /// Resolve a texture reference the way the renderer does.
  ///
  /// `CRender::texture_load` probes `$level$` before `$game_textures$`, so lightmaps and terrain
  /// atlases shipped inside the bundle resolve even though they are absent from the shared texture
  /// tree. `$game_saves$` is probed in between, but it is not part of gamedata.
  pub(crate) fn resolves_texture(&self, reference: &str) -> bool {
    let logical: String = xray_assets::texture::dds_logical_path(reference);

    let in_bundle: bool = self
      .project
      .assets
      .find_in(&self.path(), &logical)
      .ok()
      .flatten()
      .is_some();

    in_bundle
      || self
        .project
        .assets
        .dds_texture(reference)
        .ok()
        .flatten()
        .is_some()
  }
}
