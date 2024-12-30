use crate::chunk::reader::ChunkReader;
use crate::data::particle::effect_action::particle_action_generic::ParticleActionGeneric;
use crate::data::particle::particle_domain::ParticleDomain;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionSinkVelocity {
  pub kill_inside: u32,
  pub velocity: ParticleDomain,
}

impl ParticleActionSinkVelocity {
  /// Read effect_action sink velocity.
  pub fn read<T: ByteOrder>(
    reader: &mut ChunkReader,
  ) -> DatabaseResult<ParticleActionSinkVelocity> {
    Ok(ParticleActionSinkVelocity {
      kill_inside: reader.read_u32::<T>()?,
      velocity: ParticleDomain::read::<T>(reader)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionSinkVelocity {}
