use crate::file::configuration::constants::{LTX_SYMBOL_ARRAY, LTX_SYMBOL_OPTIONAL};
use crate::{LtxError, LtxReadError, LtxSchemeError};

#[derive(Clone, Debug, PartialEq)]
pub enum LtxFieldDataType {
  TypeAny,
  TypeBool,
  TypeCondlist,
  TypeEnum(Vec<String>),
  TypeF32,
  TypeI16,
  TypeI32,
  TypeI8,
  TypeRgb,
  TypeRgba,
  TypeSection,
  TypeString,
  TypeTuple(Vec<LtxFieldDataType>),
  TypeU16,
  TypeU32,
  TypeU8,
  TypeUnknown,
  TypeVector,
}

impl LtxFieldDataType {
  pub fn parse_enum(
    field_name: &str,
    section_name: &str,
    value: &str,
  ) -> Result<LtxFieldDataType, LtxError> {
    let mut allowed_values: Vec<String> = Vec::new();

    match value.split_once(':') {
      None => {
        return Err(LtxReadError::new_ltx_error(format!(
          "Failed to read scheme enum type for field '{section_name}', expected ':' separated type and values"
        )))
      }
      Some((_, allowed_values_string)) => {
        for allowed in allowed_values_string.trim().split(',').filter_map(|it| {
          let trimmed: &str = it.trim();

          if trimmed.is_empty() {
            None
          } else {
            Some(trimmed)
          }
        }) {
          allowed_values.push(allowed.into());
        }
      }
    }

    if allowed_values.is_empty() {
      Err(LtxSchemeError::new_ltx_error(
        section_name,
        field_name,
        "Failed to parse enum type, expected comma separated list of possible values after 'enum:'",
      ))
    } else {
      Ok(LtxFieldDataType::TypeEnum(allowed_values))
    }
  }

  pub fn parse_tuple(
    field_name: &str,
    section_name: &str,
    value: &str,
  ) -> Result<LtxFieldDataType, LtxError> {
    let mut types: Vec<LtxFieldDataType> = Vec::new();

    match value.split_once(':') {
      None => {
        return Err(LtxReadError::new_ltx_error(format!(
        "Failed to read scheme tuple type for field '{section_name}', expected ':' separated types"
      )))
      }
      Some((_, allowed_values_string)) => {
        for tuple_entry in allowed_values_string.trim().split(',').filter_map(|it| {
          let trimmed: &str = it.trim();

          if trimmed.is_empty() {
            None
          } else {
            Some(Self::from_field_data(field_name, section_name, trimmed))
          }
        }) {
          let schema: LtxFieldDataType = tuple_entry?;

          match schema {
            LtxFieldDataType::TypeTuple(_) => {
              return Err(LtxReadError::new_ltx_error(format!(
                "Failed to read scheme for field '{section_name}', tuple cannot contain nested tuples"
              )))
            }
            _ => types.push(schema),
          }
        }
      }
    }

    if types.is_empty() {
      Err(LtxSchemeError::new_ltx_error(
        section_name,
        field_name,
        "Failed to parse tuple type, expected comma separated list of possible values after 'tuple:'",
      ))
    } else {
      Ok(LtxFieldDataType::TypeTuple(types))
    }
  }

  /// Parse data type enum variant from provided string option.
  pub fn from_field_data(
    field_name: &str,
    section_name: &str,
    data: &str,
  ) -> Result<LtxFieldDataType, LtxError> {
    let mut data: &str = data;

    // Respect optionals.
    if data.starts_with(LTX_SYMBOL_OPTIONAL) {
      data = &data[1..];
    }

    // Respect arrays.
    if data.ends_with(LTX_SYMBOL_ARRAY) {
      data = &data[0..(data.len() - 2)];
    }

    Ok(match data {
      "bool" => LtxFieldDataType::TypeBool,
      "condlist" => LtxFieldDataType::TypeCondlist,
      "f32" => LtxFieldDataType::TypeF32,
      "i16" => LtxFieldDataType::TypeI16,
      "i32" => LtxFieldDataType::TypeI32,
      "i8" => LtxFieldDataType::TypeI8,
      "rgb" => LtxFieldDataType::TypeRgb,
      "rgba" => LtxFieldDataType::TypeRgba,
      "section" => LtxFieldDataType::TypeSection,
      "string" => LtxFieldDataType::TypeString,
      "u16" => LtxFieldDataType::TypeU16,
      "u32" => LtxFieldDataType::TypeU32,
      "u8" => LtxFieldDataType::TypeU8,
      "vector" => LtxFieldDataType::TypeVector,
      field_type => {
        if field_type.starts_with("enum") {
          LtxFieldDataType::parse_enum(field_name, section_name, data)?
        } else if field_type.starts_with("tuple") {
          LtxFieldDataType::parse_tuple(field_name, section_name, data)?
        } else {
          LtxFieldDataType::TypeUnknown
        }
      }
    })
  }

  /// Parse data type enum variant from provided string option.
  pub fn from_field_data_optional(
    field_name: &str,
    section_name: &str,
    data: Option<&str>,
  ) -> Result<LtxFieldDataType, LtxError> {
    if let Some(data) = data {
      Self::from_field_data(field_name, section_name, data)
    } else {
      Ok(LtxFieldDataType::TypeAny)
    }
  }

  /// Parse data array type from value.
  pub fn is_field_data_array(data: Option<&str>) -> bool {
    if let Some(data) = data {
      data.ends_with(LTX_SYMBOL_ARRAY)
    } else {
      false
    }
  }

  /// Parse data optional type from value.
  pub fn is_field_data_optional(data: Option<&str>) -> bool {
    if let Some(data) = data {
      data.starts_with(LTX_SYMBOL_OPTIONAL)
    } else {
      false
    }
  }

  /// Check if provided field data is declared as valid optional boolean and is true.
  pub fn is_optional_bool_field_declared(
    field_name: &str,
    section_name: &str,
    data: Option<&str>,
  ) -> Result<bool, LtxError> {
    if let Some(data) = data {
      Self::is_bool_field_declared(field_name, section_name, data)
    } else {
      Ok(false)
    }
  }

  /// Check if provided field data is declared as valid boolean and is true.
  pub fn is_bool_field_declared(
    field_name: &str,
    section_name: &str,
    data: &str,
  ) -> Result<bool, LtxError> {
    if let Ok(parsed) = data.parse::<bool>() {
      Ok(parsed)
    } else {
      Err(LtxReadError::new_ltx_error(format!(
        "Invalid ltx [{section_name}] {field_name} configuration, invalid value supplied.\
           Expected 'true' or 'false', got '{}'",
        data
      )))
    }
  }
}
