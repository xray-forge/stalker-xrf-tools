use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use crate::data::alife_object_base::{AlifeObjectGeneric, AlifeObjectInheritedReader};
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeObjectBreakable {
  pub base: AlifeObjectVisual,
  pub health: f32,
}

impl AlifeObjectInheritedReader<AlifeObjectBreakable> for AlifeObjectBreakable {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectBreakable {
    let base: AlifeObjectVisual = AlifeObjectVisual::from_chunk(chunk);
    let health: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();

    AlifeObjectBreakable { base, health }
  }
}

impl AlifeObjectGeneric for AlifeObjectBreakable {}
