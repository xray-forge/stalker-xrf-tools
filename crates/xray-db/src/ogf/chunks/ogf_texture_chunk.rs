use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{assert_chunk_read, ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct OgfTextureChunk {
  pub texture_name: String,
  pub shader_name: String,
}

impl OgfTextureChunk {
  pub const CHUNK_ID: u32 = 2;
}

impl ChunkReadWrite for OgfTextureChunk {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let texture: Self = Self {
      texture_name: reader.read_w1251_string()?,
      shader_name: reader.read_w1251_string()?,
    };

    assert_chunk_read(reader, "Expect all data to be read from ogf texture")?;

    Ok(texture)
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_w1251_string(&self.texture_name)?;
    writer.write_w1251_string(&self.shader_name)?;

    Ok(())
  }
}
