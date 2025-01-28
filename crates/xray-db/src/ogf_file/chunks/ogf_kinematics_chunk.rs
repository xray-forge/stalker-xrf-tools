use crate::{DatabaseError, DatabaseResult};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{assert_chunk_read, ChunkReader, ChunkWriter};

#[derive(Debug, Serialize, Deserialize)]
pub struct OgfKinematicsChunk {
  pub source_chunk_id: u32,
  pub motion_refs: Vec<String>,
}

impl OgfKinematicsChunk {
  pub const CHUNK_ID: u32 = 24;
  pub const CHUNK_ID_OLD: u32 = 19;

  pub fn read<T: ByteOrder>(reader: &mut ChunkReader, chunk_id: u32) -> DatabaseResult<Self> {
    log::info!(
      "Reading motion refs chunk: {} bytes, chunk id {} ",
      reader.read_bytes_remain(),
      chunk_id
    );

    let mut motion_refs: Vec<String> = Vec::new();

    if chunk_id == OgfKinematicsChunk::CHUNK_ID {
      for _ in 0..reader.read_u32::<T>()? {
        motion_refs.push(reader.read_null_terminated_win_string()?)
      }
    } else {
      motion_refs.push(reader.read_null_terminated_win_string()?);
    }

    assert_chunk_read(
      reader,
      "Expect all data to be read from ogf motion refs chunk",
    )?;

    Ok(Self {
      source_chunk_id: chunk_id,
      motion_refs,
    })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    if self.source_chunk_id == OgfKinematicsChunk::CHUNK_ID {
      writer.write_u32::<T>(self.motion_refs.len() as u32)?;

      for motion_ref in &self.motion_refs {
        writer.write_null_terminated_win_string(motion_ref)?;
      }
    } else {
      if self.motion_refs.len() != 1 {
        return Err(DatabaseError::new_not_expected_error(
          "Motions ref chunk writing error, expected vector with 1 value",
        ));
      }

      writer.write_null_terminated_win_string(self.motion_refs.first().unwrap())?;
    }

    Ok(())
  }
}
