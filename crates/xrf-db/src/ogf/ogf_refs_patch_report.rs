use serde::{Deserialize, Serialize};

/// Outcome of a guarded ogf refs patch.
///
/// Returned instead of logging from the processor so callers own their own output format, and so a
/// dry run can report exactly what a real run would have written.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfRefsPatchReport {
  /// Size of the source file before patching.
  pub original_size: usize,
  /// Size of the patched buffer, written unless the patch was a dry run.
  pub patched_size: usize,
  /// How many references were rewritten, which is what distinguishes a real patch from a no-op.
  pub patched_count: u32,
  /// Whether the patched buffer was actually written to the destination.
  pub is_dry_run: bool,
}
