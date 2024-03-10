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
  pub fn new<S, F>(section: S, name: F) -> LtxFieldScheme
  where
    S: Into<String>,
    F: Into<String>,
  {
    LtxFieldScheme {
      name: name.into(),
      section: section.into(),
      data_type: LtxFieldDataType::TypeF32,
      allowed_values: Vec::new(),
      is_optional: true,
      is_array: false,
    }
  }

  pub fn new_with_type<S, F>(section: S, name: F, data_type: LtxFieldDataType) -> LtxFieldScheme
  where
    S: Into<String>,
    F: Into<String>,
  {
    LtxFieldScheme {
      name: name.into(),
      section: section.into(),
      data_type,
      allowed_values: Vec::new(),
      is_optional: true,
      is_array: false,
    }
  }
}

impl LtxFieldScheme {
  /// Validate provided section based on current field schema definition.
  pub fn validate(&self, section: &Section) -> Option<LtxSchemeError> {
    match section.get(&self.name) {
      Some(value) => {
        if self.is_array {
          self.validate_array_data_entries(value)
        } else {
          self.validate_data_entry(value)
        }
      }
      None => {
        if self.is_optional {
          None
        } else {
          Some(self.validation_error("Field is not provided but required"))
        }
      }
    }
  }

  fn validate_array_data_entries(&self, field_data: &str) -> Option<LtxSchemeError> {
    for entry in field_data.split(',') {
      let entry: &str = entry.trim();

      if !entry.is_empty() {
        let validation_result: Option<LtxSchemeError> = self.validate_data_entry(entry);

        if validation_result.is_some() {
          return validation_result;
        }
      }
    }

    None
  }

  fn validate_data_entry(&self, field_data: &str) -> Option<LtxSchemeError> {
    match self.data_type {
      LtxFieldDataType::TypeF32 => self.validate_f32(field_data),
      LtxFieldDataType::TypeU32 => self.validate_u32(field_data),
      LtxFieldDataType::TypeI32 => self.validate_i32(field_data),
      LtxFieldDataType::TypeU16 => self.validate_u16(field_data),
      LtxFieldDataType::TypeI16 => self.validate_i16(field_data),
      LtxFieldDataType::TypeU8 => self.validate_u8(field_data),
      LtxFieldDataType::TypeI8 => self.validate_i8(field_data),
      LtxFieldDataType::TypeBool => self.validate_bool(field_data),
      LtxFieldDataType::TypeVector => self.validate_vector(field_data),
      LtxFieldDataType::TypeEnum => self.validate_enum(field_data),
      LtxFieldDataType::TypeCondlist => self.validate_condlist(field_data),
      LtxFieldDataType::TypeTuple => self.validate_tuple(field_data),
      LtxFieldDataType::TypeSection => self.validate_section(field_data),
      LtxFieldDataType::TypeString => self.validate_string(field_data),
      LtxFieldDataType::TypeUnknown => None,
      LtxFieldDataType::TypeAny => None,
    }
  }

  fn validation_error(&self, message: &str) -> LtxSchemeError {
    LtxSchemeError::new(&self.section, &self.name, message)
  }
}

