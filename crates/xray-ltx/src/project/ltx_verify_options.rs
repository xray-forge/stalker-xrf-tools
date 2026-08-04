/// Verification configuration.
#[derive(Clone, Default)]
pub struct LtxVerifyOptions {
  /// Caller-controlled live output.
  pub output: xray_output::OutputOptions,
}
