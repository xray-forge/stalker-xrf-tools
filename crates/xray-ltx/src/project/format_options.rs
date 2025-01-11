/// Formatting configuration.
#[derive(Clone, Default)]
pub struct LtxFormatOptions {
  /// Whether log is in silent mode.
  pub is_silent: bool,
  /// Whether log is in verbose mode.
  pub is_verbose: bool,
}

impl LtxFormatOptions {
  pub fn new() -> Self {
    Self {
      is_silent: false,
      is_verbose: false,
    }
  }
}

impl LtxFormatOptions {
  pub fn is_logging_enabled(&self) -> bool {
    !self.is_silent
  }

  pub fn is_verbose_logging_enabled(&self) -> bool {
    !self.is_silent && self.is_verbose
  }
}
