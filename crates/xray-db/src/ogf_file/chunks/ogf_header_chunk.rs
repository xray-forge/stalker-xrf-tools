use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::ogf::ogf_box::OgfBox;
use crate::data::ogf::ogf_sphere::OgfSphere;
use crate::{DatabaseNotImplementedError, DatabaseResult};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OgfHeaderChunk {
  pub version: u8,
  pub model_type: u8,
  pub shader_id: u16,
  pub bounding_box: OgfBox,
  pub bounding_sphere: OgfSphere,
}

impl OgfHeaderChunk {
  pub const CHUNK_ID: u32 = 1;

  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let version: u8 = reader.read_u8()?;

    if version != 4 {
      return Err(DatabaseNotImplementedError::new_database_error(format!(
        "Unexpected version '{version}' of OGF file, only version 4 is supported"
      )));
    }

    let header: Self = Self {
      version,
      model_type: reader.read_u8()?,
      shader_id: reader.read_u16::<T>()?,
      bounding_box: OgfBox::read::<T>(reader)?,
      bounding_sphere: OgfSphere::read::<T>(reader)?,
    };

    assert!(
      reader.is_ended(),
      "Expect all data to be read from ogf header, {} remain",
      reader.read_bytes_remain()
    );

    Ok(header)
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_u8(self.version)?;
    writer.write_u8(self.model_type)?;
    writer.write_u16::<T>(self.shader_id)?;

    self.bounding_box.write::<T>(writer)?;
    self.bounding_sphere.write::<T>(writer)?;

    Ok(())
  }
}
