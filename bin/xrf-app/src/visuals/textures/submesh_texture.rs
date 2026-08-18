use std::path::PathBuf;

use serde::Serialize;
use xrf_assets::XrayAssetLocation;

/// The texture the engine substitutes for a reference that resolves nowhere.
///
/// A missing texture is a known state rather than an error in the engine: `Layers/xrRender/Texture.cpp:12` loads this in
/// place of whatever was asked for. Present in all four reference gamedata trees, so substituting it means the viewport
/// shows what the game would show.
pub const MISSING_TEXTURE_REFERENCE: &str = "ed\\ed_not_existing_texture";

/// What resolving one submesh's texture reference produced.
///
/// A tagged enum rather than a struct of options, so the impossible combinations - a resolved texture with no
/// location, a root-less lookup that still found a file - cannot be constructed or arrive on the wire.
///
/// The located variants carry an [`XrayAssetLocation`] rather than their own path fields, so they gain archive-backed
/// assets when that type does.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SubmeshTextureResolution {
  /// The submesh declares no texture at all, which is normal for a skeleton's own record.
  None,
  /// Nothing above the visual looks like an X-Ray root and no project root was offered, so no lookup was attempted.
  ///
  /// Distinct from `Missing` on purpose: it says the question could not be asked, not that the answer was no.
  NoRoot,
  /// The reference resolved inside a root.
  Resolved { location: XrayAssetLocation },
  /// The reference resolved nowhere, so the engine's dummy stands in - as it does in game.
  Substituted { location: XrayAssetLocation },
  /// Neither the reference nor the dummy resolved, so there is nothing to show.
  Missing { root: PathBuf },
}

impl SubmeshTextureResolution {
  /// The asset to read bytes from, when one was located.
  pub fn location(&self) -> Option<&XrayAssetLocation> {
    match self {
      Self::Resolved { location } | Self::Substituted { location } => Some(location),
      Self::None | Self::NoRoot | Self::Missing { .. } => None,
    }
  }
}

/// One submesh's texture reference paired with what became of it.
///
/// The reference stays outside the outcome because it is what `read_texture` is addressed by, and it is the same string
/// whether resolution succeeded, substituted or failed.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmeshTexture {
  /// Index of the submesh this belongs to, so a consumer pairs them without relying on order.
  pub submesh_index: u32,
  /// X-Ray logical path the submesh declares, absent when it declares none.
  pub reference: Option<String>,
  pub resolution: SubmeshTextureResolution,
}