impl LtxFieldScheme {
  #[inline(never)]
  fn validate_f32(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<f32>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, floating point number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_u32(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<u32>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, unsigned 32 bit number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_i32(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<i32>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, signed 32 bit number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_u16(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<u16>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, unsigned 16 bit number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_i16(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<i16>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, signed 16 bit number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_u8(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<u8>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, unsigned 8 bit number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_i8(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<i8>() {
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

  /// Validate if provided value is correct comma separated vector.
  /// Expected value like `x,y,z` in f32 format.
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

  fn validate_section(&self, _: &str) -> Option<LtxSchemeError> {
    None
  }

  fn validate_tuple(&self, _: &str) -> Option<LtxSchemeError> {
    None
  }

  fn validate_condlist(&self, _: &str) -> Option<LtxSchemeError> {
    None
  }

  fn validate_string(&self, _: &str) -> Option<LtxSchemeError> {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::LtxFieldScheme;
  use crate::scheme::field_data_type::LtxFieldDataType;

  #[test]
  fn test_u32_validation() {
    let scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeI32);

    assert!(scheme.validate_u32(&u32::MIN.to_string()).is_none());
    assert!(scheme.validate_u32(&i32::MAX.to_string()).is_none());

    assert!(scheme
      .validate_u32(&((u32::MIN as i64) - 1).to_string())
      .is_some());
    assert!(scheme
      .validate_u32(&((u32::MAX as i64) + 1).to_string())
      .is_some());

    assert!(scheme.validate_u32("a").is_some());
    assert!(scheme.validate_u32("true").is_some());
    assert!(scheme.validate_u32(",").is_some());
    assert!(scheme.validate_u32("").is_some());
  }

  #[test]
  fn test_i32_validation() {
    let scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeI32);

    assert!(scheme.validate_i32(&i32::MIN.to_string()).is_none());
    assert!(scheme.validate_i32(&i32::MAX.to_string()).is_none());

    assert!(scheme
      .validate_i32(&((i32::MIN as i64) - 1).to_string())
      .is_some());
    assert!(scheme
      .validate_i32(&((i32::MAX as i64) + 1).to_string())
      .is_some());

    assert!(scheme.validate_i32("a").is_some());
    assert!(scheme.validate_i32("true").is_some());
    assert!(scheme.validate_i32(",").is_some());
    assert!(scheme.validate_i32("").is_some());
  }

  #[test]
  fn test_u16_validation() {
    let scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeU16);

    assert!(scheme.validate_u16(&u16::MIN.to_string()).is_none());
    assert!(scheme.validate_u16(&u16::MAX.to_string()).is_none());

    assert!(scheme
      .validate_u16(&((u16::MIN as i32) - 1).to_string())
      .is_some());
    assert!(scheme
      .validate_u16(&((u16::MAX as i32) + 1).to_string())
      .is_some());

    assert!(scheme.validate_u16("a").is_some());
    assert!(scheme.validate_u16("true").is_some());
    assert!(scheme.validate_u16(",").is_some());
    assert!(scheme.validate_u16("").is_some());
  }

  #[test]
  fn test_i16_validation() {
    let scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeI16);

    assert!(scheme.validate_i16(&i16::MIN.to_string()).is_none());
    assert!(scheme.validate_i16(&i16::MAX.to_string()).is_none());

    assert!(scheme
      .validate_i16(&((i16::MIN as i32) - 1).to_string())
      .is_some());
    assert!(scheme
      .validate_i16(&((i16::MAX as i32) + 1).to_string())
      .is_some());

    assert!(scheme.validate_i16("a").is_some());
    assert!(scheme.validate_i16("true").is_some());
    assert!(scheme.validate_i16(",").is_some());
    assert!(scheme.validate_i16("").is_some());
  }

  #[test]
  fn test_u8_validation() {
    let scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeU8);

    assert!(scheme.validate_u8(&u8::MIN.to_string()).is_none());
    assert!(scheme.validate_u8(&u8::MAX.to_string()).is_none());

    assert!(scheme
      .validate_u8(&((u8::MIN as i32) - 1).to_string())
      .is_some());
    assert!(scheme
      .validate_u8(&((u8::MAX as i32) + 1).to_string())
      .is_some());

    assert!(scheme.validate_u8("a").is_some());
    assert!(scheme.validate_u8("true").is_some());
    assert!(scheme.validate_u8(",").is_some());
    assert!(scheme.validate_u8("").is_some());
  }

  #[test]
  fn test_i8_validation() {
    let scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeI8);

    assert!(scheme.validate_i8(&i8::MIN.to_string()).is_none());
    assert!(scheme.validate_i8(&i8::MAX.to_string()).is_none());

    assert!(scheme
      .validate_i8(&((i8::MIN as i32) - 1).to_string())
      .is_some());
    assert!(scheme
      .validate_i8(&((i8::MAX as i32) + 1).to_string())
      .is_some());

    assert!(scheme.validate_i8("a").is_some());
    assert!(scheme.validate_i8("true").is_some());
    assert!(scheme.validate_i8(",").is_some());
    assert!(scheme.validate_i8("").is_some());
  }

  #[test]
  fn test_bool_validation() {
    let scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeBool);

    assert!(scheme.validate_bool("true").is_none());
    assert!(scheme.validate_bool("false").is_none());

    assert!(scheme.validate_bool("1,2,3,4").is_some());
    assert!(scheme.validate_bool("1").is_some());
    assert!(scheme.validate_bool("0").is_some());
    assert!(scheme.validate_bool("").is_some());
  }

  #[test]
  fn test_vector_validation() {
    let scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeVector);

    assert!(scheme.validate_vector("1,2,3").is_none());
    assert!(scheme.validate_vector("-2,2.5,-0.0025").is_none());
    assert!(scheme.validate_vector("-1.5,-2.5,-3.1").is_none());

    assert!(scheme.validate_vector("").is_some());
    assert!(scheme.validate_vector("1,2,3,4").is_some());
    assert!(scheme.validate_vector("1,2").is_some());
    assert!(scheme.validate_vector("-1,2.0").is_some());
    assert!(scheme.validate_vector("a,b,c").is_some());
  }

  #[test]
  fn test_enum_validation() {
    let mut scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeVector);

    assert!(scheme.validate_enum("a").is_some());

    scheme.allowed_values = vec![String::from("a"), String::from("b_c"), String::from("d")];

    assert!(scheme.validate_enum("a").is_none());
    assert!(scheme.validate_enum("b_c").is_none());
    assert!(scheme.validate_enum("d").is_none());

    assert!(scheme.validate_enum("").is_some());
    assert!(scheme.validate_enum("e").is_some());
    assert!(scheme.validate_enum("f").is_some());
    assert!(scheme.validate_enum("1").is_some());
  }
}
