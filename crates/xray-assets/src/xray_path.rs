/// Appends an extension to an X-Ray logical path when it is not already present.
pub fn with_extension(path: &str, extension: &str) -> String {
  if path.ends_with(extension) { path.to_string() } else { format!("{path}{extension}") }
}
