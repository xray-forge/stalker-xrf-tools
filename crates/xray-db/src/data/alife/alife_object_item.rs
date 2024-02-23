use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeObjectItem {
  pub base: AlifeObjectVisual,
  pub condition: f32,
}

impl AlifeObjectInheritedReader<AlifeObjectItem> for AlifeObjectItem {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectItem> {
    let base: AlifeObjectVisual = AlifeObjectVisual::read_from_chunk::<T>(chunk)?;

    let condition: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let upgrades_count: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();

    assert_eq!(upgrades_count, 0, "Unexpected upgraded item provided.");

    Ok(AlifeObjectItem { base, condition })
  }
}

impl AlifeObjectGeneric for AlifeObjectItem {}
