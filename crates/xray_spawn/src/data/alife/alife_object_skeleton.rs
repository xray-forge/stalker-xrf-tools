use crate::chunk::chunk::Chunk;
use crate::constants::FLAG_SKELETON_SAVED_DATA;
use crate::data::alife_object::AlifeObjectInherited;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeObjectSkeleton {
  pub name: String,
  pub flags: u8,
  pub source_id: u16,
}

impl AlifeObjectInherited<AlifeObjectSkeleton> for AlifeObjectSkeleton {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectSkeleton {
    let name: String = chunk.read_null_terminated_string().unwrap();
    let flags: u8 = chunk.read_u8().unwrap();
    let source_id: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();

    if flags & FLAG_SKELETON_SAVED_DATA == 1 {
      todo!("Extend skeleton parsing to include bones.")
    }

    AlifeObjectSkeleton {
      name,
      flags,
      source_id,
    }
  }
}
