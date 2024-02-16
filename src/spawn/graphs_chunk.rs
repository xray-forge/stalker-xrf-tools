use crate::spawn::chunk::Chunk;
use crate::spawn::data::level::Level;
use byteorder::{LittleEndian, ReadBytesExt};
use fileslice::FileSlice;

#[derive(Debug)]
pub struct GraphsChunk {
  pub id: u32,
  pub version: u8,
  pub vertex_count: u16,
  pub edge_count: u32,
  pub point_count: u32,
  pub guid: u128,
  pub size: u32,
  pub levels: Vec<Level>,
}

impl GraphsChunk {
  /// Read patrols chunk by position descriptor.
  pub fn from_chunk(file: &mut FileSlice, chunk: &Chunk) -> Option<GraphsChunk> {
    let mut file: FileSlice = chunk.in_slice(file);

    log::info!("Parsing level graphs");

    let version: u8 = file.read_u8().unwrap();
    let vertex_count: u16 = file.read_u16::<LittleEndian>().unwrap();
    let edge_count: u32 = file.read_u32::<LittleEndian>().unwrap();
    let point_count: u32 = file.read_u32::<LittleEndian>().unwrap();
    let guid: u128 = file.read_u128::<LittleEndian>().unwrap();
    let level_count: u8 = file.read_u8().unwrap();
    let size: u32 = 4096;

    let mut levels: Vec<Level> = Vec::new();

    for _ in 0..level_count {
      let level = Level::from_file(&mut file);

      levels.push(level)
    }

    log::info!("Parsed graphs v{version}");

    return Some(GraphsChunk {
      id: chunk.id,
      version,
      levels,
      vertex_count,
      edge_count,
      point_count,
      guid,
      size,
    });
  }
}
