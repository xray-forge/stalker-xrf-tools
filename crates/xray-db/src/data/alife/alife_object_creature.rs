use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeObjectCreature {
  pub base: AlifeObjectVisual,
  pub team: u8,
  pub squad: u8,
  pub group: u8,
  pub health: f32,
  pub dynamic_out_restrictions: Vec<u16>,
  pub dynamic_in_restrictions: Vec<u16>,
  pub killer_id: u16,
  pub game_death_time: u64,
}

impl AlifeObjectInheritedReader<AlifeObjectCreature> for AlifeObjectCreature {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectCreature> {
    let base: AlifeObjectVisual = AlifeObjectVisual::read_from_chunk::<T>(chunk)?;

    let team: u8 = chunk.read_u8().unwrap();
    let squad: u8 = chunk.read_u8().unwrap();
    let group: u8 = chunk.read_u8().unwrap();
    let health: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();

    let dynamic_out_restrictions: Vec<u16> = chunk.read_u16_vector::<SpawnByteOrder>().unwrap();
    let dynamic_in_restrictions: Vec<u16> = chunk.read_u16_vector::<SpawnByteOrder>().unwrap();

    let killer_id: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();
    let game_death_time: u64 = chunk.read_u64::<SpawnByteOrder>().unwrap();

    Ok(AlifeObjectCreature {
      base,
      team,
      squad,
      group,
      health,
      dynamic_out_restrictions,
      dynamic_in_restrictions,
      killer_id,
      game_death_time,
    })
  }
}
