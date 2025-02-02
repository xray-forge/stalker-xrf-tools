use crate::constants::{
  FLAG_SPAWN_DESTROY_ON_SPAWN, MINIMAL_SUPPORTED_SPAWN_VERSION, NET_ACTION_SPAWN,
};
use crate::data::generic::vector_3d::Vector3d;
use crate::data::meta::alife_class::AlifeClass;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::cls_id::ClsId;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::Write;
use xray_chunk::{ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};
use xray_utils::{assert, decode_bytes_from_base64, encode_bytes_to_base64};

/// Generic abstract alife object base.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
  pub inherited: Box<dyn AlifeObjectWriter>,
  pub update_data: Vec<u8>, // todo: Parse.
}

impl AlifeObjectBase {
  /// Read generic alife object data from the chunk.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let mut index_reader: ChunkReader = reader.read_child_by_index(0)?;

    let index: u16 = index_reader.read_u16::<T>()?;

    let mut data_reader: ChunkReader = reader.read_child_by_index(1)?;
    let mut spawn_reader: ChunkReader = data_reader.read_child_by_index(0)?;
    let mut update_reader: ChunkReader = data_reader.read_child_by_index(1)?;

    let data_length: u16 = spawn_reader.read_u16::<T>()?;

    assert_eq!(data_length as u64 + 2, spawn_reader.size);

    let net_action: u16 = spawn_reader.read_u16::<T>()?;

    assert_eq!(net_action, NET_ACTION_SPAWN);

    let section: String = spawn_reader.read_w1251_string()?;
    let clsid: ClsId = ClsId::from_section(&section);
    let class: AlifeClass = AlifeClass::from_cls_id(&clsid);
    let name: String = spawn_reader.read_w1251_string()?;
    let script_game_id: u8 = spawn_reader.read_u8()?;
    let script_rp: u8 = spawn_reader.read_u8()?;
    let position: Vector3d = spawn_reader.read_xr::<T, _>()?;
    let direction: Vector3d = spawn_reader.read_xr::<T, _>()?;
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

    assert(
      version > MINIMAL_SUPPORTED_SPAWN_VERSION,
      "Unexpected version of ALife object in spawn file, flag is {script_flags}",
    )?;

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

    let inherited: Box<dyn AlifeObjectWriter> =
      AlifeClass::read_by_class::<T>(&mut spawn_reader, &class)?;

    let update_data_length: u16 = update_reader.read_u16::<T>()?;
    let update_size: u16 = update_reader.read_u16::<T>()?;

    assert_eq!(update_data_length as u64 + 2, update_reader.size);
    assert_eq!(update_size, 0);

    let update_data: Vec<u8> =
      update_reader.read_bytes(update_reader.read_bytes_remain() as usize)?;

    assert!(index_reader.is_ended());
    assert!(data_reader.is_ended());
    assert!(spawn_reader.is_ended());
    assert!(update_reader.is_ended());
    assert!(reader.is_ended());

    Ok(Self {
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
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    let mut index_writer: ChunkWriter = ChunkWriter::new();
    let mut data_writer: ChunkWriter = ChunkWriter::new();

    let mut data_spawn_writer: ChunkWriter = ChunkWriter::new();
    let mut data_update_writer: ChunkWriter = ChunkWriter::new();

    let mut object_data_writer: ChunkWriter = ChunkWriter::new();
    let mut inherited_data_writer: ChunkWriter = ChunkWriter::new();
    let mut updated_data_writer: ChunkWriter = ChunkWriter::new();

    index_writer.write_u16::<T>(self.index)?;

    object_data_writer.write_u16::<T>(self.net_action)?;

    object_data_writer.write_w1251_string(&self.section)?;
    object_data_writer.write_w1251_string(&self.name)?;
    object_data_writer.write_u8(self.script_game_id)?;
    object_data_writer.write_u8(self.script_rp)?;

    object_data_writer.write_xr::<T, Vector3d>(&self.position)?;
    object_data_writer.write_xr::<T, Vector3d>(&self.direction)?;

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

  /// Import alife object data from ltx file section.
  pub fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "ALife object base '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    let object_section: String = read_ltx_field("section", section)?;
    let clsid: ClsId = ClsId::from_section(&object_section);

    Ok(Self {
      index: read_ltx_field("index", section)?,
      id: read_ltx_field("id", section)?,
      net_action: read_ltx_field("net_action", section)?,
      clsid: clsid.clone(),
      section: object_section,
      name: read_ltx_field("name", section)?,
      script_game_id: read_ltx_field("script_game_id", section)?,
      script_rp: read_ltx_field("script_rp", section)?,
      position: read_ltx_field("position", section)?,
      direction: read_ltx_field("direction", section)?,
      respawn_time: read_ltx_field("respawn_time", section)?,
      parent_id: read_ltx_field("parent_id", section)?,
      phantom_id: read_ltx_field("phantom_id", section)?,
      script_flags: read_ltx_field("script_flags", section)?,
      version: read_ltx_field("version", section)?,
      game_type: read_ltx_field("game_type", section)?,
      script_version: read_ltx_field("script_version", section)?,
      client_data_size: read_ltx_field("client_data_size", section)?,
      spawn_id: read_ltx_field("spawn_id", section)?,
      inherited: AlifeClass::import_by_class(&AlifeClass::from_cls_id(&clsid), section_name, ltx)?,
      update_data: decode_bytes_from_base64(&read_ltx_field::<String>("update_data", section)?)?,
    })
  }

  /// Export alife object data into ltx file.
  pub fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
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

    self.inherited.export(section_name, ltx)?;

    ltx
      .with_section(section_name)
      .set("update_data", encode_bytes_to_base64(&self.update_data));

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_base::AlifeObjectBase;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_item::AlifeObjectItem;
  use crate::data::alife::alife_object_item_custom_outfit::AlifeObjectItemCustomOutfit;
  use crate::data::generic::vector_3d::Vector3d;
  use crate::data::meta::cls_id::ClsId;
  use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };
  use xray_test_utils::FileSlice;

  #[test]
  fn test_read_write() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: AlifeObjectBase = AlifeObjectBase {
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

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 203);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 203);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 203 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read: AlifeObjectBase = AlifeObjectBase::read::<XRayByteOrder>(&mut reader)?;

    assert_eq!(read.index, original.index);
    assert_eq!(read.id, original.id);
    assert_eq!(read.net_action, original.net_action);
    assert_eq!(read.section, original.section);
    assert_eq!(read.clsid, original.clsid);
    assert_eq!(read.name, original.name);
    assert_eq!(read.script_game_id, original.script_game_id);
    assert_eq!(read.script_rp, original.script_rp);
    assert_eq!(read.position, original.position);
    assert_eq!(read.direction, original.direction);
    assert_eq!(read.respawn_time, original.respawn_time);
    assert_eq!(read.parent_id, original.parent_id);
    assert_eq!(read.phantom_id, original.phantom_id);
    assert_eq!(read.script_flags, original.script_flags);
    assert_eq!(read.version, original.version);
    assert_eq!(read.game_type, original.game_type);
    assert_eq!(read.script_version, original.script_version);
    assert_eq!(read.client_data_size, original.client_data_size);
    assert_eq!(read.spawn_id, original.spawn_id);
    assert_eq!(read.update_data, original.update_data);

    Ok(())
  }
}
