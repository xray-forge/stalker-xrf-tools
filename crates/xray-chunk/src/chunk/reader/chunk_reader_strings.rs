use crate::chunk::source::chunk_data_source::ChunkDataSource;
use crate::ChunkReader;
use std::io::{Read, SeekFrom};
use xray_error::{XRayError, XRayResult};
use xray_utils::encode_windows1251_bytes_to_string;

const STRING_READ_BUFFER_SIZE: usize = 256;

impl<D: ChunkDataSource> ChunkReader<D> {
  /// Read null terminated windows encoded string from file bytes.
  pub fn read_null_terminated_win_string(&mut self) -> XRayResult<String> {
    self.read_null_terminated_win_string_limited(10_240)
  }

  /// Read null terminated windows encoded string from file bytes with size limit.
  pub fn read_null_terminated_win_string_limited(&mut self, limit: usize) -> XRayResult<String> {
    let mut buffer: [u8; STRING_READ_BUFFER_SIZE] = [0u8; STRING_READ_BUFFER_SIZE];
    let mut collected: Vec<u8> = Vec::new();

    loop {
      let bytes_read: usize = self.read(&mut buffer)?;

      if collected.len() + bytes_read > limit {
        return Err(XRayError::new_parsing_error(
          "Cannot parse string, reading data over buffer size limit",
        ));
      }

      if bytes_read == 0 {
        return Err(XRayError::new_no_terminator_error(
          "Null terminator is not found in buffer, no data to be read",
        ));
      }

      if let Some(position) = buffer[..bytes_read].iter().position(|&it| it == 0) {
        let extra_bytes: i64 = (bytes_read - position - 1) as i64;

        collected.extend_from_slice(&buffer[..position]);
        self.source.set_seek(SeekFrom::Current(-extra_bytes))?;

        break;
      } else {
        collected.extend_from_slice(&buffer[..bytes_read]);
      }
    }

    Ok(encode_windows1251_bytes_to_string(&collected)?)
  }

  /// Read \r\n terminated windows encoded string from file bytes.
  #[inline(never)]
  pub fn read_rn_terminated_win_string(&mut self) -> XRayResult<String> {
    self.read_rn_terminated_win_string_limited(10_240)
  }

