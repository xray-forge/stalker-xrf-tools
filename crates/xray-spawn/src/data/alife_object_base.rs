use crate::chunk::chunk::Chunk;
use crate::constants::FLAG_SPAWN_DESTROY_ON_SPAWN;
use crate::data::alife_class::AlifeClass;
use crate::data::cls_id::ClsId;
use crate::types::{SpawnByteOrder, Vector3d};
use byteorder::ReadBytesExt;

/// Generic abstract alife object base.
pub struct AlifeObjectBase {
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
  pub inherited: Box<dyn AlifeObjectGeneric>,
}

impl AlifeObjectBase {
  pub fn from_chunk(chunk: &mut Chunk) -> AlifeObjectBase {
    let mut id_chunk: Chunk = chunk
      .read_child_by_index(0)
      .expect("Expected vertex ID chunk to exist.");

    let _id: u16 = id_chunk.read_u16::<SpawnByteOrder>().unwrap();

    let mut vertex_data_chunk: Chunk = chunk
      .read_child_by_index(1)
      .expect("Expected vertex data chunk to exist.");

    Self::read_object_data(&mut vertex_data_chunk)
  }

  fn read_object_data(chunk: &mut Chunk) -> AlifeObjectBase {
    let mut spawn_chunk: Chunk = chunk
      .read_child_by_index(0)
      .expect("Expected data chunk to exist in object definition.");

    let data_length: u16 = spawn_chunk.read_u16::<SpawnByteOrder>().unwrap();

    assert_eq!(data_length as u64 + 2, spawn_chunk.size);

    // todo: Is it net packet action id?
    let dummy: u16 = spawn_chunk.read_u16::<SpawnByteOrder>().unwrap();

    assert_eq!(dummy, 1);

    let section: String = spawn_chunk.read_null_terminated_string().unwrap();
    let clsid: ClsId = ClsId::from_section(&section);
    let class: AlifeClass = AlifeClass::from_cls_id(&clsid);
    let name: String = spawn_chunk.read_null_terminated_string().unwrap();
    let script_game_id: u8 = spawn_chunk.read_u8().unwrap();
    let script_rp: u8 = spawn_chunk.read_u8().unwrap();
    let position: Vector3d = spawn_chunk.read_f32_3d_vector::<SpawnByteOrder>().unwrap();
    let direction: Vector3d = spawn_chunk.read_f32_3d_vector::<SpawnByteOrder>().unwrap();
    let respawn_time: u16 = spawn_chunk.read_u16::<SpawnByteOrder>().unwrap();
    let id: u16 = spawn_chunk.read_u16::<SpawnByteOrder>().unwrap();
    let parent_id: u16 = spawn_chunk.read_u16::<SpawnByteOrder>().unwrap();
    let phantom_id: u16 = spawn_chunk.read_u16::<SpawnByteOrder>().unwrap();
    let script_flags: u16 = spawn_chunk.read_u16::<SpawnByteOrder>().unwrap();
    let version: u16 = if script_flags & FLAG_SPAWN_DESTROY_ON_SPAWN == 0 {
      0
    } else {
      spawn_chunk.read_u16::<SpawnByteOrder>().unwrap()
    };

    assert!(
      version > 120,
      "Unexpected version of alife object in spawn file."
    );

    let cse_abstract_unknown: u16 = spawn_chunk.read_u16::<SpawnByteOrder>().unwrap();
    let script_version: u16 = spawn_chunk.read_u16::<SpawnByteOrder>().unwrap();
    let client_data_size: u16 = spawn_chunk.read_u16::<SpawnByteOrder>().unwrap();

    assert_eq!(client_data_size, 0); // Or read client data?

    let spawn_id: u16 = spawn_chunk.read_u16::<SpawnByteOrder>().unwrap();
    let extended_size: u16 = spawn_chunk.read_u16::<SpawnByteOrder>().unwrap();

    assert_eq!(
      extended_size as u64 - 2,
      spawn_chunk.end_pos() - spawn_chunk.cursor_pos()
    );

    assert_ne!(class, AlifeClass::Unknown);

    let inherited: Box<dyn AlifeObjectGeneric> =
      AlifeClass::read_by_class(&mut spawn_chunk, &class);

    Self::assert_update_data(chunk);

    AlifeObjectBase {
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
      inherited,
    }
  }

  /// Validate that read data is correct and does not contain update information.
  fn assert_update_data(chunk: &mut Chunk) -> () {
    let mut update_chunk: Chunk = chunk
      .read_child_by_index(1)
      .expect("Expected data chunk to exist in object definition.");

    let data_length: u16 = update_chunk.file.read_u16::<SpawnByteOrder>().unwrap();
    let update_size: u16 = update_chunk.file.read_u16::<SpawnByteOrder>().unwrap();

    assert_eq!(data_length as u64 + 2, update_chunk.size);
    assert_eq!(update_size, 0);
    assert_eq!(chunk.read_bytes_remain(), 0);
  }
}

pub trait AlifeObjectInheritedReader<T> {
  fn from_chunk(chunk: &mut Chunk) -> T;

  fn verify(chunk: &Chunk) -> () {
    assert_eq!(
      chunk.read_bytes_remain(),
      0,
      "Expected all data to be read from chunk."
    );
  }
}

pub trait AlifeObjectGeneric {}
