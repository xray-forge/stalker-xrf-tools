use crate::chunk::reader::ChunkReader;
use crate::data::particle::effect_action::particle_action_generic::ParticleActionGeneric;
use crate::data::vector_3d::Vector3d;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionTargetRotate {
  pub rot: Vector3d,
  pub scale: f32,
}

impl ParticleActionTargetRotate {
  /// Read effect_action target rotate.
  pub fn read<T: ByteOrder>(
    reader: &mut ChunkReader,
  ) -> DatabaseResult<ParticleActionTargetRotate> {
    Ok(ParticleActionTargetRotate {
      rot: reader.read_f32_3d_vector::<T>()?,
      scale: reader.read_f32::<T>()?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionTargetRotate {}
