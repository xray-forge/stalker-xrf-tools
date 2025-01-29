use crate::{ChunkResult, ChunkWriter, U32Bytes};
use byteorder::{ByteOrder, WriteBytesExt};
use std::io::Write;
use xray_utils::encode_string_to_windows1251_bytes;

impl ChunkWriter {
  /// Write null terminated windows1251 encoded string.
  pub fn write_null_terminated_win_string(&mut self, data: &str) -> ChunkResult<usize> {
    Ok(self.write(&encode_string_to_windows1251_bytes(data)?)? + self.write(&[0u8])?)
  }

  /// Write 4 bytes value as 4 separate byte entries.
  /// Preserves write/read order for 4 values, not dependent on ByteOrder.
  pub fn write_u32_bytes(&mut self, data: &U32Bytes) -> ChunkResult<usize> {
    self.write_u8(data.0)?;
    self.write_u8(data.1)?;
    self.write_u8(data.2)?;
    self.write_u8(data.3)?;

    Ok(size_of::<U32Bytes>())
  }

  /// Write serialized vector into vector, where u32 count N is followed by N u16 entries.
  pub fn write_u16_vector<T: ByteOrder>(&mut self, data: &[u16]) -> ChunkResult<usize> {
    self.write_u32::<T>(data.len() as u32)?;

    for it in data {
      self.write_u16::<T>(*it)?;
    }

    Ok(size_of::<u32>() + size_of_val(data))
  }
}

#[cfg(test)]
mod tests {
  use crate::types::ChunkResult;
  use crate::{ChunkWriter, XRayByteOrder};

  #[test]
  fn test_write_null_terminated_string_empty() -> ChunkResult {
    let mut writer: ChunkWriter = ChunkWriter::new();

    assert_eq!(
      writer.write_null_terminated_win_string("")?,
      1,
      "Expect 1 byte written"
    );
    assert_eq!(writer.buffer, [0], "Expect null terminated empty written");
    assert_eq!(writer.bytes_written(), 1, "Expect 1 byte written");

    Ok(())
  }

  #[test]
  fn test_write_null_terminated_string_sample() -> ChunkResult {
    let mut writer: ChunkWriter = ChunkWriter::new();

    assert_eq!(
      writer.write_null_terminated_win_string("abc")?,
      4,
      "Expect 4 bytes written"
    );
    assert_eq!(
      writer.buffer,
      [b'a', b'b', b'c', 0],
      "Expect null terminated string written"
    );
    assert_eq!(writer.bytes_written(), 4, "Expect 4 bytes written");

    Ok(())
  }

  #[test]
  fn test_write_u32_bytes() -> ChunkResult {
    let mut writer: ChunkWriter = ChunkWriter::new();

    assert_eq!(
      writer.write_u32_bytes(&(0u8, 1u8, 2u8, 3u8))?,
      4,
      "Expect 4 bytes written"
    );
    assert_eq!(writer.buffer, [0, 1, 2, 3], "Expect correct written data");
    assert_eq!(writer.bytes_written(), 4, "Expect 4 bytes written");

    Ok(())
  }

  #[test]
  fn test_write_u16_vector_empty() -> ChunkResult {
    let mut writer: ChunkWriter = ChunkWriter::new();

    assert_eq!(
      writer.write_u16_vector::<XRayByteOrder>(&[])?,
      4,
      "Expect 4 bytes written"
    );
    assert_eq!(writer.buffer, [0, 0, 0, 0], "Expect correct written data");
    assert_eq!(
      writer.bytes_written(),
      4,
      "Expect 4 bytes written with empty vector"
    );

    Ok(())
  }

  #[test]
  fn test_write_u16_vector_samples() -> ChunkResult {
    let mut writer: ChunkWriter = ChunkWriter::new();

    assert_eq!(
      writer.write_u16_vector::<XRayByteOrder>(&[1, 2, 3, 4])?,
      12,
      "Expect 12 bytes written"
    );
    assert_eq!(
      writer.buffer,
      [4, 0, 0, 0, 1, 0, 2, 0, 3, 0, 4, 0],
      "Expect correct written data"
    );
    assert_eq!(
      writer.bytes_written(),
      12,
      "Expect 12 bytes written with empty vector"
    );

    Ok(())
  }
}
