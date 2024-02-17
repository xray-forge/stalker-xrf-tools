use crate::spawn::chunk::Chunk;
use crate::spawn::data::alife::alife_object_abstract::AlifeObjectAbstract;
use byteorder::ReadBytesExt;

pub struct AlifeObjectVisual {
  pub base: AlifeObjectAbstract,
  pub visual_name: String,
  pub visual_flags: u8,
}

impl AlifeObjectVisual {
  pub fn from_chunk(chunk: &mut Chunk) -> AlifeObjectVisual {
    let base: AlifeObjectAbstract = AlifeObjectAbstract::from_chunk(chunk);

    let visual_name: String = chunk.read_null_terminated_string().unwrap();
    let visual_flags: u8 = chunk.read_u8().unwrap();

    AlifeObjectVisual {
      base,
      visual_name,
      visual_flags,
    }
  }
}
