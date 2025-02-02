use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{assert_chunk_read, ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;

/// ogf_desc c++ class
#[derive(Debug, Serialize, Deserialize)]
pub struct OgfDescriptionChunk {
  pub source_file: String,
  pub convertor: String,
  pub built_at: u32,
  pub creator: String,
  pub created_at: u32,
  pub editor: String,
  pub edited_at: u32,
}

impl OgfDescriptionChunk {
  pub const CHUNK_ID: u32 = 18;
}

impl ChunkReadWrite for OgfDescriptionChunk {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    log::info!(
      "Reading description chunk: {} bytes",
      reader.read_bytes_remain()
    );

    let description: Self = Self {
      source_file: reader.read_w1251_string()?,
      convertor: reader.read_w1251_string()?,
      built_at: reader.read_u32::<T>()?,
      creator: reader.read_w1251_string()?,
      created_at: reader.read_u32::<T>()?,
      editor: reader.read_w1251_string()?,
      edited_at: reader.read_u32::<T>()?,
    };

    assert_chunk_read(reader, "Expect all data to be read from ogf description")?;

    Ok(description)
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_w1251_string(&self.source_file)?;
    writer.write_w1251_string(&self.convertor)?;
    writer.write_u32::<T>(self.built_at)?;
    writer.write_w1251_string(&self.creator)?;
    writer.write_u32::<T>(self.created_at)?;
    writer.write_w1251_string(&self.editor)?;
    writer.write_u32::<T>(self.edited_at)?;

    Ok(())
  }
}
