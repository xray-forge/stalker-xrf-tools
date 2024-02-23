use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;

pub struct AlifeSmartZone {
  pub base: AlifeObjectSpaceRestrictor,
}

impl AlifeObjectInheritedReader<AlifeSmartZone> for AlifeSmartZone {
  fn read_from_chunk(chunk: &mut Chunk) -> AlifeSmartZone {
    let base: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor::read_from_chunk(chunk);

    AlifeSmartZone { base }
  }
}
