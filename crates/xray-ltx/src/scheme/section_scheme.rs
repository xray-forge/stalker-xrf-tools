use crate::file::types::LtxSectionFieldSchemes;

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
}
