use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::constants::FLAG_SPAWN_DESTROY_ON_SPAWN;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::meta::alife_class::AlifeClass;
use crate::data::meta::cls_id::ClsId;
use crate::types::{SpawnByteOrder, Vector3d};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;
use std::io::Write;

/// Generic abstract alife object base.
pub struct AlifeObjectBase {
  pub index: u16,
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
  pub client_data_size: u16,
  pub spawn_id: u16,
  pub inherited_size: u16,
  pub inherited: Box<dyn AlifeObjectGeneric<Order = SpawnByteOrder>>,
  pub update_data_length: u16,
  pub update_size: u16,
  pub update_data: Vec<u8>, // todo: Parse.
}

impl AlifeObjectBase {
  /// Read generic alife object data from the chunk.
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectBase> {
    let mut index_chunk: Chunk = chunk.read_child_by_index(0)?;

    let index: u16 = index_chunk.read_u16::<T>()?;

    let mut data_chunk: Chunk = chunk.read_child_by_index(1)?;
    let mut spawn_chunk: Chunk = data_chunk.read_child_by_index(0)?;
    let mut update_chunk: Chunk = data_chunk.read_child_by_index(1)?;

    let data_length: u16 = spawn_chunk.read_u16::<T>()?;

    assert_eq!(data_length as u64 + 2, spawn_chunk.size);

    let net_action_id: u16 = spawn_chunk.read_u16::<T>()?;

    assert_eq!(net_action_id, 1); // todo: Constant for action ID.

    let section: String = spawn_chunk.read_null_terminated_string()?;
    let clsid: ClsId = ClsId::from_section(&section);
    let class: AlifeClass = AlifeClass::from_cls_id(&clsid);
    let name: String = spawn_chunk.read_null_terminated_string()?;
    let script_game_id: u8 = spawn_chunk.read_u8()?;
    let script_rp: u8 = spawn_chunk.read_u8()?;
    let position: Vector3d = spawn_chunk.read_f32_3d_vector::<T>()?;
    let direction: Vector3d = spawn_chunk.read_f32_3d_vector::<T>()?;
    let respawn_time: u16 = spawn_chunk.read_u16::<T>()?;
    let id: u16 = spawn_chunk.read_u16::<T>()?;
    let parent_id: u16 = spawn_chunk.read_u16::<T>()?;
    let phantom_id: u16 = spawn_chunk.read_u16::<T>()?;
    let script_flags: u16 = spawn_chunk.read_u16::<T>()?;
    let version: u16 = if script_flags & FLAG_SPAWN_DESTROY_ON_SPAWN == 0 {
      0
    } else {
      spawn_chunk.read_u16::<T>()?
    };

    assert!(
      version > 120,
      "Unexpected version of alife object in spawn file."
    );

    let cse_abstract_unknown: u16 = spawn_chunk.read_u16::<T>()?;
    let script_version: u16 = spawn_chunk.read_u16::<T>()?;
    let client_data_size: u16 = spawn_chunk.read_u16::<T>()?;

    assert_eq!(client_data_size, 0); // Or read client data?

    let spawn_id: u16 = spawn_chunk.read_u16::<T>()?;
    let inherited_size: u16 = spawn_chunk.read_u16::<T>()?;

    assert_eq!(
      inherited_size as u64 - 2,
      spawn_chunk.end_pos() - spawn_chunk.cursor_pos()
    );

    assert_ne!(class, AlifeClass::Unknown);

    let inherited: Box<dyn AlifeObjectGeneric<Order = SpawnByteOrder>> =
      AlifeClass::read_by_class::<T>(&mut spawn_chunk, &class)?;

    let update_data_length: u16 = update_chunk.file.read_u16::<T>()?;
    let update_size: u16 = update_chunk.file.read_u16::<T>()?;

    assert_eq!(update_data_length as u64 + 2, update_chunk.size);
    assert_eq!(update_size, 0);

    let update_data: Vec<u8> = update_chunk.read_bytes(update_chunk.read_bytes_remain() as usize)?;

    assert!(index_chunk.is_ended());
    assert!(data_chunk.is_ended());
    assert!(spawn_chunk.is_ended());
    assert!(update_chunk.is_ended());
    assert!(chunk.is_ended());

    Ok(AlifeObjectBase {
      index,
      section,
      clsid,
      name,
      script_game_id,
      script_rp,
      position,
      direction,
      respawn_time,
      id,
      parent_id,
      phantom_id,
      script_flags,
      version,
      cse_abstract_unknown,
      script_version,
      client_data_size,
      spawn_id,
      inherited_size,
      inherited,
      update_data_length,
      update_size,
      update_data,
    })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    let mut index_writer: ChunkWriter = ChunkWriter::new();
    let mut data_writer: ChunkWriter = ChunkWriter::new();

    let mut data_spawn_writer: ChunkWriter = ChunkWriter::new();
    let mut data_update_writer: ChunkWriter = ChunkWriter::new();

    index_writer.write_u16::<T>(self.index)?;

    data_spawn_writer.write_u16::<T>(0)?;
    data_spawn_writer.write_u16::<T>(1)?; // todo: Constant for action ID.

    data_spawn_writer.write_null_terminated_string(&self.section)?;
    data_spawn_writer.write_null_terminated_string(&self.name)?;
    data_spawn_writer.write_u8(self.script_game_id)?;
    data_spawn_writer.write_f32_3d_vector::<T>(&self.position)?;
    data_spawn_writer.write_f32_3d_vector::<T>(&self.direction)?;
    data_spawn_writer.write_u16::<T>(self.respawn_time)?;
    data_spawn_writer.write_u16::<T>(self.id)?;
    data_spawn_writer.write_u16::<T>(self.parent_id)?;
    data_spawn_writer.write_u16::<T>(self.phantom_id)?;
    data_spawn_writer.write_u16::<T>(self.script_flags)?;
    data_spawn_writer.write_u16::<T>(self.version)?;
    data_spawn_writer.write_u16::<T>(self.cse_abstract_unknown)?;
    data_spawn_writer.write_u16::<T>(self.script_version)?;
    data_spawn_writer.write_u16::<T>(self.client_data_size)?;
    data_spawn_writer.write_u16::<T>(self.spawn_id)?;
    data_spawn_writer.write_u16::<T>(self.inherited_size)?;

    self.inherited.write(&mut data_spawn_writer)?;

    data_update_writer.write_u16::<T>(self.update_data_length)?;
    data_update_writer.write_u16::<T>(self.update_size)?;

    data_writer.write_all(
      data_spawn_writer
        .flush_chunk_into_buffer::<T>(0)?
        .as_slice(),
    )?;
    data_writer.write_all(
      data_update_writer
        .flush_chunk_into_buffer::<T>(1)?
        .as_slice(),
    )?;

    writer.write_all(index_writer.flush_chunk_into_buffer::<T>(0)?.as_slice())?;
    writer.write_all(data_writer.flush_chunk_into_buffer::<T>(1)?.as_slice())?;

    Ok(())
  }
}
