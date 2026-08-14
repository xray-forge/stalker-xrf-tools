use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xrf_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xrf_error::XrfResult;

use crate::data::ogf::ogf_color::OgfColor;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfTreeDefinition {
  pub tree_xform: [f32; 16],
  pub scale: OgfColor,
  pub bias: OgfColor,
}

impl ChunkReadWrite for OgfTreeDefinition {
  fn read<T: ByteOrder>(_: &mut ChunkReader) -> XrfResult<Self> {
    todo!("Implement")
  }

  fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> XrfResult {
    todo!("Implement")
  }
}
