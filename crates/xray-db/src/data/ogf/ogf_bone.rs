use crate::data::generic::vector_3d::Vector3d;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};
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

impl OgfBone {
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      name: reader.read_null_terminated_win_string()?,
      parent: reader.read_null_terminated_win_string()?,
      rotation: (
        reader.read_xr::<T, _>()?,
        reader.read_xr::<T, _>()?,
        reader.read_xr::<T, _>()?,
      ),
      translate: reader.read_xr::<T, _>()?,
      half_size: reader.read_xr::<T, _>()?,
    })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_null_terminated_win_string(&self.name)?;
    writer.write_null_terminated_win_string(&self.parent)?;

    writer.write_xr::<T, Vector3d>(&self.rotation.0)?;
    writer.write_xr::<T, Vector3d>(&self.rotation.1)?;
    writer.write_xr::<T, Vector3d>(&self.rotation.2)?;

    writer.write_xr::<T, Vector3d>(&self.translate)?;
    writer.write_xr::<T, Vector3d>(&self.half_size)?;

    Ok(())
  }
}
