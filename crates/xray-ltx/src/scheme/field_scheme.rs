use crate::error::ltx_scheme_error::LtxSchemeError;
use crate::scheme::field_data_type::LtxFieldDataType;
use crate::Section;

/// Scheme definition for single field in LTX file section.
#[derive(Clone, Debug)]
pub struct LtxFieldScheme {
  pub section: String,
  pub name: String,
  pub data_type: LtxFieldDataType,
  pub is_optional: bool,
  pub is_array: bool,
  // todo: Add range (min-max) support.
  // todo: Add constant value support.
  // todo: Add time value support.
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
      is_optional: true,
      is_array: false,
    }
  }
}

impl LtxFieldScheme {
  /// Validate provided value based on current field schema definition.
  pub fn validate_value(&self, value: &str) -> Option<LtxSchemeError> {
    if self.is_array {
      self.validate_array_data_entries(value)
    } else {
      self.validate_data_entry(value)
    }
  }

  /// Validate provided section based on current field schema definition.
  pub fn validate_section(&self, section: &Section) -> Option<LtxSchemeError> {
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
    self.validate_data_entry_by_type(&self.data_type, field_data)
  }

  fn validate_data_entry_by_type(
    &self,
    field_type: &LtxFieldDataType,
    field_data: &str,
  ) -> Option<LtxSchemeError> {
    match field_type {
      LtxFieldDataType::TypeF32 => self.validate_f32_type(field_data),
      LtxFieldDataType::TypeU32 => self.validate_u32_type(field_data),
      LtxFieldDataType::TypeI32 => self.validate_i32_type(field_data),
      LtxFieldDataType::TypeU16 => self.validate_u16_type(field_data),
      LtxFieldDataType::TypeI16 => self.validate_i16_type(field_data),
      LtxFieldDataType::TypeU8 => self.validate_u8_type(field_data),
      LtxFieldDataType::TypeI8 => self.validate_i8_type(field_data),
      LtxFieldDataType::TypeBool => self.validate_bool_type(field_data),
      LtxFieldDataType::TypeVector => self.validate_vector_type(field_data),
      LtxFieldDataType::TypeEnum(_) => self.validate_enum_type(field_data),
      LtxFieldDataType::TypeCondlist => self.validate_condlist_type(field_data),
      LtxFieldDataType::TypeTuple(_) => self.validate_tuple_type(field_data),
      LtxFieldDataType::TypeSection => self.validate_section_type(field_data),
      LtxFieldDataType::TypeString => self.validate_string_type(field_data),
      LtxFieldDataType::TypeUnknown => None,
      LtxFieldDataType::TypeAny => None,
    }
  }

  fn validation_error(&self, message: &str) -> LtxSchemeError {
    LtxSchemeError::new(&self.section, &self.name, message)
  }
}

