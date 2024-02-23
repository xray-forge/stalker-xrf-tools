use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_creature::AlifeObjectCreature;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
use crate::data::alife::alife_object_trader_abstract::AlifeObjectTraderAbstract;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeObjectActor {
  pub base: AlifeObjectCreature,
  pub trader: AlifeObjectTraderAbstract,
  pub skeleton: AlifeObjectSkeleton,
  pub holder_id: u16,
}

impl AlifeObjectInheritedReader<AlifeObjectActor> for AlifeObjectActor {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectActor> {
    let base: AlifeObjectCreature = AlifeObjectCreature::read_from_chunk::<T>(chunk)?;
    let trader: AlifeObjectTraderAbstract = AlifeObjectTraderAbstract::read_from_chunk::<T>(chunk)?;
    let skeleton: AlifeObjectSkeleton = AlifeObjectSkeleton::read_from_chunk::<T>(chunk)?;

    let holder_id: u16 = chunk.read_u16::<T>()?;

    Ok(AlifeObjectActor {
      base,
      trader,
      skeleton,
      holder_id,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}
