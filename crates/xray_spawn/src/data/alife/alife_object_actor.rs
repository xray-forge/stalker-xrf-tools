use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_creature::AlifeObjectCreature;
use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
use crate::data::alife::alife_object_trader_abstract::AlifeObjectTraderAbstract;
use crate::data::alife_object_base::AlifeObjectInheritedReader;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeObjectActor {
  pub base: AlifeObjectCreature,
  pub trader: AlifeObjectTraderAbstract,
  pub skeleton: AlifeObjectSkeleton,
  pub holder_id: u16,
}

impl AlifeObjectInheritedReader<AlifeObjectActor> for AlifeObjectActor {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectActor {
    let base: AlifeObjectCreature = AlifeObjectCreature::from_chunk(chunk);
    let trader: AlifeObjectTraderAbstract = AlifeObjectTraderAbstract::from_chunk(chunk);
    let skeleton: AlifeObjectSkeleton = AlifeObjectSkeleton::from_chunk(chunk);

    let holder_id: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();

    AlifeObjectActor {
      base,
      trader,
      skeleton,
      holder_id,
    }
  }
}
