use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_motion::AlifeObjectMotion;
use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use byteorder::ByteOrder;
use std::io;

pub struct AlifeObjectHelicopter {
  pub base: AlifeObjectVisual,
  pub skeleton: AlifeObjectSkeleton,
  pub motion: AlifeObjectMotion,
  pub startup_animation: String,
  pub engine_sound: String,
}

impl AlifeObjectInheritedReader<AlifeObjectHelicopter> for AlifeObjectHelicopter {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectHelicopter> {
    let base: AlifeObjectVisual = AlifeObjectVisual::read_from_chunk::<T>(chunk)?;
    let motion: AlifeObjectMotion = AlifeObjectMotion::read_from_chunk::<T>(chunk)?;
    let skeleton: AlifeObjectSkeleton = AlifeObjectSkeleton::read_from_chunk::<T>(chunk)?;

    let startup_animation: String = chunk.read_null_terminated_string()?;
    let engine_sound: String = chunk.read_null_terminated_string()?;

    Ok(AlifeObjectHelicopter {
      base,
      skeleton,
      motion,
      startup_animation,
      engine_sound,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeObjectHelicopter {}
