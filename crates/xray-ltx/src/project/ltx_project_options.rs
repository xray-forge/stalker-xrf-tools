/// Verification configuration.
#[derive(Clone, Default)]
pub struct LtxProjectOptions {
  /// Whether project parsing should include schemes parsing.
  pub is_with_schemes_check: bool,
  /// Whether project parsing and checks should be stricter.
  /// Additional checks with strict mode:
  /// - Case sensitivity of include statements
  pub is_strict_check: bool,
}

impl LtxProjectOptions {
  pub fn new() -> Self {
    Self {
      is_with_schemes_check: false,
      is_strict_check: false,
    }
  }
}
