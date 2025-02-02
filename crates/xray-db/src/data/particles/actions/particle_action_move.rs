use crate::export::LtxImportExport;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;
use xray_ltx::Ltx;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionMove {}

impl ChunkReadWrite for ParticleActionMove {
  fn read<T: ByteOrder>(_: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {})
  }

  fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> XRayResult {
    Ok(())
  }
}

impl LtxImportExport for ParticleActionMove {
  fn import(_: &str, _: &Ltx) -> XRayResult<Self> {
    Ok(Self {})
  }

  fn export(&self, _: &str, _: &mut Ltx) -> XRayResult {
    Ok(())
  }
}
