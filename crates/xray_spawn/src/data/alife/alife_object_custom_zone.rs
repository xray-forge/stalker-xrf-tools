use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
use crate::data::alife_object_base::AlifeObjectInheritedReader;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeObjectCustomZone {
  pub base: AlifeObjectSpaceRestrictor,
  pub max_power: f32,
  pub owner_id: u32,
  pub enabled_time: u32,
  pub disabled_time: u32,
  pub m_start_time_shift: u32,
}

impl AlifeObjectInheritedReader<AlifeObjectCustomZone> for AlifeObjectCustomZone {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectCustomZone {
    let base: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor::from_chunk(chunk);

    let max_power: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let owner_id: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let enabled_time: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let disabled_time: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let m_start_time_shift: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();

    AlifeObjectCustomZone {
      base,
      max_power,
      owner_id,
      enabled_time,
      disabled_time,
      m_start_time_shift,
    }
  }
}
