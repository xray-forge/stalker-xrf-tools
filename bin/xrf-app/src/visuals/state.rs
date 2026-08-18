use std::path::Path;
use std::sync::Mutex;

use serde::Serialize;
use xrf_visual::{VisualDescription, VisualPackage};

use crate::visuals::textures::submesh_texture::SubmeshTexture;
use crate::visuals::textures::texture_resolver::VisualTextureResolver;

/// The visual the viewer currently points at, and its packed bytes.
///
/// Selection is state for the same reason an open archive is: a reload re-provisions the frontend, and
/// without it the viewer would come back empty while the window still says a model is open. Loading
/// itself is not stateful - both commands take the source they act on - so this only ever answers what
/// was selected, never gates what can be read.
///
/// The resolver sits beside the selection rather than inside it because its value is the index cache it
/// accumulates, which outlives any one model.
pub struct VisualState {
  pub selected: Mutex<Option<SelectedVisual>>,
  pub textures: Mutex<VisualTextureResolver>,
}

impl VisualState {
  pub fn new() -> Self {
    Self {
      selected: Mutex::new(None),
      textures: Mutex::new(VisualTextureResolver::new()),
    }
  }
}

pub struct SelectedVisual {
  pub source: VisualSource,
  pub package: VisualPackage,
  /// Resolution outcome per submesh, decided at open so `read_texture` is a lookup rather than a search.
  pub textures: Vec<SubmeshTexture>,
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

  /// Where this visual sits on disk, when it sits on disk at all.
  ///
  /// An archived visual will answer `None`, which is exactly the case the root chain's ambient link exists for.
  pub fn physical_path(&self) -> Option<&Path> {
    match self {
      Self::File { path } => Some(Path::new(path)),
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
  pub textures: Vec<SubmeshTexture>,
}
