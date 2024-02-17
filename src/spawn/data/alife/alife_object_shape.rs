use crate::spawn::chunk::Chunk;
use crate::spawn::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::spawn::types::SpawnByteOrder;

pub struct AlifeObjectShape {
  pub base: AlifeObjectAbstract,
  pub shapes: Vec<f32>,
}

impl AlifeObjectShape {
  pub fn from_chunk(chunk: &mut Chunk) -> AlifeObjectShape {
    let base: AlifeObjectAbstract = AlifeObjectAbstract::from_chunk(chunk);

    let shapes: Vec<f32> = chunk.read_shape_description::<SpawnByteOrder>().unwrap();

    AlifeObjectShape { base, shapes }
  }
}
