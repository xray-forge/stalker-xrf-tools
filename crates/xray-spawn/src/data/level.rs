use crate::chunk::chunk::Chunk;
use crate::types::Vector3d;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

#[derive(Debug)]
pub struct Level {
  pub id: u8,
  pub name: String,
  pub offset: Vector3d<f32>,
  pub section: String,
  pub guid: u128,
}

impl Level {
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<Level> {
    let name: String = chunk.read_null_terminated_string()?;
    let offset: Vector3d = chunk.read_f32_3d_vector::<T>()?;
    let id: u8 = chunk.read_u8()?;
    let section: String = chunk.read_null_terminated_string()?;
    let guid: u128 = chunk.read_u128::<T>()?;

    Ok(Level {
      id,
      name,
      offset,
      section,
      guid,
    })
  }
}
