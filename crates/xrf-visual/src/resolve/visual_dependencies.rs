use serde::Serialize;
use xrf_vfs::{XrayAssetType, XrayProbe, XrayResolution};

use crate::data::visual_description::VisualDescription;

/// One texture a visual's submesh declares, and what the reference came to.
///
/// Paired with the submesh index rather than positioned in a list, so an outcome cannot be joined to the wrong submesh by
/// a caller that reorders or resolves in parallel.
///
/// A submesh declaring no texture has no entry here at all — that is the normal case for a skeleton's own record, and
/// absence says it more plainly than a variant meaning "nothing was asked".
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisualTextureDependency {
  pub submesh_index: u32,
  pub reference: String,
  pub resolution: XrayResolution,
}

/// One motion file set a visual animates from, and what the reference came to.
///
/// A reference may be a mask — `wpn\wpn_ak74_*.omf` names every matching file — so one entry can hold several located
/// assets. Embedded motions are not here: they are inside the visual and there is nothing to resolve.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisualMotionDependency {
  pub reference: String,
  pub resolution: XrayResolution,
}

/// Everything a visual needs from outside itself, resolved.
///
/// The crate that parses a visual is the crate that knows what it references, so extraction lives beside the parser. It
/// resolves through a borrowed probe and never mounts or plans: which sources exist, and in what order, is the calling
/// binary's policy, and a viewer, a sweep and a level editor each answer it differently.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisualDependencies {
  pub textures: Vec<VisualTextureDependency>,
  pub motions: Vec<VisualMotionDependency>,
}

impl VisualDependencies {
  /// The texture the engine substitutes for a reference it cannot find.
  ///
  /// A missing texture is a state rather than an error in the engine: `Layers/xrRender/Texture.cpp:12` loads this in place
  /// of the requested one, so a viewer that shows it is showing what the game shows.
  pub const MISSING_TEXTURE_REFERENCE: &'static str = "ed\\ed_not_existing_texture";

  /// Resolves every reference a visual declares, in the order the probe searches.
  ///
  /// Never fails as a whole: a reference no logical path can be made of becomes a rejected outcome for that reference
  /// alone, because engine text is untrusted and one bad name in a mesh header must not cost the model its other assets.
  pub fn resolve(description: &VisualDescription, probe: &XrayProbe) -> Self {
    Self {
      textures: description
        .submeshes
        .iter()
        .filter_map(|submesh| {
          submesh.texture_name.as_ref().map(|reference| VisualTextureDependency {
            submesh_index: submesh.index,
            reference: reference.clone(),
            resolution: Self::resolve_texture(probe, reference),
          })
        })
        .collect(),
      motions: description
        .motion_refs
        .iter()
        .map(|reference| VisualMotionDependency {
          reference: reference.clone(),
          resolution: Self::resolve_motion(probe, reference),
        })
        .collect(),
    }
  }

  /// The outcome for one submesh, by the index the submesh reports.
  pub fn find_texture(&self, submesh_index: u32) -> Option<&VisualTextureDependency> {
    self
      .textures
      .iter()
      .find(|texture| texture.submesh_index == submesh_index)
  }

  /// Resolves a texture reference, substituting the engine's dummy the way the renderer does.
  fn resolve_texture(probe: &XrayProbe, reference: &str) -> XrayResolution {
    Self::tolerate(probe.resolve_with_fallback(XrayAssetType::Dds, reference, Self::MISSING_TEXTURE_REFERENCE))
  }

  /// Resolves a motion reference, which has no substitute: nothing stands in for an absent animation set.
  fn resolve_motion(probe: &XrayProbe, reference: &str) -> XrayResolution {
    Self::tolerate(probe.resolve(XrayAssetType::Omf, reference))
  }

  fn tolerate(resolution: xrf_error::XrfResult<XrayResolution>) -> XrayResolution {
    match resolution {
      Ok(resolution) => resolution,
      Err(error) => XrayResolution::Rejected {
        reason: error.to_string(),
      },
    }
  }
}
