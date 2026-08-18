use std::path::{Path, PathBuf};

use xrf_assets::{XrayVfs, implied_asset_root};
use xrf_visual::VisualSubmesh;

use crate::visuals::textures::submesh_texture::{MISSING_TEXTURE_REFERENCE, SubmeshTexture, SubmeshTextureResolution};

/// Resolves a visual's texture references against the sources the engine would search.
///
/// Owns the VFS, so a user clicking through models in one tree mounts it once. Lives on plugin state for that reason
/// alone - the mounts are the value, not the resolver.
#[derive(Default)]
pub struct VisualTextureResolver {
  vfs: XrayVfs,
}

impl VisualTextureResolver {
  pub fn new() -> Self {
    Self::default()
  }

  /// Every submesh's texture reference, resolved.
  ///
  /// `fallback_root` is the configured project's gamedata path, which only the frontend knows.
  pub fn resolve_submeshes(
    &mut self,
    visual: Option<&Path>,
    fallback_root: Option<&Path>,
    submeshes: &[VisualSubmesh],
  ) -> Vec<SubmeshTexture> {
    let order: Vec<PathBuf> = Self::mount_order(visual, fallback_root);

    submeshes
      .iter()
      .map(|submesh| SubmeshTexture {
        submesh_index: submesh.index,
        reference: submesh.texture_name.clone(),
        resolution: match &submesh.texture_name {
          None => SubmeshTextureResolution::None,
          Some(reference) => self.resolve(&order, reference),
        },
      })
      .collect()
  }

  /// One reference, following the engine's substitution when it resolves nowhere.
  pub fn resolve(&mut self, order: &[PathBuf], reference: &str) -> SubmeshTextureResolution {
    let Some(root) = order.first() else {
      return SubmeshTextureResolution::NoRoot;
    };

    if let Some(location) = self.vfs.dds_texture(order, reference) {
      return SubmeshTextureResolution::Resolved { location };
    }

    match self.vfs.dds_texture(order, MISSING_TEXTURE_REFERENCE) {
      Some(location) => SubmeshTextureResolution::Substituted { location },
      None => SubmeshTextureResolution::Missing { root: root.clone() },
    }
  }

  /// Sources to search for a visual's textures, nearest first.
  ///
  /// The tree holding the visual answers first, since that is the tree its references were authored against. The ambient
  /// project root answers only behind it - for a visual sitting outside any tree, an archived one, or a file dragged in
  /// from anywhere. Nothing is invented when neither applies, which is what an empty order means.
  pub fn mount_order(visual: Option<&Path>, fallback_root: Option<&Path>) -> Vec<PathBuf> {
    let implied: Option<PathBuf> = visual.and_then(implied_asset_root);
    let fallback: Option<PathBuf> = fallback_root.filter(|root| root.is_dir()).map(Path::to_path_buf);

    implied
      .into_iter()
      .chain(fallback)
      .fold(Vec::with_capacity(2), |mut order, root| {
        if !order.contains(&root) {
          order.push(root);
        }

        order
      })
  }
}
