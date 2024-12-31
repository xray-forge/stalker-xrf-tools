use crate::chunk::reader::ChunkReader;
use crate::data::particle::particle_action::particle_action_generic::ParticleActionGeneric;
use crate::data::particle::particle_domain::ParticleDomain;
use crate::data::vector_3d::Vector3d;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionSource {
  pub position: ParticleDomain,
  pub velocity: ParticleDomain,
  pub rot: ParticleDomain,
  pub size: ParticleDomain,
  pub color: ParticleDomain,
  pub alpha: f32,
  pub particle_rate: f32,
  pub age: f32,
  pub age_sigma: f32,
  pub parent_vel: Vector3d,
  pub parent_motion: f32,
}

impl ParticleActionSource {
  /// Read particle_action source.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionSource> {
    Ok(ParticleActionSource {
      position: ParticleDomain::read::<T>(reader)?,
      velocity: ParticleDomain::read::<T>(reader)?,
      rot: ParticleDomain::read::<T>(reader)?,
      size: ParticleDomain::read::<T>(reader)?,
      color: ParticleDomain::read::<T>(reader)?,
      alpha: reader.read_f32::<T>()?,
      particle_rate: reader.read_f32::<T>()?,
      age: reader.read_f32::<T>()?,
      age_sigma: reader.read_f32::<T>()?,
      parent_vel: reader.read_f32_3d_vector::<T>()?,
      parent_motion: reader.read_f32::<T>()?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionSource {}
