use std::sync::Mutex;

use serde::Serialize;
use xrf_visual::{VisualDescription, VisualPackage};

/// The visual the viewer currently points at, and its packed bytes.
///
/// Selection is state for the same reason an open archive is: a reload re-provisions the frontend, and
/// without it the viewer would come back empty while the window still says a model is open. Loading
/// itself is not stateful - both commands take the source they act on - so this only ever answers what
/// was selected, never gates what can be read.
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
}

/// Where a visual is read from.
///
/// An enum from the start because reading a visual out of an opened archive is the next source, and it
/// differs only in how bytes are obtained. Keeping the shape means that arrives as a variant rather
/// than as a second pair of commands.
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
}

/// What the viewer is showing, paired with where it came from.
///
/// The source travels back so a frontend that reloaded knows what to ask geometry for, without having
/// to remember anything of its own across the reload.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedVisualDescription {
  pub source: VisualSource,
  pub description: VisualDescription,
}
