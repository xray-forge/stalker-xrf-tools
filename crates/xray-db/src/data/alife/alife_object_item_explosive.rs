use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item::AlifeObjectItem;
use byteorder::ByteOrder;
use std::io;

pub struct AlifeObjectItemExplosive {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInheritedReader<AlifeObjectItemExplosive> for AlifeObjectItemExplosive {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectItemExplosive> {
    let base: AlifeObjectItem = AlifeObjectItem::read_from_chunk::<T>(chunk)?;

    Ok(AlifeObjectItemExplosive { base })
  }
}

impl AlifeObjectGeneric for AlifeObjectItemExplosive {}
