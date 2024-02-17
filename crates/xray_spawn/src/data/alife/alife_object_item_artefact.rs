use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::data::alife_object::AlifeObjectInherited;

pub struct AlifeItemArtefact {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInherited<AlifeItemArtefact> for AlifeItemArtefact {
  fn from_chunk(chunk: &mut Chunk) -> AlifeItemArtefact {
    let base: AlifeObjectItem = AlifeObjectItem::from_chunk(chunk);

    AlifeItemArtefact { base }
  }
}
