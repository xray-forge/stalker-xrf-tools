use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item::AlifeObjectItem;

pub struct AlifeItemArtefact {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInheritedReader<AlifeItemArtefact> for AlifeItemArtefact {
  fn read_from_chunk(chunk: &mut Chunk) -> AlifeItemArtefact {
    let base: AlifeObjectItem = AlifeObjectItem::read_from_chunk(chunk);

    AlifeItemArtefact { base }
  }
}

impl AlifeObjectGeneric for AlifeItemArtefact {}
