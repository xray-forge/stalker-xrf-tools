use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::particle::particle_action::particle_action_generic::ParticleActionGeneric;
use crate::data::particle::particle_domain::ParticleDomain;
use crate::data::vector_3d::Vector3d;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::Ltx;

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
impl ParticleActionGeneric for ParticleActionSource {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    todo!()
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("position", self.position.to_string())
      .set("velocity", self.velocity.to_string())
      .set("rot", self.rot.to_string())
      .set("size", self.size.to_string())
      .set("color", self.color.to_string())
      .set("alpha", self.alpha.to_string())
      .set("particle_rate", self.particle_rate.to_string())
      .set("age", self.age.to_string())
      .set("age_sigma", self.age_sigma.to_string())
      .set("parent_vel", self.parent_vel.to_string())
      .set("parent_motion", self.parent_motion.to_string());

    Ok(())
  }
}
