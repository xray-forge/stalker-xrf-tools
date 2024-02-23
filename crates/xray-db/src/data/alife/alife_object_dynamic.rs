use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use byteorder::ByteOrder;
use std::io;

pub struct AlifeObjectDynamic {
  pub base: AlifeObjectAbstract,
}

impl AlifeObjectInheritedReader<AlifeObjectDynamic> for AlifeObjectDynamic {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectDynamic> {
    let base: AlifeObjectAbstract = AlifeObjectAbstract::read_from_chunk::<T>(chunk)?;

    Ok(AlifeObjectDynamic { base })
  }
}
