use std::path::Path;

use xrf_error::XrfResult;
use xrf_vfs::mount_plan;
use xrf_vfs::{XrayLookupScope, XrayMountPlan, XrayVfs};
use xrf_visual::VisualSubmesh;

use crate::core::types::TauriResult;
use crate::plugins::visuals::textures::submesh_texture::{
  MISSING_TEXTURE_REFERENCE, SubmeshTexture, SubmeshTextureResolution,
};

/// Resolves visual textures while retaining mounted asset sources between requests.
#[derive(Default)]
pub struct VisualTextureResolver {
  vfs: XrayVfs,
}

impl VisualTextureResolver {
  pub fn new() -> Self {
    Self::default()
  }

  /// Resolves each submesh texture using the visual's ordered source scope.
  pub fn resolve_submeshes(
    &mut self,
    visual: Option<&Path>,
    fallback_root: Option<&Path>,
    submeshes: &[VisualSubmesh],
  ) -> Vec<SubmeshTexture> {
    let scope: XrayLookupScope = self.scope_for(visual, fallback_root);

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
  pub fn resolve(&self, scope: &XrayLookupScope, reference: &str) -> SubmeshTextureResolution {
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

  /// Reads a texture from the mount that resolved it.
  ///
  /// Resolved first, then read from the source that answered.
  pub fn read(&self, scope: &XrayLookupScope, reference: &str) -> TauriResult<Vec<u8>> {
    let Some(asset) = self
      .vfs
      .dds_texture(scope, reference)
      .map_err(|error| format!("Rejected texture reference '{reference}': {error}"))?
    else {
      return Err(format!("Failed to read texture '{reference}': it resolves to nothing"));
    };

    self
      .vfs
      .read_asset(&asset)
      .map_err(|error| format!("Failed to read texture '{reference}': {error}"))
  }

  /// Mounts texture sources for a visual and returns their search scope.
  ///
  /// Search order is the visual's implied loose root, its containing installation, then `fallback_root`. An empty scope
  /// means no source could be planned or mounted.
  pub fn scope_for(&mut self, visual: Option<&Path>, fallback_root: Option<&Path>) -> XrayLookupScope {
    let plan: XrayMountPlan = self.plan_for(visual, fallback_root);

    XrayLookupScope::only(mount_plan(&mut self.vfs, &plan).unwrap_or_default())
  }

  /// Builds a priority-ordered plan from the visual root, containing installation, and fallback root.
  ///
  /// [`XrayMountPlan::behind`] drops duplicate paths.
  fn plan_for(&self, visual: Option<&Path>, fallback_root: Option<&Path>) -> XrayMountPlan {
    let implied: XrayMountPlan = Self::planned(visual.map(XrayMountPlan::implied));
    let install: XrayMountPlan = Self::planned(visual.map(XrayMountPlan::implied_install));
    let fallback: XrayMountPlan = Self::planned(fallback_root.filter(|root| root.is_dir()).map(XrayMountPlan::root));

    implied.behind(install).behind(fallback)
  }

  /// Returns an optional plan, or an empty plan when absent or failed.
  ///
  /// Planning failures are logged; one malformed installation must not suppress the visual's loose root.
  fn planned(plan: Option<XrfResult<XrayMountPlan>>) -> XrayMountPlan {
    plan
      .transpose()
      .inspect_err(|error| log::warn!("Failed to plan visual texture mounts: {error}"))
      .unwrap_or_default()
      .unwrap_or_default()
  }

  fn lookup(&self, scope: &XrayLookupScope, reference: &str) -> Option<xrf_vfs::XrayAsset> {
    self
      .vfs
      .dds_texture(scope, reference)
      .inspect_err(|error| log::warn!("Rejected texture reference '{reference}': {error}"))
      .ok()
      .flatten()
  }

  fn mounts_in(&self, scope: &XrayLookupScope) -> usize {
    self.vfs.scoped(scope).count()
  }

  /// Describes every mount searched by a failed lookup.
  fn described_mounts(&self, scope: &XrayLookupScope) -> Vec<String> {
    self
      .vfs
      .scoped(scope)
      .map(|mount| mount.source().root_path().display().to_string())
      .collect()
  }
}
