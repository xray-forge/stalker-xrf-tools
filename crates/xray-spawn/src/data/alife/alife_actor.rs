use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_actor::AlifeObjectActor;
use crate::data::alife_object_base::AlifeObjectGeneric;
use crate::data::alife_object_base::AlifeObjectInheritedReader;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeActor {
  pub base: AlifeObjectActor,
  pub start_position_filled: u8,
}

impl AlifeObjectInheritedReader<AlifeActor> for AlifeActor {
  fn from_chunk(chunk: &mut Chunk) -> AlifeActor {
    let base: AlifeObjectActor = AlifeObjectActor::from_chunk(chunk);

    let start_position_filled: u8 = chunk.read_u8().unwrap();
    let save_marker: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();

    assert_eq!(
      save_marker, 1,
      "Unexpected save data for actor object provided."
    );

    AlifeActor {
      base,
      start_position_filled,
    }
  }
}

impl AlifeObjectGeneric for AlifeActor {}
