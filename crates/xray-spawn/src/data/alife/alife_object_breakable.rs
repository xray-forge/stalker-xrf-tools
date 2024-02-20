use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeObjectBreakable {
  pub base: AlifeObjectVisual,
  pub health: f32,
}

impl AlifeObjectInheritedReader<AlifeObjectBreakable> for AlifeObjectBreakable {
  fn read_from_chunk(chunk: &mut Chunk) -> AlifeObjectBreakable {
    let base: AlifeObjectVisual = AlifeObjectVisual::read_from_chunk(chunk);
    let health: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();

    AlifeObjectBreakable { base, health }
  }
}

impl AlifeObjectGeneric for AlifeObjectBreakable {}
