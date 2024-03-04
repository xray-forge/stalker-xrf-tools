use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use crate::data::time::Time;
use crate::export::file_import::read_ini_field;
use crate::types::SpawnByteOrder;
use byteorder::ByteOrder;
use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeZoneVisual {
  pub base: AlifeObjectAnomalyZone,
  pub visual: AlifeObjectVisual,
  pub idle_animation: String,
  pub attack_animation: String,
  pub last_spawn_time: Option<Time>,
}

impl AlifeObjectInheritedReader<AlifeZoneVisual> for AlifeZoneVisual {
  /// Read visual zone data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeZoneVisual> {
    Ok(AlifeZoneVisual {
      base: AlifeObjectAnomalyZone::read::<T>(reader)?,
      visual: AlifeObjectVisual::read::<T>(reader)?,
      idle_animation: reader
        .has_data()
        .then(|| reader.read_null_terminated_win_string().unwrap())
        .unwrap_or(String::new()),
      attack_animation: reader
        .has_data()
        .then(|| reader.read_null_terminated_win_string().unwrap())
        .unwrap_or(String::new()),
      last_spawn_time: Time::read_optional::<T>(reader)?,
    })
  }

  /// Import visual zone data from ini config section.
  fn import(props: &Properties) -> io::Result<AlifeZoneVisual> {
    Ok(AlifeZoneVisual {
      base: AlifeObjectAnomalyZone::import(props)?,
      visual: AlifeObjectVisual::import(props)?,
      idle_animation: read_ini_field("idle_animation", props)?,
      attack_animation: read_ini_field("attack_animation", props)?,
      last_spawn_time: Time::import_from_string(&read_ini_field::<String>(
        "last_spawn_time",
        props,
      )?)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeZoneVisual {
  /// Write visual zone data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;
    self.visual.write(writer)?;

    writer.write_null_terminated_win_string(&self.idle_animation)?;
    writer.write_null_terminated_win_string(&self.attack_animation)?;

    Time::write_optional::<SpawnByteOrder>(&self.last_spawn_time, writer)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ini) {
    self.base.export(section, ini);
    self.visual.export(section, ini);

    ini
      .with_section(Some(section))
      .set("idle_animation", &self.idle_animation)
      .set("attack_animation", &self.attack_animation)
      .set(
        "last_spawn_time",
        &Time::export_to_string(&self.last_spawn_time),
      );
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
  use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::alife::alife_object_visual::AlifeObjectVisual;
  use crate::data::alife::alife_zone_visual::AlifeZoneVisual;
  use crate::data::shape::Shape;
  use crate::data::vector_3d::Vector3d;
  use crate::test::utils::{
    get_test_sample_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_test_sample_file_sub_dir(file!(), "alife_object_zone_visual.chunk");

    let object: AlifeZoneVisual = AlifeZoneVisual {
      base: AlifeObjectAnomalyZone {
        base: AlifeObjectCustomZone {
          base: AlifeObjectSpaceRestrictor {
            base: AlifeObjectAbstract {
              game_vertex_id: 4500,
              distance: 3.125,
              direct_control: 1,
              level_vertex_id: 40,
              flags: 33,
              custom_data: String::from("custom_data"),
              story_id: 420,
              spawn_story_id: 101,
            },
            shape: vec![
              Shape::Sphere((Vector3d::new(3.5, 0.5, 11.5), 1.0)),
              Shape::Box((
                Vector3d::new(1.5, 1.1, 3.1),
                Vector3d::new(1.1, 2.2, 3.3),
                Vector3d::new(4.0, 5.0, 6.4),
                Vector3d::new(9.2, 8.3, 3.0),
              )),
            ],
            restrictor_type: 2,
          },
          max_power: 12.0,
          owner_id: 638,
          enabled_time: 100,
          disabled_time: 253,
          start_time_shift: 40,
        },
        offline_interactive_radius: 274.621,
        artefact_spawn_count: 3,
        artefact_position_offset: 40,
      },
      visual: AlifeObjectVisual {
        visual_name: String::from("visual_name"),
        visual_flags: 36,
      },
      idle_animation: String::from("idle_animation"),
      attack_animation: String::from("attack_animation"),
      last_spawn_time: None,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 182);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 182);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 182 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeZoneVisual = AlifeZoneVisual::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
