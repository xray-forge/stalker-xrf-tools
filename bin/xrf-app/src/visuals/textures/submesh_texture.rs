use serde::Serialize;
use xrf_assets::XrayAssetLocation;

/// The texture the engine substitutes for an unresolved reference.
///
/// A missing texture is a known state rather than an error in the engine: `Layers/xrRender/Texture.cpp:12` loads this in
/// place of the requested texture. The resolver reports `Missing` when this fallback is also absent.
pub const MISSING_TEXTURE_REFERENCE: &str = "ed\\ed_not_existing_texture";

/// The outcome of resolving one submesh texture reference.
///
/// Separate variants distinguish an omitted reference, an unavailable search root, a missing texture, and a located
/// asset. Located assets use `XrayAssetLocation` to describe either a directory or archive container.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SubmeshTextureResolution {
  /// The submesh declares no texture, as is normal for a skeleton root record.
  None,
  /// No visual or fallback root was available, so no lookup was attempted.
  NoRoot,
  /// The reference resolved within the search scope.
  Resolved { location: XrayAssetLocation },
  /// The reference was absent, but the engine's fallback texture resolved.
  Substituted { location: XrayAssetLocation },
  /// Neither the reference nor the engine's fallback texture resolved.
  ///
  /// `roots` lists every source searched by the scope.
  Missing { roots: Vec<String> },
}

impl SubmeshTextureResolution {
  /// Returns the located texture for resolved and substituted outcomes.
  pub fn location(&self) -> Option<&XrayAssetLocation> {
    match self {
      Self::Resolved { location } | Self::Substituted { location } => Some(location),
      Self::None | Self::NoRoot | Self::Missing { .. } => None,
    }
  }
}

/// One submesh texture reference and its resolution outcome.
///
/// When present, the reference is retained for `read_texture` regardless of the resolution outcome.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmeshTexture {
  /// Submesh index used to pair this outcome without relying on response order.
  pub submesh_index: u32,
  /// X-Ray texture reference declared by the submesh, or `None` when omitted.
  pub reference: Option<String>,
  pub resolution: SubmeshTextureResolution,
}
