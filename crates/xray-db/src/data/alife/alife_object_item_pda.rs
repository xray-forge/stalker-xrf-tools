use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeObjectItemPda {
  pub base: AlifeObjectItem,
  pub owner: u16,
  pub character: String,
  pub info_portion: String,
}

impl AlifeObjectInheritedReader<AlifeObjectItemPda> for AlifeObjectItemPda {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectItemPda> {
    let base: AlifeObjectItem = AlifeObjectItem::read_from_chunk::<T>(chunk)?;

    let owner: u16 = chunk.read_u16::<SpawnByteOrder>()?;
    let character: String = chunk.read_null_terminated_string()?;
    let info_portion: String = chunk.read_null_terminated_string()?;

    Ok(AlifeObjectItemPda {
      base,
      owner,
      character,
      info_portion,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeObjectItemPda {}
