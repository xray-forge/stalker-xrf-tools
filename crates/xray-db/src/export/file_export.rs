use std::fmt::Display;

/// Export ltx file content to provided file.
pub fn export_vector_to_string<T: Display>(vector: &[T]) -> String {
  vector
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<_>>()
    .join(",")
}
