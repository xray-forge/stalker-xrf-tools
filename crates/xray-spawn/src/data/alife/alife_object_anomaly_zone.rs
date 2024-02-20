use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::time::Time;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeObjectAnomalyZone {
  pub base: AlifeObjectCustomZone,
  pub offline_interactive_radius: f32,
  pub artefact_spawn_count: u16,
  pub artefact_position_offset: u32,
  pub last_spawn_time: Option<Time>,
}

impl AlifeObjectInheritedReader<AlifeObjectAnomalyZone> for AlifeObjectAnomalyZone {
  fn read_from_chunk(chunk: &mut Chunk) -> AlifeObjectAnomalyZone {
    let base: AlifeObjectCustomZone = AlifeObjectCustomZone::read_from_chunk(chunk);

    let offline_interactive_radius: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let artefact_spawn_count: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();
    let artefact_position_offset: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();

    // Last spawn time for artefacts, legacy approach:
    let last_spawn_time: Option<Time> = if chunk.is_ended() || chunk.read_u8().unwrap() == 0 {
      None
    } else {
      Time::read_from_chunk(chunk)
    };

    AlifeObjectAnomalyZone {
      base,
      offline_interactive_radius,
      artefact_spawn_count,
      artefact_position_offset,
      last_spawn_time,
    }
  }
}

impl AlifeObjectGeneric for AlifeObjectAnomalyZone {}
