use crate::{LtxError, LtxReadError, LtxSchemeError};

#[derive(Clone, Debug, PartialEq)]
pub enum LtxFieldDataType {
  TypeString,
  TypeSection,
  TypeTuple(Vec<LtxFieldDataType>),
  TypeCondlist,
  TypeF32,
  TypeU32,
  TypeI32,
  TypeU16,
  TypeI16,
  TypeU8,
  TypeI8,
  TypeBool,
  TypeVector,
  TypeEnum(Vec<String>),
  TypeUnknown,
  TypeAny,
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
    Ok(match data {
      "f32" => LtxFieldDataType::TypeF32,
      "u32" => LtxFieldDataType::TypeU32,
      "i32" => LtxFieldDataType::TypeI32,
      "u16" => LtxFieldDataType::TypeU16,
      "i16" => LtxFieldDataType::TypeI16,
      "u8" => LtxFieldDataType::TypeU8,
      "i8" => LtxFieldDataType::TypeI8,
      "string" => LtxFieldDataType::TypeString,
      "condlist" => LtxFieldDataType::TypeCondlist,
      "vector" => LtxFieldDataType::TypeVector,
      "section" => LtxFieldDataType::TypeSection,
      "bool" => LtxFieldDataType::TypeBool,
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

  /// Check if provided field data enables type optional mode.
  pub fn is_field_optional(data: Option<&str>) -> bool {
    if let Some(data) = data {
      data == "true"
    } else {
      false
    }
  }
}