impl LtxFieldScheme {
  fn validate_f32_type(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<f32>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, floating point number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_u32_type(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<u32>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, unsigned 32 bit number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_i32_type(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<i32>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, signed 32 bit number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_u16_type(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<u16>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, unsigned 16 bit number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_i16_type(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<i16>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, signed 16 bit number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_u8_type(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<u8>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, unsigned 8 bit number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_i8_type(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<i8>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, signed 8 bit number is expected, got '{value}'"
      ))),
    }
  }

  fn validate_bool_type(&self, value: &str) -> Option<LtxSchemeError> {
    match value.parse::<bool>() {
      Ok(_) => None,
      Err(_) => Some(self.validation_error(&format!(
        "Invalid value, boolean is expected, got '{value}'"
      ))),
    }
  }

  /// Validate if provided value is correct comma separated vector.
  /// Expected value like `x,y,z` in f32 format.
  fn validate_vector_type(&self, value: &str) -> Option<LtxSchemeError> {
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
  fn validate_enum_type(&self, value: &str) -> Option<LtxSchemeError> {
    match &self.data_type {
      LtxFieldDataType::TypeEnum(allowed_values) => {
        if allowed_values.is_empty() {
          Some(self.validation_error("Unexpected enum check - list of possible values is empty"))
        } else if allowed_values.contains(&value.into()) {
          None
        } else {
          Some(self.validation_error(&format!(
            "Invalid value, one of possible values [{}] expected, got '{value}'",
            allowed_values.join(",")
          )))
        }
      }
      _ => Some(self.validation_error(
        "Unexpected enum type check, trying to validate enum with non-enum field",
      )),
    }
  }

  /// Validate if provided value matches tuple description.
  fn validate_tuple_type(&self, value: &str) -> Option<LtxSchemeError> {
    match &self.data_type {
      LtxFieldDataType::TypeTuple(types) => {
        if types.is_empty() {
          Some(self.validation_error("Unexpected tuple check - list of possible values is empty"))
        } else {
          let values: Vec<&str> = value.split(',').map(|it| it.trim()).collect();

          if values.len() != types.len() {
            Some(self.validation_error(&format!(
              "Invalid value, expected {} comma separated values expected, got '{value}'",
              types.len(),
            )))
          } else {
            // Validate all provided values.
            for it in 0..values.len() {
              if let Some(error) =
                self.validate_data_entry_by_type(types.get(it).unwrap(), values.get(it).unwrap())
              {
                return Some(self.validation_error(&format!(
                  "Invalid value in tuple, one of values did not pass validation: {}",
                  error.message
                )));
              }
            }

            None
          }
        }
      }
      _ => Some(self.validation_error(
        "Unexpected tuple type check, trying to validate enum with non-enum field",
      )),
    }
  }

  fn validate_section_type(&self, _: &str) -> Option<LtxSchemeError> {
    None
  }

  fn validate_condlist_type(&self, _: &str) -> Option<LtxSchemeError> {
    None
  }

  fn validate_string_type(&self, _: &str) -> Option<LtxSchemeError> {
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

    assert!(scheme.validate_u32_type(&u32::MIN.to_string()).is_none());
    assert!(scheme.validate_u32_type(&i32::MAX.to_string()).is_none());

    assert!(scheme
      .validate_u32_type(&((u32::MIN as i64) - 1).to_string())
      .is_some());
    assert!(scheme
      .validate_u32_type(&((u32::MAX as i64) + 1).to_string())
      .is_some());

    assert!(scheme.validate_u32_type("a").is_some());
    assert!(scheme.validate_u32_type("true").is_some());
    assert!(scheme.validate_u32_type(",").is_some());
    assert!(scheme.validate_u32_type("").is_some());
  }

  #[test]
  fn test_i32_validation() {
    let scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeI32);

    assert!(scheme.validate_i32_type(&i32::MIN.to_string()).is_none());
    assert!(scheme.validate_i32_type(&i32::MAX.to_string()).is_none());

    assert!(scheme
      .validate_i32_type(&((i32::MIN as i64) - 1).to_string())
      .is_some());
    assert!(scheme
      .validate_i32_type(&((i32::MAX as i64) + 1).to_string())
      .is_some());

    assert!(scheme.validate_i32_type("a").is_some());
    assert!(scheme.validate_i32_type("true").is_some());
    assert!(scheme.validate_i32_type(",").is_some());
    assert!(scheme.validate_i32_type("").is_some());
  }

  #[test]
  fn test_u16_validation() {
    let scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeU16);

    assert!(scheme.validate_u16_type(&u16::MIN.to_string()).is_none());
    assert!(scheme.validate_u16_type(&u16::MAX.to_string()).is_none());

    assert!(scheme
      .validate_u16_type(&((u16::MIN as i32) - 1).to_string())
      .is_some());
    assert!(scheme
      .validate_u16_type(&((u16::MAX as i32) + 1).to_string())
      .is_some());

    assert!(scheme.validate_u16_type("a").is_some());
    assert!(scheme.validate_u16_type("true").is_some());
    assert!(scheme.validate_u16_type(",").is_some());
    assert!(scheme.validate_u16_type("").is_some());
  }

  #[test]
  fn test_i16_validation() {
    let scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeI16);

    assert!(scheme.validate_i16_type(&i16::MIN.to_string()).is_none());
    assert!(scheme.validate_i16_type(&i16::MAX.to_string()).is_none());

    assert!(scheme
      .validate_i16_type(&((i16::MIN as i32) - 1).to_string())
      .is_some());
    assert!(scheme
      .validate_i16_type(&((i16::MAX as i32) + 1).to_string())
      .is_some());

    assert!(scheme.validate_i16_type("a").is_some());
    assert!(scheme.validate_i16_type("true").is_some());
    assert!(scheme.validate_i16_type(",").is_some());
    assert!(scheme.validate_i16_type("").is_some());
  }

  #[test]
  fn test_u8_validation() {
    let scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeU8);

    assert!(scheme.validate_u8_type(&u8::MIN.to_string()).is_none());
    assert!(scheme.validate_u8_type(&u8::MAX.to_string()).is_none());

