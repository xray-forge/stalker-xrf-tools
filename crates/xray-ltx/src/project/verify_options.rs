/// Verification configuration.
#[derive(Clone, Default)]
pub struct LtxVerifyOptions {
  /// Whether log is in silent mode.
  pub is_silent: bool,
  /// Whether log is in verbose mode.
  pub is_verbose: bool,
}

impl LtxVerifyOptions {
  pub fn new() -> LtxVerifyOptions {
    LtxVerifyOptions {
      is_silent: false,
      is_verbose: false,
    }
  }
}
