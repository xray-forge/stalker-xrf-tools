use crate::chunk::chunk::Chunk;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;
use std::io;

#[derive(Debug)]
pub struct HeaderChunk {
  pub chunk: Chunk,
  pub version: u32,
  pub guid: u128,
  pub graph_guid: u128,
  pub count: u32,
  pub level_count: u32,
}

impl HeaderChunk {
  /// Read header chunk by position descriptor.
  pub fn from_chunk(mut chunk: Chunk) -> io::Result<HeaderChunk> {
    log::info!(
      "Parsing header chunk, {:?} -> {:?}",
      chunk.start_pos(),
      chunk.end_pos()
    );

    let version: u32 = chunk.read_u32::<SpawnByteOrder>()?;
    let guid: u128 = chunk.read_u128::<SpawnByteOrder>().unwrap();
    let graph_guid: u128 = chunk.read_u128::<SpawnByteOrder>().unwrap();
    let count: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let level_count: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();

    log::info!("Parsed header chunk, {:?} bytes", chunk.read_bytes_len());

    assert!(chunk.is_ended());

    Ok(HeaderChunk {
      chunk,
      version,
      guid,
      graph_guid,
      count,
      level_count,
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::header_chunk::HeaderChunk;
  use crate::test::utils::open_test_resource_as_slice;
  use std::io;

  #[test]
  fn test_read_empty_chunk() {
    let chunk: Chunk = Chunk::from_file(
      open_test_resource_as_slice(String::from("empty_nested_single.chunk")).unwrap(),
    )
    .unwrap()
    .read_child_by_index(0)
    .unwrap();

    let header: io::Result<HeaderChunk> = HeaderChunk::from_chunk(chunk);

    assert!(header.is_err(), "Expected failure with empty chunk.");
  }
}
