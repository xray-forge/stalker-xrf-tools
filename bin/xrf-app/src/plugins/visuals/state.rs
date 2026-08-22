use std::path::Path;
use std::sync::Mutex;

use serde::Serialize;
use xrf_visual::{VisualDependencies, VisualDescription, VisualPackage};

use crate::core::assets::AssetWorldSpec;

/// The visual the viewer currently points at, and its packed bytes.
///
/// Selection is state for the same reason an open archive is: a reload re-provisions the frontend, and without it the
/// viewer would come back empty while the window still says a model is open. Loading itself is not stateful - both
/// commands take the source they act on - so this only ever answers what was selected, never gates what can be read.
///
/// The mounted sources are not here. They live in `core/`'s asset world, shared with every other domain, so opening the
/// same gamedata in two surfaces indexes it once.
pub struct VisualState {
  pub selected: Mutex<Option<SelectedVisual>>,
}

impl VisualState {
  pub fn new() -> Self {
    Self {
      selected: Mutex::new(None),
    }
  }
}

pub struct SelectedVisual {
  pub source: VisualSource,
  /// The world the visual was opened in, kept so a later read searches what the open searched.
  pub world: AssetWorldSpec,
  pub package: VisualPackage,
  /// What the visual's own references came to, decided at open so a read is a lookup rather than a search.
  pub dependencies: VisualDependencies,
}

/// Where a visual is read from.
///
/// Both variants are self-describing, and neither is a handle into mount state: an asset is named by its engine
/// identity, which any surface can spell without having opened anything. The world it is looked for in travels beside
/// the source on every command that takes one, so one call can never mix two worlds.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum VisualSource {
  /// A loose `.ogf` file on disk, named by its filesystem path.
  File { path: String },
  /// An asset of the world, loose or archived, named by its engine identity.
  Asset { logical_path: String },
}

impl VisualSource {
  pub fn label(&self) -> &str {
    match self {
      Self::File { path } => path,
      Self::Asset { logical_path } => logical_path,
    }
  }

  /// Returns the visual's filesystem path when its source provides one.
  ///
  /// An asset has none to give: it may live inside a volume, and the point of addressing it logically is not having to
  /// care. Its own neighborhood is therefore not searched — the world it came from already covers it.
  pub fn physical_path(&self) -> Option<&Path> {
    match self {
      Self::File { path } => Some(Path::new(path)),
      Self::Asset { .. } => None,
    }
  }
}

/// What the viewer is showing, paired with where it came from.
///
/// The source travels back so a frontend that reloaded knows what to ask geometry for, without having to remember
/// anything of its own across the reload.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedVisualDescription {
  pub source: VisualSource,
  /// The world the selection was opened in, so a reloaded frontend asks for geometry the same way.
  pub world: AssetWorldSpec,
  pub description: VisualDescription,
  pub dependencies: VisualDependencies,
}
