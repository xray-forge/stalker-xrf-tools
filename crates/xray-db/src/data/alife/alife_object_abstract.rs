use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::export::file_import::read_ini_field;
use crate::export::string::{string_from_base64, string_to_base64};
use crate::types::{DatabaseResult, SpawnByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

/// Generic alife object abstraction data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectAbstract {
  pub game_vertex_id: u16,
  pub distance: f32,
  pub direct_control: u32,
  pub level_vertex_id: u32,
  pub flags: u32,
  pub custom_data: String,
  pub story_id: u32,
  pub spawn_story_id: u32,
}

impl AlifeObjectInheritedReader<AlifeObjectAbstract> for AlifeObjectAbstract {
  /// Read generic alife object base data from the chunk reader.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<AlifeObjectAbstract> {
    Ok(AlifeObjectAbstract {
      game_vertex_id: reader.read_u16::<T>()?,
      distance: reader.read_f32::<T>()?,
      direct_control: reader.read_u32::<T>()?,
      level_vertex_id: reader.read_u32::<T>()?,
      flags: reader.read_u32::<T>()?,
      custom_data: reader.read_null_terminated_win_string()?,
      story_id: reader.read_u32::<T>()?,
      spawn_story_id: reader.read_u32::<T>()?,
    })
  }

  /// Import generic alife object base data from ini config section.
  fn import(section: &Section) -> DatabaseResult<AlifeObjectAbstract> {
    Ok(AlifeObjectAbstract {
      game_vertex_id: read_ini_field("game_vertex_id", section)?,
      distance: read_ini_field("distance", section)?,
      direct_control: read_ini_field("direct_control", section)?,
      level_vertex_id: read_ini_field("level_vertex_id", section)?,
      flags: read_ini_field("flags", section)?,
      custom_data: string_from_base64(&read_ini_field::<String>("custom_data", section)?)?,
      story_id: read_ini_field("story_id", section)?,
      spawn_story_id: read_ini_field("spawn_story_id", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectAbstract {
  /// Write abstract object data into the chunk writer.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    writer.write_u16::<SpawnByteOrder>(self.game_vertex_id)?;
    writer.write_f32::<SpawnByteOrder>(self.distance)?;
    writer.write_u32::<SpawnByteOrder>(self.direct_control)?;
    writer.write_u32::<SpawnByteOrder>(self.level_vertex_id)?;
    writer.write_u32::<SpawnByteOrder>(self.flags)?;
    writer.write_null_terminated_win_string(&self.custom_data)?;
    writer.write_u32::<SpawnByteOrder>(self.story_id)?;
    writer.write_u32::<SpawnByteOrder>(self.spawn_story_id)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) {
    ini
      .with_section(section)
      .set("game_vertex_id", self.game_vertex_id.to_string())
      .set("distance", self.distance.to_string())
      .set("direct_control", self.direct_control.to_string())
      .set("level_vertex_id", self.level_vertex_id.to_string())
      .set("flags", self.flags.to_string())
      .set("custom_data", &string_to_base64(&self.custom_data))
      .set("story_id", self.story_id.to_string())
      .set("spawn_story_id", self.spawn_story_id.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::export::file::open_ini_config;
  use crate::types::{DatabaseResult, SpawnByteOrder};
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use xray_ltx::Ltx;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_resource_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let object: AlifeObjectAbstract = AlifeObjectAbstract {
      game_vertex_id: 1001,
      distance: 65.25,
      direct_control: 412421,
      level_vertex_id: 66231,
      flags: 33,
      custom_data: String::from("custom_data"),
      story_id: 400,
      spawn_story_id: 25,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 38);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 38);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 38 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectAbstract =
      AlifeObjectAbstract::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }

  #[test]
  fn test_import_export() -> DatabaseResult<()> {
    let first: AlifeObjectAbstract = AlifeObjectAbstract {
      game_vertex_id: 1001,
      distance: 65.25,
      direct_control: 412421,
      level_vertex_id: 66231,
      flags: 33,
      custom_data: String::from("[custom_data_section] field1 = 1\r\n field2 = 2\r\n"),
      story_id: 400,
      spawn_story_id: 25,
    };

    let second: AlifeObjectAbstract = AlifeObjectAbstract {
      game_vertex_id: 1002,
      distance: 23.376,
      direct_control: 421,
      level_vertex_id: 75486,
      flags: 6,
      custom_data: String::from(""),
      story_id: 2345,
      spawn_story_id: 255,
    };

    let exported_filename: String =
      get_relative_test_sample_file_path(file!(), "import_export.ini");
    let mut exported: Ltx = Ltx::new();

    first.export("first", &mut exported);
    second.export("second", &mut exported);

    exported.write_to(&mut overwrite_test_relative_resource_as_file(
      &exported_filename,
    )?)?;

    let source: Ltx = open_ini_config(&get_absolute_test_resource_path(&exported_filename))?;

    let read_first: AlifeObjectAbstract =
      AlifeObjectAbstract::import(source.section("first").unwrap())?;
    let read_second: AlifeObjectAbstract =
      AlifeObjectAbstract::import(source.section("second").unwrap())?;

    assert_eq!(read_first, first);
    assert_eq!(read_second, second);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> DatabaseResult<()> {
    let object: AlifeObjectAbstract = AlifeObjectAbstract {
      game_vertex_id: 1005,
      distance: 23.25,
      direct_control: 262342,
      level_vertex_id: 25341,
      flags: 34,
      custom_data: String::from("[custom_data_section] field1 = 1\r\n field2 = 2\r\n"),
      story_id: 433,
      spawn_story_id: 36,
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(json!(object).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      object,
      serde_json::from_str::<AlifeObjectAbstract>(&serialized).unwrap()
    );

    Ok(())
  }
}
