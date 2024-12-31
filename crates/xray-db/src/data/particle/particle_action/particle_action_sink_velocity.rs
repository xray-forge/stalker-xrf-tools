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
pub struct ParticleActionSinkVelocity {
  pub kill_inside: u32,
  pub velocity: ParticleDomain,
}

impl ParticleActionSinkVelocity {
  /// Read particle_action sink velocity.
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
impl ParticleActionGeneric for ParticleActionSinkVelocity {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    todo!()
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("kill_inside", self.kill_inside.to_string())
      .set("velocity", self.velocity.to_string());

    Ok(())
  }
}
