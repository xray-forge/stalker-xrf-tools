use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::export::file_import::read_ini_field;
use crate::export::string::{string_from_base64, string_to_base64};
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use std::io;

/// Generic alife object abstraction data.
#[derive(Clone, Debug, PartialEq)]
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
  /// Read generic alife object base data from the file.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectAbstract> {
    Ok(AlifeObjectAbstract {
      game_vertex_id: chunk.read_u16::<T>()?,
      distance: chunk.read_f32::<T>()?,
      direct_control: chunk.read_u32::<T>()?,
      level_vertex_id: chunk.read_u32::<T>()?,
      flags: chunk.read_u32::<T>()?,
      custom_data: chunk.read_null_terminated_win_string()?,
      story_id: chunk.read_u32::<T>()?,
      spawn_story_id: chunk.read_u32::<T>()?,
    })
  }

  /// Import generic alife object base data from ini config section.
  fn import(props: &Properties) -> io::Result<AlifeObjectAbstract> {
    Ok(AlifeObjectAbstract {
      game_vertex_id: read_ini_field("game_vertex_id", props)?,
      distance: read_ini_field("distance", props)?,
      direct_control: read_ini_field("direct_control", props)?,
      level_vertex_id: read_ini_field("level_vertex_id", props)?,
      flags: read_ini_field("flags", props)?,
      custom_data: string_from_base64(&read_ini_field::<String>("custom_data", props)?)?,
      story_id: read_ini_field("story_id", props)?,
      spawn_story_id: read_ini_field("spawn_story_id", props)?,
    })
  }
}

impl AlifeObjectGeneric for AlifeObjectAbstract {
  type Order = SpawnByteOrder;

  /// Write abstract object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_u16::<Self::Order>(self.game_vertex_id)?;
    writer.write_f32::<Self::Order>(self.distance)?;
    writer.write_u32::<Self::Order>(self.direct_control)?;
    writer.write_u32::<Self::Order>(self.level_vertex_id)?;
    writer.write_u32::<Self::Order>(self.flags)?;
    writer.write_null_terminated_win_string(&self.custom_data)?;
    writer.write_u32::<Self::Order>(self.story_id)?;
    writer.write_u32::<Self::Order>(self.spawn_story_id)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ini) {
    ini
      .with_section(Some(section))
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
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::export::file::{export_ini_to_file, open_ini_config};
  use crate::test::assertions::files_are_equal_by_path;
  use crate::test::utils::{
    get_test_resource_path, get_test_sample_file_sub_dir, open_test_resource_as_slice,
    overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use ini::Ini;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_test_sample_file_sub_dir(file!(), "alife_object_abstract.chunk");

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
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 38);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 38 + 8);

    let mut chunk: Chunk = Chunk::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectAbstract =
      AlifeObjectAbstract::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }

  #[test]
  fn test_import_export_object() -> io::Result<()> {
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

    let exported_filename: String = get_test_sample_file_sub_dir(file!(), "exported.ini");
    let mut exported: Ini = Ini::new();

    first.export("first", &mut exported);
    second.export("second", &mut exported);

    export_ini_to_file(
      &exported,
      &mut overwrite_test_resource_as_file(&exported_filename)?,
    )?;

    let source: Ini = open_ini_config(&get_test_resource_path(&exported_filename))?;

    let read_first: AlifeObjectAbstract =
      AlifeObjectAbstract::import(source.section(Some("first")).unwrap())?;
    let read_second: AlifeObjectAbstract =
      AlifeObjectAbstract::import(source.section(Some("second")).unwrap())?;

    assert_eq!(read_first, first);
    assert_eq!(read_second, second);

    let imported_filename: String = get_test_sample_file_sub_dir(file!(), "imported.ini");
    let mut imported: Ini = Ini::new();

    read_first.export("first", &mut imported);
    read_second.export("second", &mut imported);

    export_ini_to_file(
      &imported,
      &mut overwrite_test_resource_as_file(&imported_filename)?,
    )?;

    assert!(files_are_equal_by_path(
      get_test_resource_path(&exported_filename),
      get_test_resource_path(&imported_filename)
    )?);

    Ok(())
  }
}
