use std::fmt::Display;

/// Export ini file content to provided file.
pub fn export_vector_to_string<T: Display>(vector: &Vec<T>) -> String {
  vector
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<_>>()
    .join(",")
}
