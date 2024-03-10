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

  /// Get count of fields required in scheme.
  pub fn get_required_fields_count(&self) -> usize {
    self
      .fields
      .iter()
      .filter(|(_, definition)| !definition.is_optional)
      .count()
  }

  /// Parse whether strict mode is activated for ltx scheme.
  pub fn parse_strict_mode(value: &str) -> Result<bool, LtxReadError> {
    match value.parse::<bool>() {
      Ok(value) => Ok(value),
      Err(_) => Err(LtxReadError::new(format!(
        "Invalid scheme declaration, unexpected value for 'strict' field - '{value}', boolean expected"
      ))),
    }
  }
}
