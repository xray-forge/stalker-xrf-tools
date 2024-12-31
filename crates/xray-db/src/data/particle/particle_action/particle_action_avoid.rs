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
pub struct ParticleActionAvoid {
  pub position: ParticleDomain,
  pub look_ahead: f32,
  pub magnitude: f32,
  pub epsilon: f32,
}

impl ParticleActionAvoid {
  /// Read particle_action avoid.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionAvoid> {
    Ok(ParticleActionAvoid {
      position: ParticleDomain::read::<T>(reader)?,
      look_ahead: reader.read_f32::<T>()?,
      magnitude: reader.read_f32::<T>()?,
      epsilon: reader.read_f32::<T>()?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionAvoid {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    todo!()
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("position", self.position.to_string())
      .set("look_ahead", self.look_ahead.to_string())
      .set("magnitude", self.magnitude.to_string())
      .set("epsilon", self.epsilon.to_string());

    Ok(())
  }
}
