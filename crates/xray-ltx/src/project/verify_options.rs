/// Verification configuration.
#[derive(Clone, Default)]
pub struct LtxVerifyOptions {
  /// Whether log is in silent mode.
  pub is_silent: bool,
}

impl LtxVerifyOptions {
  pub fn new() -> LtxVerifyOptions {
    LtxVerifyOptions { is_silent: false }
  }
}
