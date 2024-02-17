use crate::spawn::chunk::Chunk;
use crate::spawn::chunk_utils::{read_f32_vector, read_null_terminated_string};
use crate::spawn::constants::FLAG_SPAWN_DESTROY_ON_SPAWN;
use crate::spawn::data::meta::{AlifeClass, ClsId};
use crate::spawn::types::Vector3d;
use byteorder::{LittleEndian, ReadBytesExt};
use fileslice::FileSlice;

/// Generic abstract alife object.
pub struct AlifeObject {
  pub id: u16,
  pub section: String,
  pub clsid: ClsId,
  pub name: String,
  pub script_game_id: u8,
  pub script_rp: u8,
  pub position: Vector3d,
  pub direction: Vector3d,
  pub respawn_time: u16,
  pub parent_id: u16,
  pub phantom_id: u16,
  pub script_flags: u16,
  pub version: u16,
  pub cse_abstract_unknown: u16,
  pub script_version: u16,
  pub spawn_id: u16,
}

impl AlifeObject {
  pub fn from_file(file: &mut FileSlice) -> AlifeObject {
    let (mut id_slice, _) =
      Chunk::open_by_index(file, 0).expect("Expected vertex ID chunk to exist.");

    let _id: u16 = id_slice.read_u16::<LittleEndian>().unwrap();

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
    let clsid: ClsId = ClsId::from_section(&section);
    let class: AlifeClass = AlifeClass::from_cls_id(&clsid);
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
    let version: u16 = if script_flags & FLAG_SPAWN_DESTROY_ON_SPAWN == 0 {
      0
    } else {
      spawn_slice.read_u16::<LittleEndian>().unwrap()
    };

    assert!(
      version > 120,
      "Unexpected version of alife object in spawn file."
    );

    let cse_abstract_unknown: u16 = spawn_slice.read_u16::<LittleEndian>().unwrap();
    let script_version: u16 = spawn_slice.read_u16::<LittleEndian>().unwrap();
    let client_data_size: u16 = spawn_slice.read_u16::<LittleEndian>().unwrap();

    assert_eq!(client_data_size, 0); // Or read client data?

    let spawn_id: u16 = spawn_slice.read_u16::<LittleEndian>().unwrap();
    let extended_size: u16 = spawn_slice.read_u16::<LittleEndian>().unwrap();

    assert_eq!(
      extended_size as u64 - 2,
      spawn_slice.end_pos() - spawn_slice.cursor_pos()
    );

    assert_ne!(class, AlifeClass::Unknown);

    AlifeClass::read_by_class(&mut spawn_slice, &class);

    Self::assert_update_data(file);

    AlifeObject {
      id,
      section,
      clsid,
      name,
      script_game_id,
      script_rp,
      position,
      direction,
      respawn_time,
      parent_id,
      phantom_id,
      script_flags,
      version,
      cse_abstract_unknown,
      script_version,
      spawn_id,
    }
  }

  /// Validate that read data is correct and does not contain update information.
  fn assert_update_data(file: &mut FileSlice) -> () {
    let (mut update_slice, update_chunk) =
      Chunk::open_by_index(file, 1).expect("Expected data chunk to exist in object definition.");

    let data_length: u16 = update_slice.read_u16::<LittleEndian>().unwrap();
    let update_size: u16 = update_slice.read_u16::<LittleEndian>().unwrap();

    assert_eq!(data_length as u32 + 2, update_chunk.size);
    assert_eq!(update_size, 0);
    assert_eq!(file.cursor_pos(), file.end_pos());
  }
}
