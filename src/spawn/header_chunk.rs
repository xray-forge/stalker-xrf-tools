use crate::spawn::chunk::chunk::Chunk;
use crate::spawn::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

#[derive(Debug)]
pub struct HeaderChunk {
  pub id: u32,
  pub version: u32,
  pub guid: u128,
  pub graph_guid: u128,
  pub count: u32,
  pub level_count: u32,
}

impl HeaderChunk {
  /// Read header chunk by position descriptor.
  pub fn from_chunk(mut chunk: Chunk) -> Option<HeaderChunk> {
    log::info!(
      "Parsing header chunk, {:?} -> {:?}",
      chunk.start_pos(),
      chunk.end_pos()
    );

    let version: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let guid: u128 = chunk.read_u128::<SpawnByteOrder>().unwrap();
    let graph_guid: u128 = chunk.read_u128::<SpawnByteOrder>().unwrap();
    let count: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let level_count: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();

    log::info!("Parsed header chunk, {:?} bytes", chunk.read_bytes_len());
    assert_eq!(chunk.read_bytes_remain(), 0);

    return Some(HeaderChunk {
      id: chunk.id,
      version,
      guid,
      graph_guid,
      count,
      level_count,
    });
  }
}
