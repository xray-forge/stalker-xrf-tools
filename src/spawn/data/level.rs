use crate::spawn::chunk_utils::{read_f32_vector, read_null_terminated_string};
use crate::spawn::types::Vector3d;
use byteorder::{LittleEndian, ReadBytesExt};
use fileslice::FileSlice;

#[derive(Debug)]
pub struct Level {
  pub id: u8,
  pub name: String,
  pub offset: Vector3d<f32>,
  pub section: String,
  pub guid: u128,
}

impl Level {
  pub fn from_file(file: &mut FileSlice) -> Level {
    let name: String = read_null_terminated_string(file);
    let offset: Vector3d = read_f32_vector::<LittleEndian>(file);
    let id: u8 = file.read_u8().unwrap();
    let section: String = read_null_terminated_string(file);
    let guid: u128 = file.read_u128::<LittleEndian>().unwrap();

    Level {
      id,
      name,
      offset,
      section,
      guid,
    }
  }
}
