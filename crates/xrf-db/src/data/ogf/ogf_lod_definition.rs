use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xrf_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xrf_error::XRayResult;

use crate::data::ogf::ogf_lod_face::OgfLodFace;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfLodDefinition {
  pub lod_faces: [OgfLodFace; 8],
}

impl ChunkReadWrite for OgfLodDefinition {
  fn read<T: ByteOrder>(_: &mut ChunkReader) -> XRayResult<Self> {
    todo!("Implement")
  }

  fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> XRayResult {
    todo!("Implement")
  }
}
