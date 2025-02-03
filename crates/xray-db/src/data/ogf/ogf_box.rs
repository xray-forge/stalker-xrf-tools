use crate::data::generic::vector_3d::Vector3d;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfBox {
  pub min: Vector3d,
  pub max: Vector3d,
}

impl ChunkReadWrite for OgfBox {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      min: reader.read_xr::<T, _>()?,
      max: reader.read_xr::<T, _>()?,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_xr::<T, _>(&self.min)?;
    writer.write_xr::<T, _>(&self.max)?;

    Ok(())
  }
}
