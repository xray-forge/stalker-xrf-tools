use crate::data::generic::vector_3d::Vector3d;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};
use xray_error::XRayResult;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfSphere {
  pub position: Vector3d,
  pub radius: f32,
}

impl OgfSphere {
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      position: Vector3d::read::<T>(reader)?,
      radius: reader.read_f32::<T>()?,
    })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    self.position.write::<T>(writer)?;

    writer.write_f32::<T>(self.radius)?;

    Ok(())
  }
}
