use crate::{Finding, GamedataCheckResult, GamedataVerificationStatus};
use xray_db::ShaderLibraryFile;

pub(crate) struct GamedataShaderLibraryVerificationResult {
  pub(crate) blender_count: usize,
  pub(crate) checked_count: u32,
  pub(crate) findings: Vec<Finding>,
  pub(crate) invalid_count: u32,
  library: Option<ShaderLibraryFile>,
}

impl GamedataShaderLibraryVerificationResult {
  pub(crate) fn passed(library: ShaderLibraryFile) -> Self {
    Self {
      blender_count: library.blenders_count(),
      checked_count: 1,
      findings: Vec::new(),
      invalid_count: 0,
      library: Some(library),
    }
  }

  pub(crate) fn failed(finding: Finding) -> Self {
    Self {
      blender_count: 0,
      checked_count: 1,
      findings: vec![finding],
      invalid_count: 1,
      library: None,
    }
  }

  pub(crate) fn library(&self) -> Option<&ShaderLibraryFile> {
    self.library.as_ref()
  }
}

impl GamedataCheckResult for GamedataShaderLibraryVerificationResult {
  fn status(&self) -> GamedataVerificationStatus {
    GamedataVerificationStatus::from_is_valid(self.invalid_count == 0)
  }
  fn failure_message(&self) -> String {
    format!(
      "{}/{} shader libraries valid, {} blender definitions",
      self.checked_count - self.invalid_count,
      self.checked_count,
      self.blender_count
    )
  }
  fn findings(&self) -> &[Finding] {
    &self.findings
  }
}
