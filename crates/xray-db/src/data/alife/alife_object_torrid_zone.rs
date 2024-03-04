use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_motion::AlifeObjectMotion;
use crate::data::time::Time;
use crate::export::file_import::read_ini_field;
use crate::types::SpawnByteOrder;
use byteorder::ByteOrder;
use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeObjectTorridZone {
  pub base: AlifeObjectCustomZone,
  pub motion: AlifeObjectMotion,
  pub last_spawn_time: Option<Time>,
}

impl AlifeObjectInheritedReader<AlifeObjectTorridZone> for AlifeObjectTorridZone {
  /// Read zone object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectTorridZone> {
    Ok(AlifeObjectTorridZone {
      base: AlifeObjectCustomZone::read::<T>(reader)?,
      motion: AlifeObjectMotion::read::<T>(reader)?,
      last_spawn_time: Time::read_optional::<T>(reader)?,
    })
  }

  /// Import torrid zone object data from ini config section.
  fn import(props: &Properties) -> io::Result<AlifeObjectTorridZone> {
    Ok(AlifeObjectTorridZone {
      base: AlifeObjectCustomZone::import(props)?,
      motion: AlifeObjectMotion::import(props)?,
      last_spawn_time: Time::import_from_string(&read_ini_field::<String>(
        "last_spawn_time",
        props,
      )?)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectTorridZone {
  /// Write zone object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;
    self.motion.write(writer)?;

    Time::write_optional::<SpawnByteOrder>(&self.last_spawn_time, writer)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ini) {
    self.base.export(section, ini);
    self.motion.export(section, ini);

    ini.with_section(Some(section)).set(
      "last_spawn_time",
      Time::export_to_string(&self.last_spawn_time),
    );
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
  use crate::data::alife::alife_object_motion::AlifeObjectMotion;
  use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::alife::alife_object_torrid_zone::AlifeObjectTorridZone;
  use crate::data::time::Time;
  use crate::test::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_relative_test_sample_file_path(file!(), "alife_object_torrid_zone.chunk");

    let object: AlifeObjectTorridZone = AlifeObjectTorridZone {
      base: AlifeObjectCustomZone {
        base: AlifeObjectSpaceRestrictor {
          base: AlifeObjectAbstract {
            game_vertex_id: 8469,
            distance: 85.323,
            direct_control: 203678,
            level_vertex_id: 8726,
            flags: 76,
            custom_data: String::from("custom-data"),
            story_id: 295786,
            spawn_story_id: 620,
          },
          shape: vec![],
          restrictor_type: 4,
        },
        max_power: 1.0,
        owner_id: 286,
        enabled_time: 1,
        disabled_time: 1,
        start_time_shift: 1,
      },
      motion: AlifeObjectMotion {
        motion_name: String::from("motion-name"),
      },
      last_spawn_time: Some(Time {
        year: 12,
        month: 6,
        day: 3,
        hour: 24,
        minute: 3,
        second: 30,
        millis: 500,
      }),
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 81);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 81);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 81 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectTorridZone =
      AlifeObjectTorridZone::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
