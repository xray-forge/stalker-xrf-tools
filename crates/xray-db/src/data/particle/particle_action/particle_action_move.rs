use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::particle::particle_action::particle_action_generic::ParticleActionGeneric;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionMove {}

impl ParticleActionMove {
  /// Read particle action move.
  pub fn read<T: ByteOrder>(_: &mut ChunkReader) -> DatabaseResult<ParticleActionMove> {
    // No data.
    Ok(ParticleActionMove {})
  }

  /// Import particle action move.
  pub fn import(_: &Section) -> DatabaseResult<ParticleActionMove> {
    // No data.
    Ok(ParticleActionMove {})
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionMove {
  fn write(&self, _: &mut ChunkWriter) -> DatabaseResult<()> {
    // No data.

    Ok(())
  }

  fn export(&self, _: &str, _: &mut Ltx) -> DatabaseResult<()> {
    // No data.

    Ok(())
  }
}
