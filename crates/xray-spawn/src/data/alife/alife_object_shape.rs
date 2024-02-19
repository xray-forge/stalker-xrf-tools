use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::shape::Shape;
use crate::types::SpawnByteOrder;

pub struct AlifeObjectShape {
  pub base: AlifeObjectAbstract,
  pub shape: Vec<Shape>,
}

impl AlifeObjectInheritedReader<AlifeObjectShape> for AlifeObjectShape {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectShape {
    let base: AlifeObjectAbstract = AlifeObjectAbstract::from_chunk(chunk);

    let shape: Vec<Shape> = chunk.read_shape_description::<SpawnByteOrder>().unwrap();

    AlifeObjectShape { base, shape }
  }
}
