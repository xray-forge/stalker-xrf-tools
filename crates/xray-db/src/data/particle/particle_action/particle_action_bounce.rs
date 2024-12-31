use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::particle::particle_action::particle_action_generic::ParticleActionGeneric;
use crate::data::particle::particle_domain::ParticleDomain;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::Ltx;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionBounce {
  pub position: ParticleDomain,
  pub one_minus_friction: f32,
  pub resilience: f32,
  pub cutoff_sqr: f32,
}

impl ParticleActionBounce {
  /// Read particle_action bounce.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionBounce> {
    Ok(ParticleActionBounce {
      position: ParticleDomain::read::<T>(reader)?,
      one_minus_friction: reader.read_f32::<T>()?,
      resilience: reader.read_f32::<T>()?,
      cutoff_sqr: reader.read_f32::<T>()?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionBounce {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    todo!()
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("position", self.position.to_string())
      .set("one_minus_friction", self.one_minus_friction.to_string())
      .set("resilience", self.resilience.to_string())
      .set("cutoff_sqr", self.cutoff_sqr.to_string());

    Ok(())
  }
}
