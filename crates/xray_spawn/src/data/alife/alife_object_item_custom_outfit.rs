use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::data::alife_object::AlifeObjectInherited;

pub struct AlifeObjectItemCustomOutfit {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInherited<AlifeObjectItemCustomOutfit> for AlifeObjectItemCustomOutfit {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectItemCustomOutfit {
    let base: AlifeObjectItem = AlifeObjectItem::from_chunk(chunk);

    AlifeObjectItemCustomOutfit { base }
  }
}
