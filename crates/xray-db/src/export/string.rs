use base64::engine::{general_purpose, GeneralPurpose};
use base64::{alphabet, Engine};
use xray_error::{XRayError, XRayResult};

pub const CUSTOM_B64_ENGINE: GeneralPurpose =
  GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

/// Encode bytes as b64.
pub fn bytes_to_base64(buffer: &[u8]) -> String {
  CUSTOM_B64_ENGINE.encode(buffer)
}

/// Encode bytes as b64.
pub fn string_to_base64(string: &str) -> String {
  CUSTOM_B64_ENGINE.encode(string)
}

/// Encode bytes as b64.
pub fn bytes_from_base64(string: &str) -> XRayResult<Vec<u8>> {
  Ok(match CUSTOM_B64_ENGINE.decode(string) {
    Ok(value) => value,
    Err(_) => {
      return Err(XRayError::new_parsing_error(
        "Failed to decode string value from base 64",
      ))
    }
  })
}

/// Encode bytes as b64.
pub fn string_from_base64(string: &str) -> XRayResult<String> {
  Ok(match CUSTOM_B64_ENGINE.decode(string) {
    Ok(value) => String::from_utf8_lossy(&value).into_owned(),
    Err(_) => {
      return Err(XRayError::new_parsing_error(
        "Failed to decode string value from base 64",
      ))
    }
  })
}
