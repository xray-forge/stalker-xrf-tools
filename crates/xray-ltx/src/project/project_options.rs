/// Verification configuration.
#[derive(Clone, Default)]
pub struct LtxProjectOptions {
  /// Whether project parsing should include schemes parsing.
  pub is_with_schemes_check: bool,
}

impl LtxProjectOptions {
  pub fn new() -> LtxProjectOptions {
    LtxProjectOptions {
      is_with_schemes_check: false,
    }
  }
}
