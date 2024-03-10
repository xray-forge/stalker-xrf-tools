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
          Err(_) => Some(self.validation_error(&format!(
            "Invalid value, floating point number is expected, got '{value}'"
          ))),
        },
        LtxFieldDataType::TypeU32 => match value.parse::<u32>() {
          Ok(_) => None,
          Err(_) => Some(self.validation_error(&format!(
            "Invalid value, unsigned 32 bit number is expected, got '{value}'"
          ))),
        },
        LtxFieldDataType::TypeI32 => match value.parse::<i32>() {
          Ok(_) => None,
          Err(_) => Some(self.validation_error(&format!(
            "Invalid value, signed 32 bit number is expected, got '{value}'"
          ))),
        },
        LtxFieldDataType::TypeU16 => match value.parse::<u16>() {
          Ok(_) => None,
          Err(_) => Some(self.validation_error(&format!(
            "Invalid value, unsigned 16 bit number is expected, got '{value}'"
          ))),
        },
        LtxFieldDataType::TypeI16 => match value.parse::<i16>() {
          Ok(_) => None,
          Err(_) => Some(self.validation_error(&format!(
            "Invalid value, signed 16 bit number is expected, got '{value}'"
          ))),
        },
        LtxFieldDataType::TypeU8 => match value.parse::<u8>() {
          Ok(_) => None,
          Err(_) => Some(self.validation_error(&format!(
            "Invalid value, unsigned 8 bit number is expected, got '{value}'"
          ))),
        },
        LtxFieldDataType::TypeI8 => match value.parse::<i8>() {
          Ok(_) => None,
          Err(_) => Some(self.validation_error(&format!(
            "Invalid value, signed 8 bit number is expected, got '{value}'"
          ))),
        },
        LtxFieldDataType::TypeBool => match value.parse::<bool>() {
          Ok(_) => None,
          Err(_) => Some(self.validation_error(&format!(
            "Invalid value, boolean is expected, got '{value}'"
          ))),
        },
        LtxFieldDataType::TypeVector => {
          let parsed_values: Vec<f32> = value
            .split(',')
            .filter_map(|x| {
              if let Ok(parsed) = x.trim().parse::<f32>() {
                Some(parsed)
              } else {
                None
              }
            })
            .collect();

          if parsed_values.len() != 3 {
            Some(self.validation_error(&format!(
              "Invalid value, comma separated 3d vector coordinates expected, got '{value}'"
            )))
          } else {
            None
          }
        }
        LtxFieldDataType::TypeEnum => None,
        LtxFieldDataType::TypeString => None,
        LtxFieldDataType::TypeUnknown => None,
        LtxFieldDataType::TypeAny => None,
      },
      None => {
        if self.is_optional {
          None
        } else {
          Some(self.validation_error("Field is not provided but required"))
        }
      }
    }
  }

  fn validation_error(&self, message: &str) -> LtxSchemeError {
    LtxSchemeError::new(&self.section, &self.name, message)
  }
}
