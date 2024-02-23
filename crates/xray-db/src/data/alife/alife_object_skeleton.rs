use crate::chunk::chunk::Chunk;
use crate::constants::FLAG_SKELETON_SAVED_DATA;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeObjectSkeleton {
  pub name: String,
  pub flags: u8,
  pub source_id: u16,
}

impl AlifeObjectInheritedReader<AlifeObjectSkeleton> for AlifeObjectSkeleton {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectSkeleton> {
    let name: String = chunk.read_null_terminated_string().unwrap();
    let flags: u8 = chunk.read_u8().unwrap();
    let source_id: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();

    if flags & FLAG_SKELETON_SAVED_DATA == 1 {
      todo!("Extend skeleton parsing to include bones.")
    }

    Ok(AlifeObjectSkeleton {
      name,
      flags,
      source_id,
    })
  }
}
