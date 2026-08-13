use serde::{Deserialize, Serialize};

/// Outcome of a guarded thm bump patch.
///
/// Returned instead of logging from the processor so callers own their own output format, and so a
/// dry run can report exactly what a real run would have written.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThmBumpPatchReport {
  /// Size of the source file before patching.
  pub original_size: usize,
  /// Size of the patched buffer, written unless the patch was a dry run.
  pub patched_size: usize,
  /// Bump name the descriptor pointed at before patching.
  pub previous_name: String,
  /// Bump mode the descriptor declared before patching, see [`crate::ThmBumpChunk`].
  pub previous_mode: u32,
  /// Whether the patched buffer was actually written to the destination.
  pub is_dry_run: bool,
}
