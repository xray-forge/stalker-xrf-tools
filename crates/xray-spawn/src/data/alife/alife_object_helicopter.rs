use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_motion::AlifeObjectMotion;
use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
use crate::data::alife::alife_object_visual::AlifeObjectVisual;

pub struct AlifeObjectHelicopter {
  pub base: AlifeObjectVisual,
  pub skeleton: AlifeObjectSkeleton,
  pub motion: AlifeObjectMotion,
  pub startup_animation: String,
  pub engine_sound: String,
}

impl AlifeObjectInheritedReader<AlifeObjectHelicopter> for AlifeObjectHelicopter {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectHelicopter {
    let base: AlifeObjectVisual = AlifeObjectVisual::from_chunk(chunk);
    let motion: AlifeObjectMotion = AlifeObjectMotion::from_chunk(chunk);
    let skeleton: AlifeObjectSkeleton = AlifeObjectSkeleton::from_chunk(chunk);

    let startup_animation: String = chunk.read_null_terminated_string().unwrap();
    let engine_sound: String = chunk.read_null_terminated_string().unwrap();

    AlifeObjectHelicopter {
      base,
      skeleton,
      motion,
      startup_animation,
      engine_sound,
    }
  }
}

impl AlifeObjectGeneric for AlifeObjectHelicopter {}
