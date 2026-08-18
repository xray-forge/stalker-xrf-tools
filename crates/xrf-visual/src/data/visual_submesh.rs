use serde::Serialize;

use crate::data::visual_bounds::VisualBounds;
use crate::data::visual_section::{VisualDrawRange, VisualSection, VisualSlideWindow};

/// Where one submesh's attributes sit inside the geometry buffer, and what to draw from them.
///
/// Every section is a byte range into the one buffer the model ships as, so a consumer builds views
/// over it without copying. `indices` covers the whole index buffer, including the coarser detail
/// levels a progressive submesh carries; the resolved draw range renders the model at full
/// detail, already resolved so a consumer never has to pick.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisualGeometry {
  pub vertex_count: u32,
  pub index_count: u32,
  pub positions: VisualSection,
  pub normals: VisualSection,
  pub uvs: VisualSection,
  pub indices: VisualSection,
  pub draw_range: VisualDrawRange,
  /// Detail levels of a progressive submesh, empty for a static one.
  ///
  /// Indices outside the resolved draw range are validated only when a consumer decides to draw them, so a
  /// detail level other than the first must be range checked before use.
  pub windows: Vec<VisualSlideWindow>,
  pub bounds: VisualBounds,
}

/// Why a submesh produced no geometry, graded so a caller does not read the message to find out.
///
/// The distinction is what separates a gap in this crate's coverage from a file that contradicts
/// itself, which is the difference between a sweep noting something and a sweep failing.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VisualSkipCause {
  /// Geometry is stored in a form the packer does not handle, such as a shared vertex or index
  /// container living outside the file.
  Unsupported,
  /// Geometry contradicts itself, such as a detail level reaching past the index buffer it indexes.
  Malformed,
}

/// Whether a submesh produced drawable geometry, and why not when it did not.
///
/// A child that cannot be packed is a value rather than an error so the rest of a model still
/// renders, and so the reason reaches the panel that lists it.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum VisualSubmeshContent {
  Packed { geometry: VisualGeometry },
  Skipped { cause: VisualSkipCause, reason: String },
}

/// One drawable piece of a visual: a child of a skeleton, or a whole single level visual.
#[cfg_attr(feature = "typescript-bindings", derive(specta::Type))]
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisualSubmesh {
  pub index: u32,
  pub model_type: u8,
  pub model_type_label: String,
  /// X-Ray logical texture path, without an extension. A skeleton keeps these on its children rather
  /// than at the top level, which is why a skeleton's own texture chunk is usually absent.
  pub texture_name: Option<String>,
  pub shader_name: Option<String>,
  pub content: VisualSubmeshContent,
}

impl VisualSubmesh {
  pub fn geometry(&self) -> Option<&VisualGeometry> {
    match &self.content {
      VisualSubmeshContent::Packed { geometry } => Some(geometry),
      VisualSubmeshContent::Skipped { .. } => None,
    }
  }

  pub fn skipped(&self) -> Option<(VisualSkipCause, &str)> {
    match &self.content {
      VisualSubmeshContent::Packed { .. } => None,
      VisualSubmeshContent::Skipped { cause, reason } => Some((*cause, reason)),
    }
  }

  #[cfg(test)]
  pub(crate) fn skipped_reason(&self) -> Option<&str> {
    self.skipped().map(|(_, reason)| reason)
  }
}
