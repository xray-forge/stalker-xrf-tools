use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};
use xray_ltx::Ltx;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionMove {}

impl ParticleActionReader for ParticleActionMove {
  fn read<B: ByteOrder>(_: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {})
  }

  fn import(_: &str, _: &Ltx) -> DatabaseResult<Self> {
    Ok(Self {})
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionMove {
  fn write(&self, _: &mut ChunkWriter) -> DatabaseResult {
    Ok(())
  }

  fn export(&self, _: &str, _: &mut Ltx) -> DatabaseResult {
    Ok(())
  }
}
