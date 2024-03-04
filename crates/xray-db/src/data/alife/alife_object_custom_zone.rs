use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
use crate::export::file_import::read_ini_field;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeObjectCustomZone {
  pub base: AlifeObjectSpaceRestrictor,
  pub max_power: f32,
  pub owner_id: u32,
  pub enabled_time: u32,
  pub disabled_time: u32,
  pub start_time_shift: u32,
}

impl AlifeObjectInheritedReader<AlifeObjectCustomZone> for AlifeObjectCustomZone {
  /// Read alife custom zone object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectCustomZone> {
    Ok(AlifeObjectCustomZone {
      base: AlifeObjectSpaceRestrictor::read::<T>(reader)?,
      max_power: reader.read_f32::<SpawnByteOrder>()?,
      owner_id: reader.read_u32::<SpawnByteOrder>()?,
      enabled_time: reader.read_u32::<SpawnByteOrder>()?,
      disabled_time: reader.read_u32::<SpawnByteOrder>()?,
      start_time_shift: reader.read_u32::<SpawnByteOrder>()?,
    })
  }

  /// Import alife custom zone object data from ini config section..
  fn import(props: &Properties) -> io::Result<AlifeObjectCustomZone> {
    Ok(AlifeObjectCustomZone {
      base: AlifeObjectSpaceRestrictor::import(props)?,
      max_power: read_ini_field("max_power", props)?,
      owner_id: read_ini_field("owner_id", props)?,
      enabled_time: read_ini_field("enabled_time", props)?,
      disabled_time: read_ini_field("disabled_time", props)?,
      start_time_shift: read_ini_field("start_time_shift", props)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectCustomZone {
  /// Write custom zone object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_f32::<SpawnByteOrder>(self.max_power)?;
    writer.write_u32::<SpawnByteOrder>(self.owner_id)?;
    writer.write_u32::<SpawnByteOrder>(self.enabled_time)?;
    writer.write_u32::<SpawnByteOrder>(self.disabled_time)?;
    writer.write_u32::<SpawnByteOrder>(self.start_time_shift)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ini) {
    self.base.export(section, ini);

    ini
      .with_section(Some(section))
      .set("max_power", self.max_power.to_string())
      .set("owner_id", self.owner_id.to_string())
      .set("enabled_time", self.enabled_time.to_string())
      .set("disabled_time", self.disabled_time.to_string())
      .set("start_time_shift", self.start_time_shift.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::shape::Shape;
  use crate::data::vector_3d::Vector3d;
  use crate::test::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_relative_test_sample_file_path(file!(), "alife_object_custom_zone.chunk");

    let object: AlifeObjectCustomZone = AlifeObjectCustomZone {
      base: AlifeObjectSpaceRestrictor {
        base: AlifeObjectAbstract {
          game_vertex_id: 42343,
          distance: 255.4,
          direct_control: 3,
          level_vertex_id: 1003,
          flags: 32,
          custom_data: String::from("custom-data"),
          story_id: 441,
          spawn_story_id: 254,
        },
        shape: vec![
          Shape::Sphere((Vector3d::new(2.5, 3.5, 1.5), 1.0)),
          Shape::Box((
            Vector3d::new(1.1, 1.1, 3.1),
            Vector3d::new(1.1, 2.2, 3.3),
            Vector3d::new(4.0, 5.0, 6.4),
            Vector3d::new(9.2, 8.3, 7.0),
          )),
        ],
        restrictor_type: 3,
      },
      max_power: 2.0,
      owner_id: 553,
      enabled_time: 100,
      disabled_time: 220,
      start_time_shift: 300,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 126);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 126);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 126 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectCustomZone =
      AlifeObjectCustomZone::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
