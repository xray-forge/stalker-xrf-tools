/// Appends an extension to an X-Ray logical path when it is not already present.
pub fn with_extension(path: &str, extension: &str) -> String {
  if path.ends_with(extension) {
    path.to_string()
  } else {
    format!("{path}{extension}")
  }
}

#[cfg(test)]
mod tests {
  use super::with_extension;

  #[test]
  fn preserves_an_existing_extension() {
    assert_eq!(with_extension("actors\\stalker.ogf", ".ogf"), "actors\\stalker.ogf");
  }

  #[test]
  fn appends_a_missing_extension() {
    assert_eq!(with_extension("actors\\stalker", ".ogf"), "actors\\stalker.ogf");
  }
}
