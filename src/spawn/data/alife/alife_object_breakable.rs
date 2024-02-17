use crate::spawn::chunk::Chunk;
use crate::spawn::data::alife::alife_object_visual::AlifeObjectVisual;
use byteorder::{LittleEndian, ReadBytesExt};

pub struct AlifeObjectBreakable {
  pub base: AlifeObjectVisual,
  pub health: f32,
}

impl AlifeObjectBreakable {
  pub fn from_chunk(chunk: &mut Chunk) -> AlifeObjectBreakable {
    let base: AlifeObjectVisual = AlifeObjectVisual::from_chunk(chunk);
    let health: f32 = chunk.read_f32::<LittleEndian>().unwrap();

    assert_eq!(
      chunk.read_bytes_remain(),
      0,
      "Expected all data to be read from chunk."
    );

    AlifeObjectBreakable { base, health }
  }
}
