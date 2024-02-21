use crate::chunk::chunk::Chunk;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

#[derive(Debug)]
pub struct GraphHeader {
  pub version: u8,
  pub vertex_count: u16,
  pub edge_count: u32,
  pub point_count: u32,
  pub guid: u128,
  pub level_count: u8,
}

impl GraphHeader {
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<GraphHeader> {
    let version: u8 = chunk.read_u8()?;
    let vertex_count: u16 = chunk.read_u16::<T>()?;
    let edge_count: u32 = chunk.read_u32::<T>()?;
    let point_count: u32 = chunk.read_u32::<T>()?;
    let guid: u128 = chunk.read_u128::<T>()?;
    let level_count: u8 = chunk.read_u8()?;

    Ok(GraphHeader {
      version,
      vertex_count,
      edge_count,
      point_count,
      guid,
      level_count,
    })
  }
}
