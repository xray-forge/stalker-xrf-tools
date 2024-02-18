use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::data::alife_object::AlifeObjectInherited;

pub struct AlifeObjectItemDetector {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInherited<AlifeObjectItemDetector> for AlifeObjectItemDetector {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectItemDetector {
    let base: AlifeObjectItem = AlifeObjectItem::from_chunk(chunk);

    AlifeObjectItemDetector { base }
  }
}
