use crate::types::U32Bytes;
use std::io;
use std::str::FromStr;
use xray_ltx::Properties;

/// Export ini file content to provided file.
pub fn import_vector_from_string<T: FromStr>(value: &str) -> io::Result<Vec<T>> {
  let mut vector: Vec<T> = Vec::new();

  if value.trim().is_empty() {
    return Ok(vector);
  }

  for it in value.split(',') {
    vector.push(match it.trim().parse::<T>() {
      Ok(v) => v,
      _ => {
        return Err(io::Error::new(
          io::ErrorKind::InvalidInput,
          String::from("Failed to parse vector from string"),
        ))
      }
    });
  }

  Ok(vector)
}

/// Export ini file content to provided file.
pub fn import_sized_vector_from_string<T: FromStr>(size: usize, value: &str) -> io::Result<Vec<T>> {
  let mut vector: Vec<T> = Vec::new();

  for (index, it) in value.split(',').enumerate() {
    vector.push(match it.trim().parse::<T>() {
      Ok(v) => v,
      _ => {
        return Err(io::Error::new(
          io::ErrorKind::InvalidInput,
          String::from("Failed to parse sized vector from string"),
        ))
      }
    });

    if index >= size {
      return Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        String::from(
          "Failed to parse sized vector from string, it has more elements than required",
        ),
      ));
    }
  }

  if vector.len() != size {
    return Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      String::from("Failed to parse sized vector from string, it has less elements than required"),
    ));
  }

  Ok(vector)
}

/// Read value from ini section and parse it as provided T type.
pub fn read_ini_field<T: FromStr>(field: &str, props: &Properties) -> io::Result<T> {
  Ok(
    match props
      .get(field)
      .unwrap_or_else(|| panic!("'{field}' to be in ini"))
      .parse::<T>()
    {
      Ok(value) => value,

      _ => {
        return Err(io::Error::new(
          io::ErrorKind::InvalidInput,
          format!(
            "Failed to parse ini field '{field}' value, valid {:?} is expected",
            std::any::type_name::<T>()
          ),
        ))
      }
    },
  )
}

/// Read value from ini section and parse it as provided T type.
pub fn read_ini_u32_bytes_field(field: &str, props: &Properties) -> io::Result<U32Bytes> {
  let vertex_type: Vec<u8> =
    import_sized_vector_from_string(4, &read_ini_field::<String>(field, props)?)?;

  Ok((
    *vertex_type.get(0).unwrap(),
    *vertex_type.get(1).unwrap(),
    *vertex_type.get(2).unwrap(),
    *vertex_type.get(3).unwrap(),
  ))
}
