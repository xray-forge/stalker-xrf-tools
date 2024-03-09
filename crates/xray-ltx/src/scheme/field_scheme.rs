use crate::error::ltx_scheme_error::LtxSchemeError;
use crate::scheme::field_data_type::LtxFieldDataType;
use crate::Section;

/// Scheme definition for single field in LTX file section.
#[derive(Clone, Debug)]
pub struct LtxFieldScheme {
  pub section: String,
  pub name: String,
  pub is_optional: bool,
  pub data_type: LtxFieldDataType,
}

impl LtxFieldScheme {
  pub fn new(section: String, name: String) -> LtxFieldScheme {
    LtxFieldScheme {
      section,
      name,
      is_optional: true,
      data_type: LtxFieldDataType::TypeF32,
    }
  }
}

impl LtxFieldScheme {
  pub fn validate(&self, section: &Section) -> Option<LtxSchemeError> {
    match section.get(&self.name) {
      Some(value) => match self.data_type {
        LtxFieldDataType::TypeF32 => match value.parse::<f32>() {
          Ok(_) => None,
          Err(_) => Some(self.validation_error("Invalid value, floating point number is expected")),
        },
        LtxFieldDataType::TypeU32 => match value.parse::<f32>() {
          Ok(_) => None,
          Err(_) => {
            Some(self.validation_error("Invalid value, unsigned 32 bit number is expected"))
          }
        },
        LtxFieldDataType::TypeI32 => match value.parse::<f32>() {
          Ok(_) => None,
          Err(_) => Some(self.validation_error("Invalid value, signed 32 bit number is expected")),
        },
        LtxFieldDataType::TypeU16 => match value.parse::<f32>() {
          Ok(_) => None,
          Err(_) => {
            Some(self.validation_error("Invalid value, unsigned 16 bit number is expected"))
          }
        },
        LtxFieldDataType::TypeI16 => match value.parse::<f32>() {
          Ok(_) => None,
          Err(_) => Some(self.validation_error("Invalid value, signed 16 bit number is expected")),
        },
        LtxFieldDataType::TypeString => None,
        LtxFieldDataType::TypeUnknown => None,
        LtxFieldDataType::TypeAny => None,
      },
      None => {
        if self.is_optional {
          None
        } else {
          Some(self.validation_error("Field is required"))
        }
      }
    }
  }

  fn validation_error(&self, message: &str) -> LtxSchemeError {
    LtxSchemeError::new(&self.section, &self.name, message)
  }
}
