use xrf_db::{OgfFile, XRayByteOrder};
use xrf_visual::{VisualPackage, VisualPacker};

use crate::app::types::TauriResult;
use crate::plugins::visuals::state::VisualSource;

/// Read a visual and flatten it for rendering.
///
/// Shared by every command that needs geometry, so a description and the buffer it describes always
/// come out of the same code path even when they were asked for separately.
pub fn pack_source(source: &VisualSource) -> TauriResult<VisualPackage> {
  match source {
    VisualSource::File { path } => {
      let file: OgfFile = OgfFile::read_from_path::<XRayByteOrder, _>(path)
        .map_err(|error| format!("Failed to read visual '{path}': {error}"))?;

      Ok(VisualPacker::pack(&file))
    }
  }
}
