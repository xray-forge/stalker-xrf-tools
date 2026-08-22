use serde::Serialize;

use crate::XrayAsset;

/// What one reference lookup came to.
///
/// A fact about a lookup, not about the kind of thing looked up: a texture, a motion set and a level asset all end in one
/// of these four states, so a consumer renders one shape and a domain crate pairs the outcome with its own reference
/// identity rather than defining its own vocabulary.
///
/// A missing asset is a state rather than an error, because it is one in the engine too — `Missing` carries where the
/// probe looked so a report can say that instead of only that nothing was found.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum XrayResolution {
  /// The reference itself resolved.
  ///
  /// `assets` is never empty, and holds more than one entry only for a mask — a motion reference may name a set.
  Resolved { step: String, assets: Vec<XrayAsset> },
  /// The reference did not resolve, but the fallback the caller offered did.
  ///
  /// Substitution is engine behavior a caller opts into per kind, so the fallback reference travels back: reporting the
  /// asset alone would show a located texture while hiding that it is not the requested one.
  Substituted {
    step: String,
    fallback: String,
    assets: Vec<XrayAsset>,
  },
  /// Nothing resolved, across every step of the probe.
  ///
  /// `roots` is every source searched, in probe order and without duplicates.
  Missing { roots: Vec<String> },
  /// There was nothing to search: the probe had no step, or no step selected a mounted source.
  ///
  /// Distinct from `Missing` because it says the question could not be asked rather than that the answer was no, which is
  /// the difference between an unconfigured project and an absent asset.
  NoScope,
  /// The reference could not be turned into a lookup at all, so none was attempted.
  ///
  /// Engine text is untrusted: a mesh header may hold a name no logical path can be made of. Folding that into `Missing`
  /// would report a garbage reference as an absent asset, and substituting for it would report it as a present one.
  Rejected { reason: String },
}

impl XrayResolution {
  /// The located assets, empty unless the reference or its fallback resolved.
  pub fn get_assets(&self) -> &[XrayAsset] {
    match self {
      Self::Resolved { assets, .. } | Self::Substituted { assets, .. } => assets,
      Self::Missing { .. } | Self::NoScope | Self::Rejected { .. } => &[],
    }
  }

  /// The first located asset, for a reference that cannot be a mask.
  pub fn get_asset(&self) -> Option<&XrayAsset> {
    self.get_assets().first()
  }

  /// The probe step that answered, for a located outcome.
  pub fn get_step(&self) -> Option<&str> {
    match self {
      Self::Resolved { step, .. } | Self::Substituted { step, .. } => Some(step),
      Self::Missing { .. } | Self::NoScope | Self::Rejected { .. } => None,
    }
  }

  /// Whether anything was located, by the reference or by its fallback.
  pub fn is_located(&self) -> bool {
    !self.get_assets().is_empty()
  }
}
