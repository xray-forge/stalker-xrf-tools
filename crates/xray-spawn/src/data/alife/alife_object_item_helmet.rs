use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item::AlifeObjectItem;

pub struct AlifeObjectItemHelmet {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInheritedReader<AlifeObjectItemHelmet> for AlifeObjectItemHelmet {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectItemHelmet {
    let base: AlifeObjectItem = AlifeObjectItem::from_chunk(chunk);

    AlifeObjectItemHelmet { base }
  }
}

impl AlifeObjectGeneric for AlifeObjectItemHelmet {}
