use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::ogf::ogf_lod_face::OgfLodFace;
use crate::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfLodDefinition {
  pub lod_faces: [OgfLodFace; 8],
}

impl OgfLodDefinition {
  pub fn read<T: ByteOrder>(_: &mut ChunkReader) -> DatabaseResult<Self> {
    todo!("Implement")
  }

  pub fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> DatabaseResult {
    todo!("Implement")
  }
}
