use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_motion::AlifeObjectMotion;
use crate::data::time::Time;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeObjectTorridZone {
  pub base: AlifeObjectCustomZone,
  pub motion: AlifeObjectMotion,
  pub last_spawn_time: Option<Time>,
}

impl AlifeObjectInheritedReader<AlifeObjectTorridZone> for AlifeObjectTorridZone {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectTorridZone> {
    let base: AlifeObjectCustomZone = AlifeObjectCustomZone::read_from_chunk::<T>(chunk)?;
    let motion: AlifeObjectMotion = AlifeObjectMotion::read_from_chunk::<T>(chunk)?;

    // Last spawn time for artefacts, legacy approach:
    let last_spawn_time: Option<Time> = if chunk.is_ended() || chunk.read_u8()? == 0 {
      None
    } else {
      Some(Time::read_from_chunk::<SpawnByteOrder>(chunk)?)
    };

    Ok(AlifeObjectTorridZone {
      base,
      motion,
      last_spawn_time,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeObjectTorridZone {}
