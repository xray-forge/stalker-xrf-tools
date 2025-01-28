use crate::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};

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

  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    log::info!(
      "Reading description chunk: {} bytes",
      reader.read_bytes_remain()
    );

    let description: Self = Self {
      source_file: reader.read_null_terminated_win_string()?,
      convertor: reader.read_null_terminated_win_string()?,
      built_at: reader.read_u32::<T>()?,
      creator: reader.read_null_terminated_win_string()?,
      created_at: reader.read_u32::<T>()?,
      editor: reader.read_null_terminated_win_string()?,
      edited_at: reader.read_u32::<T>()?,
    };

    assert!(
      reader.is_ended(),
      "Expect all data to be read from ogf description, {} remain",
      reader.read_bytes_remain()
    );

    Ok(description)
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_null_terminated_win_string(&self.source_file)?;
    writer.write_null_terminated_win_string(&self.convertor)?;
    writer.write_u32::<T>(self.built_at)?;
    writer.write_null_terminated_win_string(&self.creator)?;
    writer.write_u32::<T>(self.created_at)?;
    writer.write_null_terminated_win_string(&self.editor)?;
    writer.write_u32::<T>(self.edited_at)?;

    Ok(())
  }
}
