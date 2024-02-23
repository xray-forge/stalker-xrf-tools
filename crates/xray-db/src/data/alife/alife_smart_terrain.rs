use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_smart_zone::AlifeSmartZone;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeSmartTerrain {
  pub base: AlifeSmartZone,
  pub arriving_objects_count: u8,
  pub object_job_descriptors_count: u8,
  pub dead_objects_infos_count: u8,
  pub smart_terrain_actor_control: u8,
  pub respawn_point: u8,
  pub staying_objects_count: u8,
}

impl AlifeObjectInheritedReader<AlifeSmartTerrain> for AlifeSmartTerrain {
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
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeSmartTerrain {}
