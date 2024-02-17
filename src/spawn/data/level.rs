use crate::spawn::chunk::chunk::Chunk;
use crate::spawn::types::{SpawnByteOrder, Vector3d};
use byteorder::ReadBytesExt;

#[derive(Debug)]
pub struct Level {
  pub id: u8,
  pub name: String,
  pub offset: Vector3d<f32>,
  pub section: String,
  pub guid: u128,
}

impl Level {
  pub fn from_chunk(chunk: &mut Chunk) -> Level {
    let name: String = chunk.read_null_terminated_string().unwrap();
    let offset: Vector3d = chunk.read_f32_vector::<SpawnByteOrder>().unwrap();
    let id: u8 = chunk.read_u8().unwrap();
    let section: String = chunk.read_null_terminated_string().unwrap();
    let guid: u128 = chunk.read_u128::<SpawnByteOrder>().unwrap();

    Level {
      id,
      name,
      offset,
      section,
      guid,
    }
  }
}
