#[derive(Clone, Debug, PartialEq)]
pub enum LtxFieldDataType {
  TypeString,
  TypeF32,
  TypeU32,
  TypeI32,
  TypeU16,
  TypeI16,
  TypeUnknown,
  TypeAny,
}

impl LtxFieldDataType {
  /// Parse data type enum variant from provided string option.
  pub fn from_field_data(data: &str) -> LtxFieldDataType {
    match data {
      "string" => LtxFieldDataType::TypeString,
      "f32" => LtxFieldDataType::TypeF32,
      "u32" => LtxFieldDataType::TypeU32,
      "i32" => LtxFieldDataType::TypeI32,
      "u16" => LtxFieldDataType::TypeU16,
      "i16" => LtxFieldDataType::TypeI16,
      _ => LtxFieldDataType::TypeUnknown,
    }
  }

  /// Parse data type enum variant from provided string option.
  pub fn from_field_data_optional(data: Option<&str>) -> LtxFieldDataType {
    if let Some(data) = data {
      Self::from_field_data(data)
    } else {
      LtxFieldDataType::TypeAny
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
