use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::particle::particle_action::particle_action_generic::ParticleActionGeneric;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::Ltx;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionFollow {
  pub magnitude: f32,
  pub epsilon: f32,
  pub max_radius: f32,
}

impl ParticleActionFollow {
  /// Read particle_action follow.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionFollow> {
    Ok(ParticleActionFollow {
      magnitude: reader.read_f32::<T>()?,
      epsilon: reader.read_f32::<T>()?,
      max_radius: reader.read_f32::<T>()?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionFollow {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    todo!()
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("magnitude", self.magnitude.to_string())
      .set("epsilon", self.epsilon.to_string())
      .set("max_radius", self.max_radius.to_string());

    Ok(())
  }
}
