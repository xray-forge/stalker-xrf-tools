use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::generic::vector_3d::Vector3d;
use crate::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfSphere {
  pub position: Vector3d,
  pub radius: f32,
}

impl OgfSphere {
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      position: reader.read_f32_3d_vector::<T>()?,
      radius: reader.read_f32::<T>()?,
    })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_f32_3d_vector::<T>(&self.position)?;
    writer.write_f32::<T>(self.radius)?;

    Ok(())
  }
}
