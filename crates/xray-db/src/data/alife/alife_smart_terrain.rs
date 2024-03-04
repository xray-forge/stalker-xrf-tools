use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_smart_zone::AlifeSmartZone;
use crate::export::file_import::read_ini_field;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeSmartTerrain {
  pub base: AlifeSmartZone,
  pub arriving_objects_count: u8,
  pub object_job_descriptors_count: u8,
  pub dead_objects_infos_count: u8,
  pub smart_terrain_actor_control: u8,
  pub respawn_point: u8,
  pub staying_objects_count: u8,
  pub save_marker: u16,
}

impl AlifeObjectInheritedReader<AlifeSmartTerrain> for AlifeSmartTerrain {
  /// Read alife smart terrain data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeSmartTerrain> {
    let base: AlifeSmartZone = AlifeSmartZone::read::<T>(reader)?;

    let arriving_objects_count: u8 = reader.read_u8()?;

    assert_eq!(
      arriving_objects_count, 0,
      "Unexpected arriving objects in smart terrain"
    );

    let object_job_descriptors_count: u8 = reader.read_u8()?;

    assert_eq!(
      object_job_descriptors_count, 0,
      "Unexpected job objects in smart terrain"
    );

    let dead_objects_infos_count: u8 = reader.read_u8()?;

    assert_eq!(
      dead_objects_infos_count, 0,
      "Unexpected dead objects in smart terrain"
    );

    let smart_terrain_actor_control: u8 = reader.read_u8()?;

    assert_eq!(
      smart_terrain_actor_control, 0,
      "Unexpected smart terrain actor control"
    );

    let respawn_point: u8 = reader.read_u8()?;

    if respawn_point != 0 {
      panic!("Not expected respawn point handler")
    }

    let staying_objects_count: u8 = reader.read_u8()?;

    assert_eq!(
      staying_objects_count, 0,
      "Unexpected smart terrain staying objects"
    );

    let save_marker: u16 = reader.read_u16::<SpawnByteOrder>()?;

    assert_eq!(
      save_marker, 6,
      "Unexpected data provided with smart terrain save"
    );

    Ok(AlifeSmartTerrain {
      base,
      arriving_objects_count,
      object_job_descriptors_count,
      dead_objects_infos_count,
      smart_terrain_actor_control,
      respawn_point,
      staying_objects_count,
      save_marker,
    })
  }

  /// Import alife smart terrain data from ini config section.
  fn import(props: &Properties) -> io::Result<AlifeSmartTerrain> {
    Ok(AlifeSmartTerrain {
      base: AlifeSmartZone::import(props)?,
      arriving_objects_count: read_ini_field("arriving_objects_count", props)?,
      object_job_descriptors_count: read_ini_field("object_job_descriptors_count", props)?,
      dead_objects_infos_count: read_ini_field("dead_objects_infos_count", props)?,
      smart_terrain_actor_control: read_ini_field("smart_terrain_actor_control", props)?,
      respawn_point: read_ini_field("respawn_point", props)?,
      staying_objects_count: read_ini_field("staying_objects_count", props)?,
      save_marker: read_ini_field("save_marker", props)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeSmartTerrain {
  /// Write smart terrain data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_u8(self.arriving_objects_count)?;
    writer.write_u8(self.object_job_descriptors_count)?;
    writer.write_u8(self.dead_objects_infos_count)?;
    writer.write_u8(self.smart_terrain_actor_control)?;
    writer.write_u8(self.respawn_point)?;
    writer.write_u8(self.staying_objects_count)?;
    writer.write_u16::<SpawnByteOrder>(self.save_marker)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ini) {
    self.base.export(section, ini);

    ini
      .with_section(Some(section))
      .set(
        "arriving_objects_count",
        self.arriving_objects_count.to_string(),
      )
      .set(
        "object_job_descriptors_count",
        self.object_job_descriptors_count.to_string(),
      )
      .set(
        "dead_objects_infos_count",
        self.dead_objects_infos_count.to_string(),
      )
      .set(
        "smart_terrain_actor_control",
        self.smart_terrain_actor_control.to_string(),
      )
      .set("respawn_point", self.respawn_point.to_string())
      .set(
        "staying_objects_count",
        self.staying_objects_count.to_string(),
      )
      .set("save_marker", self.save_marker.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::alife::alife_smart_terrain::AlifeSmartTerrain;
  use crate::data::alife::alife_smart_zone::AlifeSmartZone;
  use crate::data::shape::Shape;
  use crate::data::vector_3d::Vector3d;
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
    let filename: String = get_relative_test_sample_file_path(file!(), "alife_smart_terrain.chunk");

    let object: AlifeSmartTerrain = AlifeSmartTerrain {
      base: AlifeSmartZone {
        base: AlifeObjectSpaceRestrictor {
          base: AlifeObjectAbstract {
            game_vertex_id: 1002,
            distance: 65.25,
            direct_control: 31231,
            level_vertex_id: 3213,
            flags: 34,
            custom_data: String::from("custom_data"),
            story_id: 400,
            spawn_story_id: 25,
          },
          shape: vec![
            Shape::Sphere((Vector3d::new(3.5, -2.5, 11.5), 1.0)),
            Shape::Box((
              Vector3d::new(1.5, 1.1, 73.1),
              Vector3d::new(5.1, 2.2, 3.3),
              Vector3d::new(4.0, 6.0, 2.4),
              Vector3d::new(9.2, 4.3, 3.0),
            )),
          ],
          restrictor_type: 2,
        },
      },
      arriving_objects_count: 0,
      object_job_descriptors_count: 0,
      dead_objects_infos_count: 0,
      smart_terrain_actor_control: 0,
      respawn_point: 0,
      staying_objects_count: 0,
      save_marker: 6,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 114);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 114);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 114 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeSmartTerrain = AlifeSmartTerrain::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
