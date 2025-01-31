use std::fmt::Display;

pub fn export_vector_to_string<T: Display>(vector: &[T]) -> String {
  vector
    .iter()
    .map(|x| x.to_string())
    .collect::<Vec<_>>()
    .join(",")
}
