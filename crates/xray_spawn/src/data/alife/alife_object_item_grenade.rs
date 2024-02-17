use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::data::alife_object::AlifeObjectInherited;

pub struct AlifeObjectItemGrenade {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInherited<AlifeObjectItemGrenade> for AlifeObjectItemGrenade {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectItemGrenade {
    let base: AlifeObjectItem = AlifeObjectItem::from_chunk(chunk);

    AlifeObjectItemGrenade { base }
  }
}
