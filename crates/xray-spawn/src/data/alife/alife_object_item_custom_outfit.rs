use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item::AlifeObjectItem;

pub struct AlifeObjectItemCustomOutfit {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInheritedReader<AlifeObjectItemCustomOutfit> for AlifeObjectItemCustomOutfit {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectItemCustomOutfit {
    let base: AlifeObjectItem = AlifeObjectItem::from_chunk(chunk);

    AlifeObjectItemCustomOutfit { base }
  }
}

impl AlifeObjectGeneric for AlifeObjectItemCustomOutfit {}
