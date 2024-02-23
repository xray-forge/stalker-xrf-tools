use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::time::Time;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeObjectAnomalyZone {
  pub base: AlifeObjectCustomZone,
  pub offline_interactive_radius: f32,
  pub artefact_spawn_count: u16,
  pub artefact_position_offset: u32,
  pub last_spawn_time: Option<Time>,
}

impl AlifeObjectInheritedReader<AlifeObjectAnomalyZone> for AlifeObjectAnomalyZone {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectAnomalyZone> {
    let base: AlifeObjectCustomZone = AlifeObjectCustomZone::read_from_chunk::<T>(chunk)?;

    let offline_interactive_radius: f32 = chunk.read_f32::<SpawnByteOrder>()?;
    let artefact_spawn_count: u16 = chunk.read_u16::<SpawnByteOrder>()?;
    let artefact_position_offset: u32 = chunk.read_u32::<SpawnByteOrder>()?;

    // Last spawn time for artefacts, legacy approach:
    let last_spawn_time: Option<Time> = if chunk.is_ended() || chunk.read_u8()? == 0 {
      None
    } else {
      Some(Time::read_from_chunk::<SpawnByteOrder>(chunk)?)
    };

    Ok(AlifeObjectAnomalyZone {
      base,
      offline_interactive_radius,
      artefact_spawn_count,
      artefact_position_offset,
      last_spawn_time,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeObjectAnomalyZone {}
