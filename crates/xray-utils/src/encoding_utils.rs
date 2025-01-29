use encoding_rs::WINDOWS_1251;
use std::borrow::Cow;
use std::io;
use std::io::ErrorKind;

/// Try encoding provided u8 raw bytes as window 1251 string and convert into regular rust string.
pub fn encode_windows1251_bytes_to_string(bytes: &[u8]) -> io::Result<String> {
  let (cow, _, had_errors) = WINDOWS_1251.decode(bytes);

  if had_errors {
    Err(io::Error::new(
      ErrorKind::InvalidData,
      format!(
        "Failed to decode buffer data with WINDOWS_1251 encoding, {} bytes",
        bytes.len()
      ),
    ))
  } else {
    Ok(cow.to_string())
  }
}

/// Try encoding provided string as windows1251 bytes.
pub fn encode_string_to_windows1251_bytes(string: &str) -> io::Result<Vec<u8>> {
  let (transformed, _, had_errors) = WINDOWS_1251.encode(string);

  if had_errors {
    Err(io::Error::new(
      ErrorKind::InvalidData,
      format!(
        "Failed to encode string with WINDOWS_1251 encoding, {} characters",
        string.len()
      ),
    ))
  } else {
    Ok(match transformed {
      Cow::Borrowed(value) => value.to_vec(),
      Cow::Owned(value) => value,
    })
  }
}
