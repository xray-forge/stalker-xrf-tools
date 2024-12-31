use crate::chunk::reader::ChunkReader;
use crate::data::particle::particle_action::particle_action_generic::ParticleActionGeneric;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionSpeedLimit {
  pub min_speed: f32,
  pub max_speed: f32,
}

impl ParticleActionSpeedLimit {
  /// Read particle_action speed limit.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionSpeedLimit> {
    Ok(ParticleActionSpeedLimit {
      min_speed: reader.read_f32::<T>()?,
      max_speed: reader.read_f32::<T>()?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionSpeedLimit {}
