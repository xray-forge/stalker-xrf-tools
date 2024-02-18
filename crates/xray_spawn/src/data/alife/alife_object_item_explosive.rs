use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::data::alife_object::AlifeObjectInherited;

pub struct AlifeObjectItemExplosive {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInherited<AlifeObjectItemExplosive> for AlifeObjectItemExplosive {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectItemExplosive {
    let base: AlifeObjectItem = AlifeObjectItem::from_chunk(chunk);

    AlifeObjectItemExplosive { base }
  }
}
