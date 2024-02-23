use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;

pub struct AlifeObjectMotion {
  pub motion_name: String,
}

impl AlifeObjectInheritedReader<AlifeObjectMotion> for AlifeObjectMotion {
  fn read_from_chunk(chunk: &mut Chunk) -> AlifeObjectMotion {
    let motion_name: String = chunk.read_null_terminated_string().unwrap();

    AlifeObjectMotion { motion_name }
  }
}
