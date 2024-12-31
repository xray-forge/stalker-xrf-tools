use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::particle::particle_action::particle_action_generic::ParticleActionGeneric;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::Ltx;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionKillOld {
  pub age_limit: f32,
  pub kill_less_than: u32,
}

impl ParticleActionKillOld {
  /// Read particle_action kill old.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionKillOld> {
    Ok(ParticleActionKillOld {
      age_limit: reader.read_f32::<T>()?,
      kill_less_than: reader.read_u32::<T>()?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionKillOld {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    todo!()
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("age_limit", self.age_limit.to_string())
      .set("kill_less_than", self.kill_less_than.to_string());

    Ok(())
  }
}
