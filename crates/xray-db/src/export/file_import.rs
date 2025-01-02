use crate::error::database_parse_error::DatabaseParseError;
use crate::types::{DatabaseResult, U32Bytes};
use std::str::FromStr;
use xray_ltx::Section;

/// Export ini file content to provided file.
pub fn import_vector_from_string<T: FromStr>(value: &str) -> DatabaseResult<Vec<T>> {
  let mut vector: Vec<T> = Vec::new();

  if value.trim().is_empty() {
    return Ok(vector);
  }

  for it in value.split(',') {
    vector.push(match it.trim().parse::<T>() {
      Ok(v) => v,
      _ => {
        return Err(DatabaseParseError::new_database_error(
          "Failed to parse vector from string value",
        ))
      }
    });
  }

  Ok(vector)
}

/// Export ini file content to provided file.
pub fn import_sized_vector_from_string<T: FromStr>(
  size: usize,
  value: &str,
) -> DatabaseResult<Vec<T>> {
  let mut vector: Vec<T> = Vec::new();

  for (index, it) in value.split(',').enumerate() {
    vector.push(match it.trim().parse::<T>() {
      Ok(v) => v,
      _ => {
        return Err(DatabaseParseError::new_database_error(
          "Failed to parse sized vector from string",
        ))
      }
    });

    if index >= size {
      return Err(DatabaseParseError::new_database_error(
        "Failed to parse sized vector from string, it has more elements than required",
      ));
    }
  }

  if vector.len() != size {
    return Err(DatabaseParseError::new_database_error(
      "Failed to parse sized vector from string, it has less elements than required",
    ));
  }

  Ok(vector)
}

/// Read value from ini section and parse it as provided T type.
pub fn read_ini_field<T: FromStr>(field_name: &str, section: &Section) -> DatabaseResult<T> {
  Ok(
    match section
      .get(field_name)
      .ok_or_else(|| {
        DatabaseParseError::new_database_error(format!(
          "Field '{field_name}' was not found in ini file"
        ))
      })?
      .parse::<T>()
    {
      Ok(value) => value,
      _ => {
        return Err(DatabaseParseError::new_database_error(format!(
          "Failed to parse ini field '{field_name}' value, valid {:?} is expected",
          std::any::type_name::<T>()
        )))
      }
    },
  )
}

/// Read optional value from ini section and parse it as provided T type.
pub fn read_ini_optional_field<T: FromStr>(
  field_name: &str,
  section: &Section,
) -> DatabaseResult<Option<T>> {
  let field_data: Option<&str> = section.get(field_name);

  Ok(match field_data {
    Some(value) => match value.parse::<T>() {
      Ok(parsed) => Some(parsed),
      _ => {
        return Err(DatabaseParseError::new_database_error(format!(
          "Failed to parse optional ini field '{field_name}' value, correct {:?} is expected",
          std::any::type_name::<T>()
        )))
      }
    },
    None => None,
  })
}

/// Read value from ini section and parse it as provided T type.
pub fn read_ini_u32_bytes_field(field: &str, section: &Section) -> DatabaseResult<U32Bytes> {
  let vertex_type: Vec<u8> =
    import_sized_vector_from_string(4, &read_ini_field::<String>(field, section)?)?;

  Ok((
    *vertex_type.get(0).unwrap(),
    *vertex_type.get(1).unwrap(),
    *vertex_type.get(2).unwrap(),
    *vertex_type.get(3).unwrap(),
  ))
}
