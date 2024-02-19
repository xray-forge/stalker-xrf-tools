use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item::AlifeObjectItem;

pub struct AlifeObjectItemExplosive {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInheritedReader<AlifeObjectItemExplosive> for AlifeObjectItemExplosive {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectItemExplosive {
    let base: AlifeObjectItem = AlifeObjectItem::from_chunk(chunk);

    AlifeObjectItemExplosive { base }
  }
}

impl AlifeObjectGeneric for AlifeObjectItemExplosive {}
