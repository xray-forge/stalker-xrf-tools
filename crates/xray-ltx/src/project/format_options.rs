/// Formatting configuration.
#[derive(Clone, Default)]
pub struct LtxFormatOptions {
  /// Whether log is in silent mode.
  pub is_silent: bool,
}

impl LtxFormatOptions {
  pub fn new() -> LtxFormatOptions {
    LtxFormatOptions { is_silent: false }
  }
}
