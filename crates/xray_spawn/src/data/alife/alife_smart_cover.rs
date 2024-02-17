use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_smart_cover::AlifeObjectSmartCover;
use crate::data::alife_object::AlifeObjectInherited;
use byteorder::ReadBytesExt;

/// Represents script extension of base server smart cover class.
pub struct AlifeSmartCover {
  pub base: AlifeObjectSmartCover,
  pub last_description: String,
  pub loopholes: Vec<SmartCoverLoophole>,
}

impl AlifeObjectInherited<AlifeSmartCover> for AlifeSmartCover {
  fn from_chunk(chunk: &mut Chunk) -> AlifeSmartCover {
    let base: AlifeObjectSmartCover = AlifeObjectSmartCover::from_chunk(chunk);

    let last_description: String = chunk.read_null_terminated_string().unwrap();
    let count: u8 = chunk.read_u8().unwrap();
    let mut loopholes: Vec<SmartCoverLoophole> = Vec::new();

    for _ in 0..count {
      let name: String = chunk.read_null_terminated_string().unwrap();
      let enabled: u8 = chunk.read_u8().unwrap();

      loopholes.push(SmartCoverLoophole { name, enabled })
    }

    Self::verify(chunk);

    AlifeSmartCover {
      base,
      last_description,
      loopholes,
    }
  }
}

pub struct SmartCoverLoophole {
  pub name: String,
  pub enabled: u8,
}
