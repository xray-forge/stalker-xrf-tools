use crate::spawn::chunk::Chunk;
use byteorder::{LittleEndian, ReadBytesExt};
use fileslice::FileSlice;

pub struct AlifeObject {
  pub id: u16,
}

impl AlifeObject {
  pub fn from_file(file: &mut FileSlice) -> AlifeObject {
    let (mut vertex_id_slice, _) =
      Chunk::open_by_index(file, 0).expect("Expected vertex ID chunk to exist.");

    let id: u16 = vertex_id_slice.read_u16::<LittleEndian>().unwrap();

    let (mut vertex_data_slice, _) =
      Chunk::open_by_index(file, 1).expect("Expected vertex data chunk to exist.");

    Self::read_object_data(&mut vertex_data_slice);

    // todo: Collect object data here.

    AlifeObject { id }
  }

  fn read_object_data(file: &mut FileSlice) -> () {
    let (mut id_slice, _) =
      Chunk::open_by_index(file, 1).expect("Expected id chunk to exist in object definition.");

    let id: u32 = id_slice.read_u32::<LittleEndian>().unwrap();

    let (mut data_slice, data_chunk) =
      Chunk::open_by_index(file, 0).expect("Expected data chunk to exist in object definition.");

    let data_length: u16 = data_slice.read_u16::<LittleEndian>().unwrap();

    assert_eq!(data_length as u32 + 2, data_chunk.size);

    // todo: Parse object.
  }
}
