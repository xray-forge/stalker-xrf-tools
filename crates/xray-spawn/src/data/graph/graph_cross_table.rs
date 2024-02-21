use crate::chunk::chunk::Chunk;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct GraphCrossTable {
  pub size: u32,
  pub version: u32,
  pub cell_count: u32,
  pub vertex_count: u32,
  pub level_guid: u128,
  pub game_guid: u128,
}

impl GraphCrossTable {
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<GraphCrossTable> {
    let size: u32 = chunk.read_u32::<T>()?;
    let version: u32 = chunk.read_u32::<T>()?;
    let cell_count: u32 = chunk.read_u32::<T>()?;
    let vertex_count: u32 = chunk.read_u32::<T>()?;
    let level_guid: u128 = chunk.read_u128::<T>()?;
    let game_guid: u128 = chunk.read_u128::<T>()?;

    Ok(GraphCrossTable {
      size,
      version,
      cell_count,
      vertex_count,
      level_guid,
      game_guid,
    })
  }
}
