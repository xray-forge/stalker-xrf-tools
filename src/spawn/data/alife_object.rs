use crate::spawn::chunk::Chunk;
use crate::spawn::chunk_utils::{read_f32_vector, read_null_terminated_string};
use crate::spawn::types::Vector3d;
use byteorder::{LittleEndian, ReadBytesExt};
use fileslice::FileSlice;

pub struct AlifeObject {
  pub id: u16,
  pub section: String,
  pub name: String,
  pub script_game_id: u8,
  pub script_rp: u8,
  pub position: Vector3d,
  pub direction: Vector3d,
  pub respawn_time: u16,
  pub parent_id: u16,
  pub phantom_id: u16,
  pub script_flags: u16,
}

impl AlifeObject {
  pub fn from_file(file: &mut FileSlice) -> AlifeObject {
    let (mut id_slice, _) =
      Chunk::open_by_index(file, 0).expect("Expected vertex ID chunk to exist.");

    let id: u16 = id_slice.read_u16::<LittleEndian>().unwrap();

    let (mut vertex_data_slice, _) =
      Chunk::open_by_index(file, 1).expect("Expected vertex data chunk to exist.");

    Self::read_object_data(&mut vertex_data_slice)
  }

  fn read_object_data(file: &mut FileSlice) -> AlifeObject {
    let (mut spawn_slice, spawn_chunk) =
      Chunk::open_by_index(file, 0).expect("Expected data chunk to exist in object definition.");

    let data_length: u16 = spawn_slice.read_u16::<LittleEndian>().unwrap();

    assert_eq!(data_length as u32 + 2, spawn_chunk.size);

    let dummy: u16 = spawn_slice.read_u16::<LittleEndian>().unwrap();

    assert_eq!(dummy, 1);

    let section: String = read_null_terminated_string(&mut spawn_slice);
    let name: String = read_null_terminated_string(&mut spawn_slice);
    let script_game_id: u8 = spawn_slice.read_u8().unwrap();
    let script_rp: u8 = spawn_slice.read_u8().unwrap();
    let position: Vector3d = read_f32_vector::<LittleEndian>(&mut spawn_slice);
    let direction: Vector3d = read_f32_vector::<LittleEndian>(&mut spawn_slice);
    let respawn_time: u16 = spawn_slice.read_u16::<LittleEndian>().unwrap();
    let id: u16 = spawn_slice.read_u16::<LittleEndian>().unwrap();
    let parent_id: u16 = spawn_slice.read_u16::<LittleEndian>().unwrap();
    let phantom_id: u16 = spawn_slice.read_u16::<LittleEndian>().unwrap();
    let script_flags: u16 = spawn_slice.read_u16::<LittleEndian>().unwrap();

    // todo: Parse spawn data.

    let (mut update_slice, update_chunk) =
      Chunk::open_by_index(file, 1).expect("Expected data chunk to exist in object definition.");

    let data_length: u16 = update_slice.read_u16::<LittleEndian>().unwrap();

    assert_eq!(data_length as u32 + 2, update_chunk.size);

    // todo: Parse update data.

    AlifeObject {
      id,
      section,
      name,
      script_game_id,
      script_rp,
      position,
      direction,
      respawn_time,
      parent_id,
      phantom_id,
      script_flags,
    }
  }
}
