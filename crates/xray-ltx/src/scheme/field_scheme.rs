use crate::error::ltx_scheme_error::LtxSchemeError;
use crate::scheme::field_data_type::LtxFieldDataType;
use crate::Section;

/// Scheme definition for single field in LTX file section.
#[derive(Clone, Debug)]
pub struct LtxFieldScheme {
  pub section: String,
  pub name: String,
  pub data_type: LtxFieldDataType,
  pub allowed_values: Vec<String>,
  pub is_optional: bool,
  pub is_array: bool,
}

impl LtxFieldScheme {
  pub fn new(section: String, name: String) -> LtxFieldScheme {
    LtxFieldScheme {
      section,
      name,
      data_type: LtxFieldDataType::TypeF32,
      allowed_values: Vec::new(),
      is_optional: true,
      is_array: false,
    }
  }
}

impl LtxFieldScheme {
  pub fn validate(&self, section: &Section) -> Option<LtxSchemeError> {
    match section.get(&self.name) {
      Some(value) => match self.data_type {
        LtxFieldDataType::TypeF32 => self.validate_f32(value),
        LtxFieldDataType::TypeU32 => self.validate_u32(value),
        LtxFieldDataType::TypeI32 => self.validate_i32(value),
        LtxFieldDataType::TypeU16 => self.validate_u16(value),
        LtxFieldDataType::TypeI16 => self.validate_i16(value),
        LtxFieldDataType::TypeU8 => self.validate_u8(value),
        LtxFieldDataType::TypeI8 => self.validate_i8(value),
        LtxFieldDataType::TypeBool => self.validate_bool(value),
        LtxFieldDataType::TypeVector => self.validate_vector(value),
        LtxFieldDataType::TypeEnum => self.validate_enum(value),
        LtxFieldDataType::TypeString => self.validate_string(value),
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

impl LtxFieldScheme {
  fn validate_f32(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<f32>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, floating point number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_u32(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<f32>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, unsigned 32 bit number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_i32(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<f32>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, signed 32 bit number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_u16(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<f32>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, unsigned 16 bit number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_i16(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<f32>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, signed 16 bit number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_u8(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<f32>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, unsigned 8 bit number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_i8(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<f32>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, signed 8 bit number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_bool(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<bool>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, boolean is expected, got '{value}'"
      ))),
    }
  }

  /// Validate if provided value is correct comma separated vector
  /// Expected value like `x,y,z` in f32 format
  fn validate_vector(&self, value: &str) -> Option<LtxSchemeError> {
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

  /// Validate if provided value is correct enumeration defined field.
  fn validate_enum(&self, value: &str) -> Option<LtxSchemeError> {
    if self.allowed_values.is_empty() {
      Some(self.validation_error("Unexpected enum check - list of possible values is empty"))
    } else if self.allowed_values.contains(&value.into()) {
      None
    } else {
      Some(self.validation_error(&format!(
        "Invalid value, one of possible values [{}] expected, got '{value}'",
        self.allowed_values.join(",")
      )))
    }
  }

  fn validate_string(&self, _: &str) -> Option<LtxSchemeError> {
    None
  }
}
