use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::constants::{
  FLAG_SPAWN_DESTROY_ON_SPAWN, MINIMAL_SUPPORTED_SPAWN_VERSION, NET_ACTION_SPAWN,
};
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::meta::alife_class::AlifeClass;
use crate::data::meta::cls_id::ClsId;
use crate::data::vector_3d::Vector3d;
use crate::export::file_import::read_ini_field;
use crate::export::string::{bytes_from_base64, bytes_to_base64};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::Write;

/// Generic abstract alife object base.
#[derive(Debug, Serialize, Deserialize)]
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
  pub game_type: u16,
  pub script_version: u16,
  pub client_data_size: u16,
  pub spawn_id: u16,
  pub inherited: Box<dyn AlifeObjectGeneric>,
  pub update_data: Vec<u8>, // todo: Parse.
}

impl AlifeObjectBase {
  /// Read generic alife object data from the chunk.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectBase> {
    let mut index_reader: ChunkReader = reader.read_child_by_index(0)?;

    let index: u16 = index_reader.read_u16::<T>()?;

    let mut data_reader: ChunkReader = reader.read_child_by_index(1)?;
    let mut spawn_reader: ChunkReader = data_reader.read_child_by_index(0)?;
    let mut update_reader: ChunkReader = data_reader.read_child_by_index(1)?;

    let data_length: u16 = spawn_reader.read_u16::<T>()?;

    assert_eq!(data_length as u64 + 2, spawn_reader.size);

    let net_action: u16 = spawn_reader.read_u16::<T>()?;

    assert_eq!(net_action, NET_ACTION_SPAWN);

    let section: String = spawn_reader.read_null_terminated_win_string()?;
    let clsid: ClsId = ClsId::from_section(&section);
    let class: AlifeClass = AlifeClass::from_cls_id(&clsid);
    let name: String = spawn_reader.read_null_terminated_win_string()?;
    let script_game_id: u8 = spawn_reader.read_u8()?;
    let script_rp: u8 = spawn_reader.read_u8()?;
    let position: Vector3d = spawn_reader.read_f32_3d_vector::<T>()?;
    let direction: Vector3d = spawn_reader.read_f32_3d_vector::<T>()?;
    let respawn_time: u16 = spawn_reader.read_u16::<T>()?;
    let id: u16 = spawn_reader.read_u16::<T>()?;
    let parent_id: u16 = spawn_reader.read_u16::<T>()?;
    let phantom_id: u16 = spawn_reader.read_u16::<T>()?;
    let script_flags: u16 = spawn_reader.read_u16::<T>()?;
    let version: u16 = if script_flags & FLAG_SPAWN_DESTROY_ON_SPAWN == 0 {
      0
    } else {
      spawn_reader.read_u16::<T>()?
    };

    assert!(
      version > MINIMAL_SUPPORTED_SPAWN_VERSION,
      "Unexpected version of alife object in spawn file, flag is {script_flags}"
    );

    let game_type: u16 = spawn_reader.read_u16::<T>()?;
    let script_version: u16 = spawn_reader.read_u16::<T>()?;
    let client_data_size: u16 = spawn_reader.read_u16::<T>()?;

    assert_eq!(client_data_size, 0); // Or read client data?

    let spawn_id: u16 = spawn_reader.read_u16::<T>()?;
    let inherited_size: u16 = spawn_reader.read_u16::<T>()?;

    assert_eq!(
      inherited_size as u64 - 2,
      spawn_reader.end_pos() - spawn_reader.cursor_pos()
    );

    assert_ne!(class, AlifeClass::Unknown);

    let inherited: Box<dyn AlifeObjectGeneric> =
      AlifeClass::read_by_class::<T>(&mut spawn_reader, &class)?;

    let update_data_length: u16 = update_reader.file.read_u16::<T>()?;
    let update_size: u16 = update_reader.file.read_u16::<T>()?;

    assert_eq!(update_data_length as u64 + 2, update_reader.size);
    assert_eq!(update_size, 0);

    let update_data: Vec<u8> =
      update_reader.read_bytes(update_reader.read_bytes_remain() as usize)?;

    assert!(index_reader.is_ended());
    assert!(data_reader.is_ended());
    assert!(spawn_reader.is_ended());
    assert!(update_reader.is_ended());
    assert!(reader.is_ended());

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
      game_type,
      script_version,
      client_data_size,
      spawn_id,
      inherited,
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

    object_data_writer.write_null_terminated_win_string(&self.section)?;
    object_data_writer.write_null_terminated_win_string(&self.name)?;
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
    object_data_writer.write_u16::<T>(self.game_type)?;
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

