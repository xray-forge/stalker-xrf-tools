use encoding_rs::{Encoding, UTF_8};
use encoding_rs::{WINDOWS_1250, WINDOWS_1251};
use std::borrow::Cow;
use std::io;
use std::io::{ErrorKind, Read};

pub type XRayEncoding = &'static Encoding;

/// Return encoding factory for windows1250.
pub fn get_windows1250_encoder() -> XRayEncoding {
  WINDOWS_1250
}

/// Return encoding factory for windows1251.
pub fn get_windows1251_encoder() -> XRayEncoding {
  WINDOWS_1251
}

/// Return encoding factory for UTF-8.
pub fn get_utf8_encoder() -> XRayEncoding {
  UTF_8
}

/// Try encoding provided u8 raw bytes as string value.
pub fn decode_bytes_to_string(bytes: &[u8], encoding: XRayEncoding) -> io::Result<String> {
  let (cow, _, had_errors) = encoding.decode(bytes);

  if had_errors {
    Err(io::Error::new(
      ErrorKind::InvalidData,
      format!(
        "Failed to decode buffer data with {:?} encoding, {} bytes",
        encoding,
        bytes.len()
      ),
    ))
  } else {
    Ok(cow.to_string())
  }
}

/// Try encoding provided u8 raw bytes as string value.
pub fn decode_bytes_to_string_without_bom_handling(
  bytes: &[u8],
  encoding: XRayEncoding,
) -> io::Result<String> {
  let (cow, had_errors) = encoding.decode_without_bom_handling(bytes);

  if had_errors {
    Err(io::Error::new(
      ErrorKind::InvalidData,
      format!(
        "Failed to decode (no bom handling) buffer data with {:?} encoding, {} bytes",
        encoding,
        bytes.len()
      ),
    ))
  } else {
    Ok(cow.to_string())
  }
}

/// Try encoding provided string value as bytes.
pub fn encode_string_to_bytes(string: &str, encoding: XRayEncoding) -> io::Result<Vec<u8>> {
  let (transformed, _, had_errors) = encoding.encode(string);

  if had_errors {
    Err(io::Error::new(
      ErrorKind::InvalidData,
      format!(
        "Failed to encode string with {:?} encoding, {} characters",
        encoding,
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

/// Read whole encoded reader content to a string.
pub fn read_as_string_from_encoded<R: Read>(
  reader: &mut R,
  encoding: XRayEncoding,
) -> io::Result<String> {
  let mut raw_data: Vec<u8> = Vec::new();

  reader.read_to_end(&mut raw_data)?;

  decode_bytes_to_string(&raw_data, encoding)
}

/// Try encoding provided u8 raw bytes as window 1251 string and convert into regular rust string.
pub fn read_as_string_from_windows1251_encoded<R: Read>(reader: &mut R) -> io::Result<String> {
  read_as_string_from_encoded(reader, WINDOWS_1251)
}

/// Try encoding provided u8 raw bytes as window 1251 string and convert into regular rust string.
pub fn encode_windows1251_bytes_to_string(bytes: &[u8]) -> io::Result<String> {
  decode_bytes_to_string(bytes, WINDOWS_1251)
}

/// Try encoding provided string as windows1251 bytes.
pub fn encode_string_to_windows1251_bytes(string: &str) -> io::Result<Vec<u8>> {
  encode_string_to_bytes(string, WINDOWS_1251)
}
