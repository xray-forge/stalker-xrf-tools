/// Verification configuration.
#[derive(Clone, Default)]
pub struct LtxVerifyOptions {
  /// Whether log is in silent mode.
  pub is_silent: bool,
  /// Whether log is in verbose mode.
  pub is_verbose: bool,
  /// Whether check is in strict mode and requires validation schemas for all data.
  pub is_strict: bool,
}

impl LtxVerifyOptions {
  pub fn new() -> Self {
    Self {
      is_silent: false,
      is_verbose: false,
      is_strict: false,
    }
  }
}