  /// Import alife object data from ini file section.
  pub fn import(props: &Properties) -> io::Result<AlifeObjectBase> {
    let section: String = read_ini_field("section", props)?;
    let clsid: ClsId = ClsId::from_section(&section);

    Ok(AlifeObjectBase {
      index: read_ini_field("index", props)?,
      id: read_ini_field("id", props)?,
      net_action: read_ini_field("net_action", props)?,
      clsid: clsid.clone(),
      section,
      name: read_ini_field("name", props)?,
      script_game_id: read_ini_field("script_game_id", props)?,
      script_rp: read_ini_field("script_rp", props)?,
      position: read_ini_field("position", props)?,
      direction: read_ini_field("direction", props)?,
      respawn_time: read_ini_field("respawn_time", props)?,
      parent_id: read_ini_field("parent_id", props)?,
      phantom_id: read_ini_field("phantom_id", props)?,
      script_flags: read_ini_field("script_flags", props)?,
      version: read_ini_field("version", props)?,
      game_type: read_ini_field("game_type", props)?,
      script_version: read_ini_field("script_version", props)?,
      client_data_size: read_ini_field("client_data_size", props)?,
      spawn_id: read_ini_field("spawn_id", props)?,
      inherited: AlifeClass::import_by_class(&AlifeClass::from_cls_id(&clsid), props)?,
      update_data: bytes_from_base64(&read_ini_field::<String>("update_data", props)?)?,
    })
  }

  /// Export alife object data into ini file.
  pub fn export(&self, section: &str, ini: &mut Ini) {
    ini
      .with_section(Some(section))
      .set("index", self.index.to_string())
      .set("id", self.id.to_string())
      .set("net_action", self.net_action.to_string())
      .set("section", &self.section)
      .set("name", &self.name)
      .set("script_game_id", self.script_game_id.to_string())
      .set("script_rp", self.script_rp.to_string())
      .set("position", self.position.to_string())
      .set("direction", self.position.to_string())
      .set("respawn_time", self.respawn_time.to_string())
      .set("parent_id", self.parent_id.to_string())
      .set("phantom_id", self.phantom_id.to_string())
      .set("script_flags", self.script_flags.to_string())
      .set("version", self.version.to_string())
      .set("game_type", self.game_type.to_string())
      .set("script_version", self.script_version.to_string())
      .set("client_data_size", self.client_data_size.to_string())
      .set("spawn_id", self.script_version.to_string())
      .set("index", self.index.to_string());

    self.inherited.export(section, ini);

    ini
      .with_section(Some(section))
      .set("update_data", bytes_to_base64(&self.update_data));
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_item::AlifeObjectItem;
  use crate::data::alife::alife_object_item_custom_outfit::AlifeObjectItemCustomOutfit;
  use crate::data::alife_object_base::AlifeObjectBase;
  use crate::data::meta::cls_id::ClsId;
  use crate::data::vector_3d::Vector3d;
  use crate::test::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object_base() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "alife_object_base.chunk");

    let object: AlifeObjectBase = AlifeObjectBase {
      index: 10,
      id: 340,
      net_action: 1,
      section: String::from("dolg_heavy_outfit"),
      clsid: ClsId::EStlk,
      name: String::from("test-outfit-object"),
      script_game_id: 2,
      script_rp: 3,
      position: Vector3d::new(1.0, 2.0, 3.0),
      direction: Vector3d::new(3.0, 2.0, 1.0),
      respawn_time: 50000,
      parent_id: 2143,
      phantom_id: 0,
      script_flags: 33,
      version: 128,
      game_type: 1,
      script_version: 10,
      client_data_size: 0,
      spawn_id: 2354,
      inherited: Box::new(AlifeObjectItemCustomOutfit {
        base: AlifeObjectItem {
          base: AlifeObjectDynamicVisual {
            base: AlifeObjectAbstract {
              game_vertex_id: 12434,
              distance: 124.33,
              direct_control: 624345,
              level_vertex_id: 48528,
              flags: 34,
              custom_data: String::from("custom-data"),
              story_id: 523,
              spawn_story_id: 2865268,
            },
            visual_name: String::from("visual-name"),
            visual_flags: 0,
          },
          condition: 1.0,
          upgrades_count: 0,
        },
      }),
      update_data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
    };

    object.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 203);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 203);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 203 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectBase = AlifeObjectBase::read::<SpawnByteOrder>(&mut reader)?;

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
    assert_eq!(read_object.game_type, object.game_type);
    assert_eq!(read_object.script_version, object.script_version);
    assert_eq!(read_object.client_data_size, object.client_data_size);
    assert_eq!(read_object.spawn_id, object.spawn_id);
    assert_eq!(read_object.update_data, object.update_data);

    Ok(())
  }
}
