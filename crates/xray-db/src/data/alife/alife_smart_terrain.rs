use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_smart_zone::AlifeSmartZone;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, PartialEq)]
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
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeSmartTerrain> {
    let base: AlifeSmartZone = AlifeSmartZone::read_from_chunk::<T>(chunk)?;

    let arriving_objects_count: u8 = chunk.read_u8()?;

    assert_eq!(
      arriving_objects_count, 0,
      "Unexpected arriving objects in smart terrain."
    );

    let object_job_descriptors_count: u8 = chunk.read_u8()?;

    assert_eq!(
      object_job_descriptors_count, 0,
      "Unexpected job objects in smart terrain."
    );

    let dead_objects_infos_count: u8 = chunk.read_u8()?;

    assert_eq!(
      dead_objects_infos_count, 0,
      "Unexpected dead objects in smart terrain."
    );

    let smart_terrain_actor_control: u8 = chunk.read_u8()?;

    assert_eq!(
      smart_terrain_actor_control, 0,
      "Unexpected smart terrain actor control."
    );

    let respawn_point: u8 = chunk.read_u8()?;

    if respawn_point != 0 {
      panic!("Not expected respawn point handler.")
    }

    let staying_objects_count: u8 = chunk.read_u8()?;

    assert_eq!(
      staying_objects_count, 0,
      "Unexpected smart terrain staying objects."
    );

    let save_marker: u16 = chunk.read_u16::<SpawnByteOrder>()?;

    assert_eq!(
      save_marker, 6,
      "Unexpected data provided with smart terrain save."
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

  /// Write smart terrain data into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write::<T>(writer)?;

    writer.write_u8(self.arriving_objects_count)?;
    writer.write_u8(self.object_job_descriptors_count)?;
    writer.write_u8(self.dead_objects_infos_count)?;
    writer.write_u8(self.smart_terrain_actor_control)?;
    writer.write_u8(self.respawn_point)?;
    writer.write_u8(self.staying_objects_count)?;
    writer.write_u16::<T>(self.save_marker)?;

    Ok(())
  }
}

impl AlifeObjectGeneric for AlifeSmartTerrain {}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::alife::alife_smart_terrain::AlifeSmartTerrain;
  use crate::data::alife::alife_smart_zone::AlifeSmartZone;
  use crate::data::shape::Shape;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_smart_terrain.chunk"));

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
            Shape::Sphere(((3.5, -2.5, 11.5), 1.0)),
            Shape::Box((
              (1.5, 1.1, 73.1),
              (5.1, 2.2, 3.3),
              (4.0, 6.0, 2.4),
              (9.2, 4.3, 3.0),
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

    object.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 114);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 114);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 114 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeSmartTerrain =
      AlifeSmartTerrain::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
