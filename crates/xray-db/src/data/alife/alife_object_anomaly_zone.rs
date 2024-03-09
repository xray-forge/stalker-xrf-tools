use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::export::file_import::read_ini_field;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeObjectAnomalyZone {
  #[serde(rename = "base")]
  pub base: AlifeObjectCustomZone,
  #[serde(rename = "offlineInteractiveRadius")]
  pub offline_interactive_radius: f32,
  #[serde(rename = "artefactSpawnCount")]
  pub artefact_spawn_count: u16,
  #[serde(rename = "artefactPositionOffset")]
  pub artefact_position_offset: u32,
}

impl AlifeObjectInheritedReader<AlifeObjectAnomalyZone> for AlifeObjectAnomalyZone {
  /// Read anomaly zone object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectAnomalyZone> {
    Ok(AlifeObjectAnomalyZone {
      base: AlifeObjectCustomZone::read::<T>(reader)?,
      offline_interactive_radius: reader.read_f32::<T>()?,
      artefact_spawn_count: reader.read_u16::<T>()?,
      artefact_position_offset: reader.read_u32::<T>()?,
    })
  }

  /// Import anomaly zone object data from ini config section.
  fn import(section: &Section) -> io::Result<AlifeObjectAnomalyZone> {
    Ok(AlifeObjectAnomalyZone {
      base: AlifeObjectCustomZone::import(section)?,
      offline_interactive_radius: read_ini_field("offline_interactive_radius", section)?,
      artefact_spawn_count: read_ini_field("artefact_spawn_count", section)?,
      artefact_position_offset: read_ini_field("artefact_position_offset", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectAnomalyZone {
  /// Write anomaly zone object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_f32::<SpawnByteOrder>(self.offline_interactive_radius)?;
    writer.write_u16::<SpawnByteOrder>(self.artefact_spawn_count)?;
    writer.write_u32::<SpawnByteOrder>(self.artefact_position_offset)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) {
    self.base.export(section, ini);

    ini
      .with_section(section)
      .set(
        "offline_interactive_radius",
        self.offline_interactive_radius.to_string(),
      )
      .set(
        "artefact_spawn_count",
        self.artefact_spawn_count.to_string(),
      )
      .set(
        "artefact_position_offset",
        self.artefact_position_offset.to_string(),
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
  use crate::data::shape::Shape;
  use crate::data::vector_3d::Vector3d;
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_relative_test_sample_file_path(file!(), "alife_object_anomaly_zone.chunk");

    let object: AlifeObjectAnomalyZone = AlifeObjectAnomalyZone {
      base: AlifeObjectCustomZone {
        base: AlifeObjectSpaceRestrictor {
          base: AlifeObjectAbstract {
            game_vertex_id: 241,
            distance: 55.3,
            direct_control: 4,
            level_vertex_id: 87,
            flags: 12,
            custom_data: "".to_string(),
            story_id: 6211,
            spawn_story_id: 143,
          },
          shape: vec![
            Shape::Sphere((Vector3d::new(0.5, 0.5, 0.5), 1.0)),
            Shape::Box((
              Vector3d::new(4.1, 1.1, 32.1),
              Vector3d::new(1.1, 2.2, 3.3),
              Vector3d::new(4.0, 5.0, 1.4),
              Vector3d::new(9.2, 8.3, 1.0),
            )),
          ],
          restrictor_type: 4,
        },
        max_power: 255.33,
        owner_id: 1,
        enabled_time: 3312,
        disabled_time: 521,
        start_time_shift: 250,
      },
      offline_interactive_radius: -3231.1,
      artefact_spawn_count: 3,
      artefact_position_offset: 5,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 125);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 125);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 125 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectAnomalyZone =
      AlifeObjectAnomalyZone::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
