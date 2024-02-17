use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::alife_object::AlifeObjectInherited;
use crate::types::SpawnByteOrder;

pub struct AlifeObjectShape {
  pub base: AlifeObjectAbstract,
  pub shapes: Vec<f32>,
}

impl AlifeObjectInherited<AlifeObjectShape> for AlifeObjectShape {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectShape {
    let base: AlifeObjectAbstract = AlifeObjectAbstract::from_chunk(chunk);

    let shapes: Vec<f32> = chunk.read_shape_description::<SpawnByteOrder>().unwrap();

    AlifeObjectShape { base, shapes }
  }
}
