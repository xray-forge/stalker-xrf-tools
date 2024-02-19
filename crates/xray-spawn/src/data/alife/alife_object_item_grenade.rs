use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item::AlifeObjectItem;

pub struct AlifeObjectItemGrenade {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInheritedReader<AlifeObjectItemGrenade> for AlifeObjectItemGrenade {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectItemGrenade {
    let base: AlifeObjectItem = AlifeObjectItem::from_chunk(chunk);

    AlifeObjectItemGrenade { base }
  }
}

impl AlifeObjectGeneric for AlifeObjectItemGrenade {}
