use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;

pub struct AlifeObjectDynamic {
  pub base: AlifeObjectAbstract,
}

impl AlifeObjectInheritedReader<AlifeObjectDynamic> for AlifeObjectDynamic {
  fn read_from_chunk(chunk: &mut Chunk) -> AlifeObjectDynamic {
    let base: AlifeObjectAbstract = AlifeObjectAbstract::read_from_chunk(chunk);

    AlifeObjectDynamic { base }
  }
}
