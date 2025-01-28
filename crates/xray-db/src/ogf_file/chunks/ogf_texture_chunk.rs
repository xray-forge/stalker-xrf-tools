use crate::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};

#[derive(Debug, Serialize, Deserialize)]
pub struct OgfTextureChunk {
  pub texture_name: String,
  pub shader_name: String,
}

impl OgfTextureChunk {
  pub const CHUNK_ID: u32 = 2;

  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    log::info!(
      "Reading texture chunk: {} bytes",
      reader.read_bytes_remain()
    );

    let texture: Self = Self {
      texture_name: reader.read_null_terminated_win_string()?,
      shader_name: reader.read_null_terminated_win_string()?,
    };

    assert!(
      reader.is_ended(),
      "Expect all data to be read from ogf texture, {} remain",
      reader.read_bytes_remain()
    );

    Ok(texture)
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_null_terminated_win_string(&self.texture_name)?;
    writer.write_null_terminated_win_string(&self.shader_name)?;

    Ok(())
  }
}
