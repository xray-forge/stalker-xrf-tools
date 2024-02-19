use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item::AlifeObjectItem;

pub struct AlifeObjectItemDetector {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInheritedReader<AlifeObjectItemDetector> for AlifeObjectItemDetector {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectItemDetector {
    let base: AlifeObjectItem = AlifeObjectItem::from_chunk(chunk);

    AlifeObjectItemDetector { base }
  }
}

impl AlifeObjectGeneric for AlifeObjectItemDetector {}
