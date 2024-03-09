use crate::file::types::LtxSectionFieldSchemes;
use crate::LtxReadError;

/// Scheme definition for section in LTX file.
#[derive(Clone, Debug, Default)]
pub struct LtxSectionScheme {
  pub name: String,
  pub is_strict: bool,
  pub fields: LtxSectionFieldSchemes,
}

impl LtxSectionScheme {
  pub fn new(name: &str) -> LtxSectionScheme {
    LtxSectionScheme {
      name: name.into(),
      is_strict: false,
      fields: Default::default(),
    }
  }

  /// Parse whether strict mode is activated for ltx scheme.
  pub fn parse_strict_mode(value: &str) -> Result<bool, LtxReadError> {
    match value {
      "true" => Ok(true),
      "false" => Ok(false),
      &_ => Err(LtxReadError::new(format!(
        "Invalid scheme declaration, unexpected value for 'strict field' - {value}"
      ))),
    }
  }
}
