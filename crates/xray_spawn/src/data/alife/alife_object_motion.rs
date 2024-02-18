use crate::chunk::chunk::Chunk;
use crate::data::alife_object::AlifeObjectInherited;

pub struct AlifeObjectMotion {
  pub motion_name: String,
}

impl AlifeObjectInherited<AlifeObjectMotion> for AlifeObjectMotion {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectMotion {
    let motion_name: String = chunk.read_null_terminated_string().unwrap();

    AlifeObjectMotion { motion_name }
  }
}
