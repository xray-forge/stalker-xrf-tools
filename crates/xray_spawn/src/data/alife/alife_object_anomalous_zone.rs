use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
use crate::data::alife_object::AlifeObjectInherited;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeObjectAnomalyZone {
  pub base: AlifeObjectCustomZone,
  pub offline_interactive_radius: f32,
  pub artefact_spawn_count: u16,
  pub artefact_position_offset: u32,
}

impl AlifeObjectInherited<AlifeObjectAnomalyZone> for AlifeObjectAnomalyZone {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectAnomalyZone {
    let base: AlifeObjectCustomZone = AlifeObjectCustomZone::from_chunk(chunk);

    let offline_interactive_radius: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let artefact_spawn_count: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();
    let artefact_position_offset: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();

    // Unknown value - even oxray omits it after first verification with read-write cycle.
    let _dummy: u8 = chunk.read_u8().unwrap();

    // Self::verify(chunk);

    AlifeObjectAnomalyZone {
      base,
      offline_interactive_radius,
      artefact_spawn_count,
      artefact_position_offset,
    }
  }
}
