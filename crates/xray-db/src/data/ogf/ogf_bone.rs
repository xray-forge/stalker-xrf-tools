use crate::data::generic::vector_3d::Vector3d;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfBone {
  pub name: String,
  pub parent: String,
  pub rotation: (Vector3d, Vector3d, Vector3d),
  pub translate: Vector3d,
  pub half_size: Vector3d,
}

impl ChunkReadWrite for OgfBone {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      name: reader.read_w1251_string()?,
      parent: reader.read_w1251_string()?,
      rotation: (
        reader.read_xr::<T, _>()?,
        reader.read_xr::<T, _>()?,
        reader.read_xr::<T, _>()?,
      ),
      translate: reader.read_xr::<T, _>()?,
      half_size: reader.read_xr::<T, _>()?,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_w1251_string(&self.name)?;
    writer.write_w1251_string(&self.parent)?;

    writer.write_xr::<T, _>(&self.rotation.0)?;
    writer.write_xr::<T, _>(&self.rotation.1)?;
    writer.write_xr::<T, _>(&self.rotation.2)?;

    writer.write_xr::<T, _>(&self.translate)?;
    writer.write_xr::<T, _>(&self.half_size)?;

    Ok(())
  }
}
