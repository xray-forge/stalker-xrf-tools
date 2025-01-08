use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::generic::vector_3d::Vector3d;
use crate::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfBox {
  pub min: Vector3d,
  pub max: Vector3d,
}

impl OgfBox {
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      min: reader.read_f32_3d_vector::<T>()?,
      max: reader.read_f32_3d_vector::<T>()?,
    })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_f32_3d_vector::<T>(&self.min)?;
    writer.write_f32_3d_vector::<T>(&self.max)?;

    Ok(())
  }
}
