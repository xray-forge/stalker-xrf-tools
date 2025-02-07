use std::path::Path;

/// Stringify list of paths for better display in logs / debug info / information printing.
#[inline]
pub fn path_vec_to_string<T: AsRef<Path>>(paths: &[T]) -> String {
  path_vec_to_string_sep(paths, ", ")
}

/// Stringify list of paths for better display in logs / debug info / information printing.
#[inline]
pub fn path_vec_to_string_sep<T: AsRef<Path>>(paths: &[T], separator: &str) -> String {
  paths
    .iter()
    .map(|it| it.as_ref().display().to_string())
    .collect::<Vec<_>>()
    .join(separator)
}
