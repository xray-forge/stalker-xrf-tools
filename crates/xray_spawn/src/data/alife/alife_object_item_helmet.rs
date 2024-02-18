use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::data::alife_object::AlifeObjectInherited;

pub struct AlifeObjectItemHelmet {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInherited<AlifeObjectItemHelmet> for AlifeObjectItemHelmet {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectItemHelmet {
    let base: AlifeObjectItem = AlifeObjectItem::from_chunk(chunk);

    AlifeObjectItemHelmet { base }
  }
}
