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
        Vector3d::read::<T>(reader)?,
        Vector3d::read::<T>(reader)?,
        Vector3d::read::<T>(reader)?,
      ),
      translate: Vector3d::read::<T>(reader)?,
      half_size: Vector3d::read::<T>(reader)?,
    })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_null_terminated_win_string(&self.name)?;
    writer.write_null_terminated_win_string(&self.parent)?;

    self.rotation.0.write::<T>(writer)?;
    self.rotation.1.write::<T>(writer)?;
    self.rotation.2.write::<T>(writer)?;

    self.translate.write::<T>(writer)?;
    self.half_size.write::<T>(writer)?;

    Ok(())
  }
}
