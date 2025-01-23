use crate::{LtxError, LtxResult};
use encoding_rs::WINDOWS_1251;
use std::io::Read;

pub fn read_data_as_string<R: Read>(reader: &mut R) -> LtxResult<String> {
  let mut raw_data: Vec<u8> = Vec::new();
  let raw_data_read: usize = reader.read_to_end(&mut raw_data)?;

  assert_eq!(
    raw_data_read,
    raw_data.len(),
    "Expected raw data size to match in-memory buffer"
  );

  let (cow, encoding_used, had_errors) = WINDOWS_1251.decode(&raw_data);

  if had_errors {
    Err(LtxError::new_read_error(format!(
      "Failed to decode LTX file data from reader with {:?} encoding, {} bytes",
      encoding_used,
      raw_data.len()
    )))
  } else {
    Ok(cow.to_string())
  }
}
