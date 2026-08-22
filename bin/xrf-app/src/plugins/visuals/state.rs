use std::path::Path;
use std::sync::Mutex;

use serde::Serialize;
use xrf_visual::{VisualDependencies, VisualDescription, VisualPackage};

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
  pub package: VisualPackage,
  /// What the visual's own references came to, decided at open so a read is a lookup rather than a search.
  pub dependencies: VisualDependencies,
}

/// Where a visual is read from.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum VisualSource {
  /// A loose `.ogf` file on disk.
  File { path: String },
}

impl VisualSource {
  pub fn label(&self) -> &str {
    match self {
      Self::File { path } => path,
    }
  }

  /// Returns the visual's filesystem path when its source provides one.
  pub fn physical_path(&self) -> Option<&Path> {
    match self {
      Self::File { path } => Some(Path::new(path)),
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
  pub description: VisualDescription,
  pub dependencies: VisualDependencies,
}
