use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeObjectCustomZone {
  pub base: AlifeObjectSpaceRestrictor,
  pub max_power: f32,
  pub owner_id: u32,
  pub enabled_time: u32,
  pub disabled_time: u32,
  pub m_start_time_shift: u32,
}

impl AlifeObjectInheritedReader<AlifeObjectCustomZone> for AlifeObjectCustomZone {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectCustomZone> {
    let base: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor::read_from_chunk::<T>(chunk)?;

    let max_power: f32 = chunk.read_f32::<SpawnByteOrder>()?;
    let owner_id: u32 = chunk.read_u32::<SpawnByteOrder>()?;
    let enabled_time: u32 = chunk.read_u32::<SpawnByteOrder>()?;
    let disabled_time: u32 = chunk.read_u32::<SpawnByteOrder>()?;
    let m_start_time_shift: u32 = chunk.read_u32::<SpawnByteOrder>()?;

    Ok(AlifeObjectCustomZone {
      base,
      max_power,
      owner_id,
      enabled_time,
      disabled_time,
      m_start_time_shift,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}
