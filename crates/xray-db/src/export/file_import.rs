use crate::error::DatabaseError;
use crate::types::{DatabaseResult, U32Bytes};
use std::str::FromStr;
use xray_ltx::Section;

/// Export ltx file content to provided file.
pub fn import_vector_from_string<T: FromStr>(value: &str) -> DatabaseResult<Vec<T>> {
  let mut vector: Vec<T> = Vec::new();

  if value.trim().is_empty() {
    return Ok(vector);
  }

  for it in value.split(',') {
    vector.push(match it.trim().parse::<T>() {
      Ok(v) => v,
      _ => {
        return Err(DatabaseError::new_parse_error(
          "Failed to parse vector from string value",
        ))
      }
    });
  }

  Ok(vector)
}

/// Export ltx file content to provided file.
pub fn import_sized_vector_from_string<T: FromStr>(
  size: usize,
  value: &str,
) -> DatabaseResult<Vec<T>> {
  let mut vector: Vec<T> = Vec::new();

  for (index, it) in value.split(',').enumerate() {
    vector.push(match it.trim().parse::<T>() {
      Ok(v) => v,
      _ => {
        return Err(DatabaseError::new_parse_error(
          "Failed to parse sized vector from string",
        ))
      }
    });

    if index >= size {
      return Err(DatabaseError::new_parse_error(
        "Failed to parse sized vector from string, it has more elements than required",
      ));
    }
  }

  if vector.len() != size {
    return Err(DatabaseError::new_parse_error(
      "Failed to parse sized vector from string, it has less elements than required",
    ));
  }

  Ok(vector)
}

/// Read value from ltx section and parse it as provided T type.
pub fn read_ltx_field<T: FromStr>(field_name: &str, section: &Section) -> DatabaseResult<T> {
  Ok(
    match section
      .get(field_name)
      .ok_or_else(|| {
        DatabaseError::new_parse_error(format!("Field '{field_name}' was not found in ltx file"))
      })?
      .parse::<T>()
    {
      Ok(value) => value,
      _ => {
        return Err(DatabaseError::new_parse_error(format!(
          "Failed to parse ltx field '{}' value, valid {} is expected",
          field_name,
          std::any::type_name::<T>()
        )))
      }
    },
  )
}

/// Read optional value from ltx section and parse it as provided T type.
pub fn read_ini_optional_field<T: FromStr>(
  field_name: &str,
  section: &Section,
) -> DatabaseResult<Option<T>> {
  let field_data: Option<&str> = section.get(field_name);

  Ok(match field_data {
    Some(value) => match value.parse::<T>() {
      Ok(parsed) => Some(parsed),
      _ => {
        return Err(DatabaseError::new_parse_error(format!(
          "Failed to parse optional ltx field '{}' value, correct {:?} is expected",
          field_name,
          std::any::type_name::<T>()
        )))
      }
    },
    None => None,
  })
}

/// Read value from ltx section and parse it as provided T type.
pub fn read_ini_u32_bytes_field(field: &str, section: &Section) -> DatabaseResult<U32Bytes> {
  let vertex_type: Vec<u8> =
    import_sized_vector_from_string(4, &read_ltx_field::<String>(field, section)?)?;

  Ok((
    *vertex_type.get(0).unwrap(),
    *vertex_type.get(1).unwrap(),
    *vertex_type.get(2).unwrap(),
    *vertex_type.get(3).unwrap(),
  ))
}
