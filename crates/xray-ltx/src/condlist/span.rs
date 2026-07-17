use xray_error::XRayError;

/// A byte range in the original condition-list value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceSpan {
  pub start: usize,
  pub end: usize,
}

impl SourceSpan {
  pub const fn new(start: usize, end: usize) -> Self {
    Self { start, end }
  }

  pub(crate) fn parsing_error(start: usize, _end: usize, message: impl Into<String>) -> XRayError {
    XRayError::new_parsing_error(format!(
      "Invalid condlist syntax at byte {}: {}",
      start + 1,
      message.into()
    ))
  }
}
