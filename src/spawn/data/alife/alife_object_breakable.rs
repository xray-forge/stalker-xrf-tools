use crate::spawn::chunk::chunk::Chunk;
use crate::spawn::data::alife::alife_object_visual::AlifeObjectVisual;
use crate::spawn::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeObjectBreakable {
  pub base: AlifeObjectVisual,
  pub health: f32,
}

impl AlifeObjectBreakable {
  pub fn from_chunk(chunk: &mut Chunk) -> AlifeObjectBreakable {
    let base: AlifeObjectVisual = AlifeObjectVisual::from_chunk(chunk);
    let health: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();

    assert_eq!(
      chunk.read_bytes_remain(),
      0,
      "Expected all data to be read from chunk."
    );

    AlifeObjectBreakable { base, health }
  }
}
