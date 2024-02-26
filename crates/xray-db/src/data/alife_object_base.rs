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
#[derive(Debug)]
pub struct AlifeObjectBase {
  pub index: u16,
  pub id: u16,
  pub net_action: u16,
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
  #[inline(never)]
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectBase> {
    let mut index_chunk: Chunk = chunk.read_child_by_index(0)?;

    let index: u16 = index_chunk.read_u16::<T>()?;

    let mut data_chunk: Chunk = chunk.read_child_by_index(1)?;
    let mut spawn_chunk: Chunk = data_chunk.read_child_by_index(0)?;
    let mut update_chunk: Chunk = data_chunk.read_child_by_index(1)?;

    let data_length: u16 = spawn_chunk.read_u16::<T>()?;

    assert_eq!(data_length as u64 + 2, spawn_chunk.size);

    let net_action: u16 = spawn_chunk.read_u16::<T>()?;

    assert_eq!(net_action, 1); // todo: Constant for action ID.

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
      "Unexpected version of alife object in spawn file, flag is {script_flags}."
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
      net_action,
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

  /// Write alife object data into the writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    let mut index_writer: ChunkWriter = ChunkWriter::new();
    let mut data_writer: ChunkWriter = ChunkWriter::new();

    let mut data_spawn_writer: ChunkWriter = ChunkWriter::new();
    let mut data_update_writer: ChunkWriter = ChunkWriter::new();

    let mut object_data_writer: ChunkWriter = ChunkWriter::new();
    let mut inherited_data_writer: ChunkWriter = ChunkWriter::new();
    let mut updated_data_writer: ChunkWriter = ChunkWriter::new();

    index_writer.write_u16::<T>(self.index)?;

    object_data_writer.write_u16::<T>(self.net_action)?;

    object_data_writer.write_null_terminated_string(&self.section)?;
    object_data_writer.write_null_terminated_string(&self.name)?;
    object_data_writer.write_u8(self.script_game_id)?;
    object_data_writer.write_u8(self.script_rp)?;
    object_data_writer.write_f32_3d_vector::<T>(&self.position)?;
    object_data_writer.write_f32_3d_vector::<T>(&self.direction)?;
    object_data_writer.write_u16::<T>(self.respawn_time)?;
    object_data_writer.write_u16::<T>(self.id)?;
    object_data_writer.write_u16::<T>(self.parent_id)?;
    object_data_writer.write_u16::<T>(self.phantom_id)?;
    object_data_writer.write_u16::<T>(self.script_flags)?;
    object_data_writer.write_u16::<T>(self.version)?;
    object_data_writer.write_u16::<T>(self.cse_abstract_unknown)?;
    object_data_writer.write_u16::<T>(self.script_version)?;
    object_data_writer.write_u16::<T>(self.client_data_size)?;
    object_data_writer.write_u16::<T>(self.spawn_id)?;

    self.inherited.write(&mut inherited_data_writer)?;

    object_data_writer.write_u16::<T>(inherited_data_writer.bytes_written() as u16 + 2)?;
    object_data_writer.write_all(inherited_data_writer.flush_raw_into_buffer()?.as_slice())?;

    data_spawn_writer.write_u16::<T>(object_data_writer.bytes_written() as u16)?;
    data_spawn_writer.write_all(object_data_writer.flush_raw_into_buffer()?.as_slice())?;

    updated_data_writer.write_u16::<T>(0)?;
    updated_data_writer.write_all(&self.update_data)?;

    data_update_writer.write_u16::<T>(updated_data_writer.bytes_written() as u16)?;
    data_update_writer.write_all(updated_data_writer.flush_raw_into_buffer()?.as_slice())?;

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

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_item::AlifeObjectItem;
  use crate::data::alife::alife_object_item_custom_outfit::AlifeObjectItemCustomOutfit;
  use crate::data::alife::alife_object_visual::AlifeObjectVisual;
  use crate::data::alife_object_base::AlifeObjectBase;
  use crate::data::meta::cls_id::ClsId;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_object_base.chunk"));

    let object: AlifeObjectBase = AlifeObjectBase {
      index: 10,
      id: 340,
      net_action: 1,
      section: String::from("dolg_heavy_outfit"),
      clsid: ClsId::EStlk,
      name: String::from("test-outfit-object"),
      script_game_id: 2,
      script_rp: 3,
      position: (1.0, 2.0, 3.0),
      direction: (3.0, 2.0, 1.0),
      respawn_time: 50000,
      parent_id: 2143,
      phantom_id: 0,
      script_flags: 33,
      version: 128,
      cse_abstract_unknown: 1,
      script_version: 10,
      client_data_size: 0,
      spawn_id: 2354,
      inherited_size: 39,
      inherited: Box::new(AlifeObjectItemCustomOutfit {
        base: AlifeObjectItem {
          base: AlifeObjectVisual {
            base: AlifeObjectAbstract {
              game_vertex_id: 12434,
              distance: 124.33,
              direct_control: 0,
              level_vertex_id: 0,
              flags: 34,
              custom_data: "".to_string(),
              story_id: 0,
              spawn_story_id: 0,
            },
            visual_name: "".to_string(),
            visual_flags: 0,
          },
          condition: 1.0,
          upgrades_count: 0,
        },
      }),
      update_data_length: 19,
      update_size: 0,
      update_data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
    };

    object.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 181);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 181);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 181 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectBase =
      AlifeObjectBase::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object.index, object.index);
    assert_eq!(read_object.id, object.id);
    assert_eq!(read_object.net_action, object.net_action);
    assert_eq!(read_object.section, object.section);
    assert_eq!(read_object.clsid, object.clsid);
    assert_eq!(read_object.name, object.name);
    assert_eq!(read_object.script_game_id, object.script_game_id);
    assert_eq!(read_object.script_rp, object.script_rp);
    assert_eq!(read_object.position, object.position);
    assert_eq!(read_object.direction, object.direction);
    assert_eq!(read_object.respawn_time, object.respawn_time);
    assert_eq!(read_object.parent_id, object.parent_id);
    assert_eq!(read_object.phantom_id, object.phantom_id);
    assert_eq!(read_object.script_flags, object.script_flags);
    assert_eq!(read_object.version, object.version);
    assert_eq!(
      read_object.cse_abstract_unknown,
      object.cse_abstract_unknown
    );
    assert_eq!(read_object.script_version, object.script_version);
    assert_eq!(read_object.client_data_size, object.client_data_size);
    assert_eq!(read_object.spawn_id, object.spawn_id);
    assert_eq!(read_object.inherited_size, object.inherited_size);
    assert_eq!(read_object.update_data_length, object.update_data_length);
    assert_eq!(read_object.update_size, object.update_size);
    assert_eq!(read_object.update_data, object.update_data);

    Ok(())
  }
}
