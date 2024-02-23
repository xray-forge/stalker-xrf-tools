use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::shape::Shape;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeObjectSpaceRestrictor {
  pub base: AlifeObjectAbstract,
  pub shape: Vec<Shape>,
  pub restrictor_type: u8,
}

impl AlifeObjectInheritedReader<AlifeObjectSpaceRestrictor> for AlifeObjectSpaceRestrictor {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectSpaceRestrictor> {
    let base: AlifeObjectAbstract = AlifeObjectAbstract::read_from_chunk::<T>(chunk)?;

    let shape: Vec<Shape> = chunk.read_shape_description::<SpawnByteOrder>().unwrap();
    let restrictor_type: u8 = chunk.read_u8().unwrap();

    Ok(AlifeObjectSpaceRestrictor {
      base,
      shape,
      restrictor_type,
    })
  }
}

impl AlifeObjectGeneric for AlifeObjectSpaceRestrictor {}
