use crate::data::generic::vector_3d::Vector3d;
use crate::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfBox {
  pub min: Vector3d,
  pub max: Vector3d,
}

impl OgfBox {
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      min: Vector3d::read::<T>(reader)?,
      max: Vector3d::read::<T>(reader)?,
    })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    self.min.write::<T>(writer)?;
    self.max.write::<T>(writer)?;

    Ok(())
  }
}
