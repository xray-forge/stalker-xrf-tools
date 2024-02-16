use crate::spawn::chunk::Chunk;
use byteorder::{LittleEndian, ReadBytesExt};
use fileslice::FileSlice;

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
  pub fn from_chunk(file: &mut FileSlice, chunk: &Chunk) -> Option<HeaderChunk> {
    let mut file: FileSlice = chunk.in_slice(file);

    log::info!(
      "Parsing header chunk, {:?} -> {:?}",
      file.start_pos(),
      file.end_pos()
    );

    let version: u32 = file.read_u32::<LittleEndian>().unwrap();
    let guid: u128 = file.read_u128::<LittleEndian>().unwrap();
    let graph_guid: u128 = file.read_u128::<LittleEndian>().unwrap();
    let count: u32 = file.read_u32::<LittleEndian>().unwrap();
    let level_count: u32 = file.read_u32::<LittleEndian>().unwrap();

    log::info!(
      "Parsed header chunk, {:?} bytes",
      file.end_pos() - file.start_pos()
    );

    assert_eq!(file.cursor_pos(), file.end_pos());

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
