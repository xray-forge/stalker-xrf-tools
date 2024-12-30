use crate::chunk::reader::ChunkReader;
use crate::data::particle::effect_action::particle_action_generic::ParticleActionGeneric;
use crate::data::vector_3d::Vector3d;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionTargetSize {
  pub size: Vector3d,
  pub scale: Vector3d,
}

impl ParticleActionTargetSize {
  /// Read effect_action target size.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionTargetSize> {
    Ok(ParticleActionTargetSize {
      size: reader.read_f32_3d_vector::<T>()?,
      scale: reader.read_f32_3d_vector::<T>()?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionTargetSize {}