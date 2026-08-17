use crate::data::visual_description::VisualDescription;

/// A visual flattened into the two things a renderer needs: what it contains, and its bytes.
///
/// The pair is deliberately separate rather than one payload. The description is a typed contract, so
/// a byte range that drifts is caught where types are checked, and the buffer stays a plain blob that
/// crosses a boundary in one transfer without being parsed.
#[derive(Debug)]
pub struct VisualPackage {
  pub description: VisualDescription,
  pub buffer: Vec<u8>,
}
