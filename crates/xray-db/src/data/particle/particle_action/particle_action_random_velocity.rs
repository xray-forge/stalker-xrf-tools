use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::particle::particle_action::particle_action_generic::ParticleActionGeneric;
use crate::data::particle::particle_domain::ParticleDomain;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_ltx::Ltx;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionRandomVelocity {
  pub gen_vel: ParticleDomain,
}

impl ParticleActionRandomVelocity {
  /// Read particle_action random velocity.
  pub fn read<T: ByteOrder>(
    reader: &mut ChunkReader,
  ) -> DatabaseResult<ParticleActionRandomVelocity> {
    Ok(ParticleActionRandomVelocity {
      gen_vel: ParticleDomain::read::<T>(reader)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionRandomVelocity {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    todo!()
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("gen_vel", self.gen_vel.to_string());

    Ok(())
  }
}
