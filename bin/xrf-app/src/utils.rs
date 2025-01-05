use std::error::Error;

/// Stringify provided error to simplify tauri error casting.
pub fn error_to_string<T: Error>(error: T) -> String {
  error.to_string()
}
