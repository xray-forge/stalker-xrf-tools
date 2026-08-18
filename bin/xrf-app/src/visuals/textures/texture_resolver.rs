use std::path::{Path, PathBuf};

use xrf_assets::{XrayMountId, XrayScope, XrayVfs, implied_asset_root};
use xrf_visual::VisualSubmesh;

use crate::types::TauriResult;
use crate::visuals::textures::submesh_texture::{MISSING_TEXTURE_REFERENCE, SubmeshTexture, SubmeshTextureResolution};

/// Resolves visual textures while retaining mounted asset sources between requests.
#[derive(Default)]
pub struct VisualTextureResolver {
  vfs: XrayVfs,
}

impl VisualTextureResolver {
  pub fn new() -> Self {
    Self::default()
  }

  /// Resolves each submesh texture against the visual's implied root and optional fallback root.
  ///
  /// The implied root has priority over `fallback_root`.
  pub fn resolve_submeshes(
    &mut self,
    visual: Option<&Path>,
    fallback_root: Option<&Path>,
    submeshes: &[VisualSubmesh],
  ) -> Vec<SubmeshTexture> {
    let scope: XrayScope = self.scope_for(visual, fallback_root);

    submeshes
      .iter()
      .map(|submesh| SubmeshTexture {
        submesh_index: submesh.index,
        reference: submesh.texture_name.clone(),
        resolution: match &submesh.texture_name {
          None => SubmeshTextureResolution::None,
          Some(reference) => self.resolve(&scope, reference),
        },
      })
      .collect()
  }

  /// Resolves one reference and applies the engine's missing-texture substitution.
  pub fn resolve(&self, scope: &XrayScope, reference: &str) -> SubmeshTextureResolution {
    if self.mounts_in(scope) == 0 {
      return SubmeshTextureResolution::NoRoot;
    }

    if let Some(location) = self.lookup(scope, reference) {
      return SubmeshTextureResolution::Resolved { location };
    }

    match self.lookup(scope, MISSING_TEXTURE_REFERENCE) {
      Some(location) => SubmeshTextureResolution::Substituted { location },
      None => SubmeshTextureResolution::Missing {
        roots: self.described_mounts(scope),
      },
    }
  }

  /// Reads a texture from the winning mount.
  ///
  /// Reading through the VFS supports both loose and archived textures.
  pub fn read(&self, scope: &XrayScope, reference: &str) -> TauriResult<Vec<u8>> {
    let logical_path: String = xrf_assets::texture::dds_logical_path(reference);

    self
      .vfs
      .read(scope, &format!("textures\\{logical_path}"))
      .map_err(|error| format!("Failed to read texture '{reference}': {error}"))
  }

  /// Mounts the available texture roots and returns their search scope.
  ///
  /// The visual's implied root is searched before the fallback root. Non-directory fallbacks and roots that fail to mount
  /// are omitted; an empty scope means no lookup can be attempted.
  pub fn scope_for(&mut self, visual: Option<&Path>, fallback_root: Option<&Path>) -> XrayScope {
    let implied: Option<PathBuf> = visual.and_then(implied_asset_root);
    let fallback: Option<PathBuf> = fallback_root.filter(|root| root.is_dir()).map(Path::to_path_buf);

    let mounts: Vec<XrayMountId> = implied
      .into_iter()
      .chain(fallback)
      .filter_map(|root| {
        self
          .vfs
          .mount_directory("", &root)
          .inspect_err(|error| log::warn!("Failed to mount root {}: {error}", root.display()))
          .ok()
      })
      .collect();

    XrayScope::only(mounts)
  }

  fn lookup(&self, scope: &XrayScope, reference: &str) -> Option<xrf_assets::XrayAssetLocation> {
    self
      .vfs
      .dds_texture(scope, reference)
      .inspect_err(|error| log::warn!("Rejected texture reference '{reference}': {error}"))
      .ok()
      .flatten()
  }

  fn mounts_in(&self, scope: &XrayScope) -> usize {
    self.vfs.scoped(scope).count()
  }

  /// Describes every mount searched by a failed lookup.
  fn described_mounts(&self, scope: &XrayScope) -> Vec<String> {
    self
      .vfs
      .scoped(scope)
      .map(|mount| mount.source().root_path().display().to_string())
      .collect()
  }
}