    assert!(scheme
      .validate_u8_type(&((u8::MIN as i32) - 1).to_string())
      .is_some());
    assert!(scheme
      .validate_u8_type(&((u8::MAX as i32) + 1).to_string())
      .is_some());

    assert!(scheme.validate_u8_type("a").is_some());
    assert!(scheme.validate_u8_type("true").is_some());
    assert!(scheme.validate_u8_type(",").is_some());
    assert!(scheme.validate_u8_type("").is_some());
  }

  #[test]
  fn test_i8_validation() {
    let scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeI8);

    assert!(scheme.validate_i8_type(&i8::MIN.to_string()).is_none());
    assert!(scheme.validate_i8_type(&i8::MAX.to_string()).is_none());

    assert!(scheme
      .validate_i8_type(&((i8::MIN as i32) - 1).to_string())
      .is_some());
    assert!(scheme
      .validate_i8_type(&((i8::MAX as i32) + 1).to_string())
      .is_some());

    assert!(scheme.validate_i8_type("a").is_some());
    assert!(scheme.validate_i8_type("true").is_some());
    assert!(scheme.validate_i8_type(",").is_some());
    assert!(scheme.validate_i8_type("").is_some());
  }

  #[test]
  fn test_bool_validation() {
    let scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeBool);

    assert!(scheme.validate_bool_type("true").is_none());
    assert!(scheme.validate_bool_type("false").is_none());

    assert!(scheme.validate_bool_type("1,2,3,4").is_some());
    assert!(scheme.validate_bool_type("1").is_some());
    assert!(scheme.validate_bool_type("0").is_some());
    assert!(scheme.validate_bool_type("").is_some());
  }

  #[test]
  fn test_vector_validation() {
    let scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeVector);

    assert!(scheme.validate_vector_type("1,2,3").is_none());
    assert!(scheme.validate_vector_type("-2,2.5,-0.0025").is_none());
    assert!(scheme.validate_vector_type("-1.5,-2.5,-3.1").is_none());

    assert!(scheme.validate_vector_type("").is_some());
    assert!(scheme.validate_vector_type("1,2,3,4").is_some());
    assert!(scheme.validate_vector_type("1,2").is_some());
    assert!(scheme.validate_vector_type("-1,2.0").is_some());
    assert!(scheme.validate_vector_type("a,b,c").is_some());
  }

  #[test]
  fn test_enum_validation() {
    let mut scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeVector);

    assert!(scheme.validate_enum_type("a").is_some());

    scheme.data_type = LtxFieldDataType::TypeEnum(vec![
      String::from("a"),
      String::from("b_c"),
      String::from("d"),
    ]);

    assert!(scheme.validate_enum_type("a").is_none());
    assert!(scheme.validate_enum_type("b_c").is_none());
    assert!(scheme.validate_enum_type("d").is_none());

    assert!(scheme.validate_enum_type("").is_some());
    assert!(scheme.validate_enum_type("e").is_some());
    assert!(scheme.validate_enum_type("f").is_some());
    assert!(scheme.validate_enum_type("1").is_some());
  }

  #[test]
  fn test_tuple_validation() {
    let mut scheme: LtxFieldScheme =
      LtxFieldScheme::new_with_type("test_section", "test_field", LtxFieldDataType::TypeVector);

    assert!(scheme.validate_enum_type("a").is_some());

    scheme.data_type = LtxFieldDataType::TypeTuple(vec![
      LtxFieldDataType::TypeF32,
      LtxFieldDataType::TypeString,
      LtxFieldDataType::TypeBool,
    ]);

    assert!(scheme.validate_tuple_type("15.25, abc, true").is_none());
    assert!(scheme.validate_tuple_type("-12.50, def, false").is_none());
    assert!(scheme.validate_tuple_type("0, a, true").is_none());

    assert!(scheme.validate_tuple_type("").is_some());
    assert!(scheme.validate_tuple_type("e").is_some());
    assert!(scheme.validate_tuple_type("f").is_some());
    assert!(scheme.validate_tuple_type("1").is_some());
    assert!(scheme.validate_tuple_type("10,,true_true").is_some());
    assert!(scheme.validate_tuple_type("10,,1").is_some());
    assert!(scheme.validate_tuple_type("a,b,c").is_some());
    assert!(scheme.validate_tuple_type("a,b,c,d").is_some());
  }
}
