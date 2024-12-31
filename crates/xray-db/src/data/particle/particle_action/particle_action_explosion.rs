use crate::chunk::reader::ChunkReader;
use crate::data::particle::particle_action::particle_action_generic::ParticleActionGeneric;
use crate::data::particle::particle_domain::ParticleDomain;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionExplosion {
  pub center: ParticleDomain,
  pub velocity: f32,
  pub magnitude: f32,
  pub st_dev: f32,
  pub age: f32,
  pub epsilon: f32,
}

impl ParticleActionExplosion {
  /// Read particle_action explosion.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionExplosion> {
    Ok(ParticleActionExplosion {
      center: ParticleDomain::read::<T>(reader)?,
      velocity: reader.read_f32::<T>()?,
      magnitude: reader.read_f32::<T>()?,
      st_dev: reader.read_f32::<T>()?,
      age: reader.read_f32::<T>()?,
      epsilon: reader.read_f32::<T>()?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionExplosion {}
