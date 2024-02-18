use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use crate::data::alife_object_base::{AlifeObjectGeneric, AlifeObjectInheritedReader};
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeObjectItem {
  pub base: AlifeObjectVisual,
  pub condition: f32,
}

impl AlifeObjectInheritedReader<AlifeObjectItem> for AlifeObjectItem {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectItem {
    let base: AlifeObjectVisual = AlifeObjectVisual::from_chunk(chunk);

    let condition: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let upgrades_count: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();

    assert_eq!(upgrades_count, 0, "Unexpected upgraded item provided.");

    AlifeObjectItem { base, condition }
  }
}

impl AlifeObjectGeneric for AlifeObjectItem {}