  /// Read \r\n terminated windows encoded string from file bytes.
  #[inline(never)]
  pub fn read_rn_terminated_win_string_limited(&mut self, limit: usize) -> XRayResult<String> {
    let mut buffer: [u8; STRING_READ_BUFFER_SIZE] = [0u8; STRING_READ_BUFFER_SIZE];
    let mut collected: Vec<u8> = Vec::new();

    loop {
      let bytes_read: usize = self.read(&mut buffer)?;

      if collected.len() + bytes_read > limit {
        return Err(XRayError::new_parsing_error(
          "Cannot parse string, reading data over buffer size limit",
        ));
      }

      if bytes_read == 0 {
        return Err(XRayError::new_no_terminator_error(
          "RN sequence is not found in buffer, no data to be read",
        ));
      }

      if let Some(position) = buffer[..bytes_read].windows(2).position(|it| it == b"\r\n") {
        let extra_bytes: i64 = (bytes_read - (position + 2)) as i64;

        collected.extend_from_slice(&buffer[..position]);
        self.source.set_seek(SeekFrom::Current(-extra_bytes))?;

        break;
      } else {
        collected.extend_from_slice(&buffer[..bytes_read]);
      }
    }

    Ok(encode_windows1251_bytes_to_string(&collected)?)
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::chunk_reader::ChunkReader;
  use crate::chunk::source::chunk_memory_source::InMemoryChunkDataSource;
  use xray_error::XRayResult;

  #[test]
  fn test_read_null_terminated_string_empty() -> XRayResult {
    let mut chunk: ChunkReader<InMemoryChunkDataSource> = ChunkReader::from_bytes(&[])?;

    assert_eq!(chunk.read_bytes_remain(), 0, "Expect 0 bytes remaining");
    assert_eq!(chunk.cursor_pos(), 0, "Expect 0 bytes read");

    assert_eq!(
      chunk
        .read_null_terminated_win_string()
        .unwrap_err()
        .to_string(),
      "Missing terminator error: Null terminator is not found in buffer, no data to be read",
      "Expect error on empty read"
    );
    assert_eq!(chunk.cursor_pos(), 0, "Expect 0 bytes read");

    Ok(())
  }

  #[test]
  fn test_read_null_terminated_string_empty_null() -> XRayResult {
    let mut chunk: ChunkReader<InMemoryChunkDataSource> = ChunkReader::from_bytes(&[0])?;

    assert_eq!(chunk.cursor_pos(), 0, "Expect 0 bytes read");
    assert_eq!(chunk.read_bytes_remain(), 1, "Expect 1 byte remaining");

    assert_eq!(
      chunk.read_null_terminated_win_string()?,
      "",
      "Expect empty string with terminator"
    );
    assert_eq!(chunk.cursor_pos(), 1, "Expect 1 byte read");
    assert_eq!(chunk.read_bytes_remain(), 0, "Expect 0 bytes remaining");

    Ok(())
  }

  #[test]
  fn test_read_null_terminated_string_empty_remaining_data() -> XRayResult {
    let mut chunk: ChunkReader<InMemoryChunkDataSource> = ChunkReader::from_bytes(&[0, 0, 0, 0])?;

    assert_eq!(chunk.cursor_pos(), 0, "Expect 0 bytes read");
    assert_eq!(chunk.read_bytes_remain(), 4, "Expect 4 bytes remaining");

    assert_eq!(
      chunk.read_null_terminated_win_string()?,
      "",
      "Expect empty string"
    );
    assert_eq!(chunk.cursor_pos(), 1, "Expect 1 byte read");
    assert_eq!(chunk.read_bytes_remain(), 3, "Expect 3 bytes remaining");

    Ok(())
  }

  #[test]
  fn test_read_null_terminated_strings_few() -> XRayResult {
    let mut chunk: ChunkReader<InMemoryChunkDataSource> =
      ChunkReader::from_bytes(&[b'a', b'b', b'c', 0, b'c', b'b', b'a', 0])?;

    assert_eq!(chunk.read_bytes_remain(), 8, "Expect 8 bytes remaining");
    assert_eq!(chunk.cursor_pos(), 0, "Expect 0 bytes read");

    assert_eq!(
      chunk.read_null_terminated_win_string()?,
      "abc",
      "Expect string read"
    );
    assert_eq!(chunk.cursor_pos(), 4, "Expect 4 bytes read");
    assert_eq!(chunk.read_bytes_remain(), 4, "Expect 4 bytes remaining");

    assert_eq!(
      chunk.read_null_terminated_win_string()?,
      "cba",
      "Expect string read"
    );
    assert_eq!(chunk.cursor_pos(), 8, "Expect 8 bytes read");
    assert_eq!(chunk.read_bytes_remain(), 0, "Expect 0 bytes remaining");

    Ok(())
  }

  #[test]
  fn test_read_null_terminated_string_over_limit() -> XRayResult {
    let mut chunk: ChunkReader<InMemoryChunkDataSource> = ChunkReader::from_bytes(&[b'a'; 1024])?;

    assert_eq!(chunk.cursor_pos(), 0, "Expect 0 bytes read");
    assert_eq!(
      chunk.read_bytes_remain(),
      1024,
      "Expect 1024 bytes remaining"
    );

    assert_eq!(
      chunk
        .read_null_terminated_win_string_limited(500)
        .unwrap_err()
        .to_string(),
      "Parsing error: Cannot parse string, reading data over buffer size limit",
      "Expect buffer limit error"
    );

    Ok(())
  }

  #[test]
  fn test_read_rn_terminated_string_empty() -> XRayResult {
    let mut chunk: ChunkReader<InMemoryChunkDataSource> = ChunkReader::from_bytes(&[])?;

    assert_eq!(chunk.read_bytes_remain(), 0, "Expect 0 bytes remaining");
    assert_eq!(chunk.cursor_pos(), 0, "Expect 0 bytes read");

    assert_eq!(
      chunk
        .read_rn_terminated_win_string()
        .unwrap_err()
        .to_string(),
      "Missing terminator error: RN sequence is not found in buffer, no data to be read",
      "Expect error on empty read"
    );
    assert_eq!(chunk.cursor_pos(), 0, "Expect 0 bytes read");

    Ok(())
  }

  #[test]
  fn test_read_rn_terminated_string_empty_null() -> XRayResult {
    let mut chunk: ChunkReader<InMemoryChunkDataSource> = ChunkReader::from_bytes(&[0])?;

    assert_eq!(chunk.cursor_pos(), 0, "Expect 0 bytes read");
    assert_eq!(chunk.read_bytes_remain(), 1, "Expect 1 byte remaining");

    assert_eq!(
      chunk.read_null_terminated_win_string()?,
      "",
      "Expect empty string with terminator"
    );
    assert_eq!(chunk.cursor_pos(), 1, "Expect 1 byte read");
    assert_eq!(chunk.read_bytes_remain(), 0, "Expect 0 bytes remaining");

    Ok(())
  }

  #[test]
  fn test_read_rn_terminated_string_empty_remaining_data() -> XRayResult {
    let mut chunk: ChunkReader<InMemoryChunkDataSource> =
      ChunkReader::from_bytes(&[b'\r', b'\n', 0, 0, 0])?;

    assert_eq!(chunk.cursor_pos(), 0, "Expect 0 bytes read");
    assert_eq!(chunk.read_bytes_remain(), 5, "Expect 5 bytes remaining");

    assert_eq!(
      chunk.read_rn_terminated_win_string()?,
      "",
      "Expect empty string"
    );
    assert_eq!(chunk.cursor_pos(), 2, "Expect 2 byte read");
    assert_eq!(chunk.read_bytes_remain(), 3, "Expect 3 bytes remaining");

    Ok(())
  }

  #[test]
  fn test_read_rn_terminated_strings_few() -> XRayResult {
    let mut chunk: ChunkReader<InMemoryChunkDataSource> = ChunkReader::from_bytes(&[
      b'a', b'b', b'c', b'\r', b'\n', b'c', b'b', b'a', b'\r', b'\n',
    ])?;

    assert_eq!(chunk.read_bytes_remain(), 10, "Expect 10 bytes remaining");
    assert_eq!(chunk.cursor_pos(), 0, "Expect 0 bytes read");

    assert_eq!(
      chunk.read_rn_terminated_win_string()?,
      "abc",
      "Expect string read"
    );
    assert_eq!(chunk.cursor_pos(), 5, "Expect 5 bytes read");
    assert_eq!(chunk.read_bytes_remain(), 5, "Expect 5 bytes remaining");

    assert_eq!(
      chunk.read_rn_terminated_win_string()?,
      "cba",
      "Expect string read"
    );
    assert_eq!(chunk.cursor_pos(), 10, "Expect 10 bytes read");
    assert_eq!(chunk.read_bytes_remain(), 0, "Expect 0 bytes remaining");

    Ok(())
  }

  #[test]
  fn test_read_rn_terminated_string_over_limit() -> XRayResult {
    let mut chunk: ChunkReader<InMemoryChunkDataSource> = ChunkReader::from_bytes(&[b'a'; 1024])?;

    assert_eq!(chunk.cursor_pos(), 0, "Expect 0 bytes read");
    assert_eq!(
      chunk.read_bytes_remain(),
      1024,
      "Expect 1024 bytes remaining"
    );

    assert_eq!(
      chunk
        .read_rn_terminated_win_string_limited(500)
        .unwrap_err()
        .to_string(),
      "Parsing error: Cannot parse string, reading data over buffer size limit",
      "Expect buffer limit error"
    );

    Ok(())
  }
}
