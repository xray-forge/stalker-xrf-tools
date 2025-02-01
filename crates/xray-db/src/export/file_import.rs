use std::str::FromStr;
use xray_error::{XRayError, XRayResult};
use xray_ltx::Section;

/// Read value from ltx section and parse it as provided T type.
pub fn read_ltx_field<T: FromStr>(field_name: &str, section: &Section) -> XRayResult<T> {
  Ok(
    match section
      .get(field_name)
      .ok_or_else(|| {
        XRayError::new_parsing_error(format!("Field '{field_name}' was not found in ltx file"))
      })?
      .parse::<T>()
    {
      Ok(value) => value,
      _ => {
        return Err(XRayError::new_parsing_error(format!(
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
) -> XRayResult<Option<T>> {
  let field_data: Option<&str> = section.get(field_name);

  Ok(match field_data {
    Some(value) => match value.parse::<T>() {
      Ok(parsed) => Some(parsed),
      _ => {
        return Err(XRayError::new_parsing_error(format!(
          "Failed to parse optional ltx field '{}' value, correct {:?} is expected",
          field_name,
          std::any::type_name::<T>()
        )))
      }
    },
    None => None,
  })
}
