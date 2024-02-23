use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_motion::AlifeObjectMotion;
use crate::data::time::Time;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeObjectTorridZone {
  pub base: AlifeObjectCustomZone,
  pub motion: AlifeObjectMotion,
  pub last_spawn_time: Option<Time>,
}

impl AlifeObjectInheritedReader<AlifeObjectTorridZone> for AlifeObjectTorridZone {
  fn read_from_chunk(chunk: &mut Chunk) -> AlifeObjectTorridZone {
    let base: AlifeObjectCustomZone = AlifeObjectCustomZone::read_from_chunk(chunk);
    let motion: AlifeObjectMotion = AlifeObjectMotion::read_from_chunk(chunk);

    // Last spawn time for artefacts, legacy approach:
    let last_spawn_time: Option<Time> = if chunk.is_ended() || chunk.read_u8().unwrap() == 0 {
      None
    } else {
      Some(Time::read_from_chunk::<SpawnByteOrder>(chunk).unwrap())
    };

    AlifeObjectTorridZone {
      base,
      motion,
      last_spawn_time,
    }
  }
}

impl AlifeObjectGeneric for AlifeObjectTorridZone {}
