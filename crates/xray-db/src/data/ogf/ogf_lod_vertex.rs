use crate::data::generic::vector_3d::Vector3d;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfLodVertex {
  pub v: Vector3d,
  pub t: (f32, f32),
  pub rgb_hemi: u32,
  pub sun: u8,
  pub pad: [u8; 12],
}

impl ChunkReadWrite for OgfLodVertex {
  fn read<T: ByteOrder>(_: &mut ChunkReader) -> XRayResult<Self> {
    todo!("Implement")
  }

  fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> XRayResult {
    todo!("Implement")
  }
}
