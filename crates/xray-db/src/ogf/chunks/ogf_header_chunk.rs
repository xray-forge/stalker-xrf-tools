use crate::data::ogf::ogf_box::OgfBox;
use crate::data::ogf::ogf_sphere::OgfSphere;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};

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
}

impl ChunkReadWrite for OgfHeaderChunk {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    log::info!("Reading header chunk: {} bytes", reader.read_bytes_remain());

    let version: u8 = reader.read_u8()?;

    if version != 4 {
      return Err(XRayError::new_not_implemented_error(format!(
        "Unexpected version '{}' of OGF file, only version 4 is supported",
        version
      )));
    }

    let header: Self = Self {
      version,
      model_type: reader.read_u8()?,
      shader_id: reader.read_u16::<T>()?,
      bounding_box: reader.read_xr::<T, _>()?,
      bounding_sphere: reader.read_xr::<T, _>()?,
    };

    reader.assert_read("Expect all data to be read from ogf header, {} remain")?;

    Ok(header)
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_u8(self.version)?;
    writer.write_u8(self.model_type)?;
    writer.write_u16::<T>(self.shader_id)?;
    writer.write_xr::<T, _>(&self.bounding_box)?;
    writer.write_xr::<T, _>(&self.bounding_sphere)?;

    Ok(())
  }
}
