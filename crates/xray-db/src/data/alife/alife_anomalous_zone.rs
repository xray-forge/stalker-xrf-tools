use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::time::Time;
use crate::types::SpawnByteOrder;
use byteorder::ByteOrder;
use ini::{Ini, Properties};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeAnomalousZone {
  pub base: AlifeObjectAnomalyZone,
  pub last_spawn_time: Option<Time>,
}

impl AlifeObjectInheritedReader<AlifeAnomalousZone> for AlifeAnomalousZone {
  /// Read anomalous zone object data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeAnomalousZone> {
    Ok(AlifeAnomalousZone {
      base: AlifeObjectAnomalyZone::read_from_chunk::<T>(chunk)?,
      last_spawn_time: Time::read_optional_from_chunk::<T>(chunk)?,
    })
  }

  /// Import anomalous zone object data from ini config section.
  fn import(props: &Properties) -> io::Result<AlifeAnomalousZone> {
    Ok(AlifeAnomalousZone {
      base: AlifeObjectAnomalyZone::import(props)?,
      last_spawn_time: None, // todo: Read actual time object.
    })
  }
}

impl AlifeObjectGeneric for AlifeAnomalousZone {
  type Order = SpawnByteOrder;

  /// Write alife anomalous zone data to the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    Time::write_optional::<Self::Order>(&self.last_spawn_time, writer)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ini) {
    self.base.export(section, ini);

    ini.with_section(Some(section)).set(
      "last_spawn_time",
      &Time::export_to_string(&self.last_spawn_time),
    );
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_anomalous_zone::AlifeAnomalousZone;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
  use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::shape::Shape;
  use crate::data::time::Time;
  use crate::data::vector_3d::Vector3d;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_test_chunk_file_sub_dir(file!(), "alife_anomalous_zone.chunk");

    let object: AlifeAnomalousZone = AlifeAnomalousZone {
      base: AlifeObjectAnomalyZone {
        base: AlifeObjectCustomZone {
          base: AlifeObjectSpaceRestrictor {
            base: AlifeObjectAbstract {
              game_vertex_id: 34565,
              distance: 234.0,
              direct_control: 2346,
              level_vertex_id: 7357,
              flags: 55,
              custom_data: String::from("custom-data"),
              story_id: 8567,
              spawn_story_id: 7685,
            },
            shape: vec![
              Shape::Sphere((Vector3d::new(2.5, 5.1, 1.5), 1.0)),
              Shape::Box((
                Vector3d::new(4.1, 1.1, 3.1),
                Vector3d::new(1.1, 3.2, 3.3),
                Vector3d::new(4.0, 5.0, 6.4),
                Vector3d::new(9.2, 8.3, 3.0),
              )),
            ],
            restrictor_type: 4,
          },
          max_power: 1.0,
          owner_id: 64,
          enabled_time: 235,
          disabled_time: 3457,
          start_time_shift: 253,
        },
        offline_interactive_radius: 330.0,
        artefact_spawn_count: 4,
        artefact_position_offset: 12,
      },
      last_spawn_time: Some(Time {
        year: 22,
        month: 10,
        day: 24,
        hour: 20,
        minute: 30,
        second: 50,
        millis: 250,
      }),
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 145);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 145);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 145 + 8);

    let mut chunk: Chunk = Chunk::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeAnomalousZone =
      AlifeAnomalousZone::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
