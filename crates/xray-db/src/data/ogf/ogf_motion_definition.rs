use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfMotionDefinition {
  pub bone_or_part: u16,
  pub motion: u16,
  pub speed: f32,
  pub power: f32,
  pub accrue: f32,
  pub falloff: f32,
}

impl OgfMotionDefinition {
  pub fn read<T: ByteOrder>(_: &mut ChunkReader) -> DatabaseResult<Self> {
    todo!("Implement")
  }

  pub fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> DatabaseResult {
    todo!("Implement")
  }
}
