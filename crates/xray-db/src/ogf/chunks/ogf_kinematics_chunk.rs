use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct OgfKinematicsChunk {
  pub source_chunk_id: u32,
  pub motion_refs: Vec<String>,
}

impl OgfKinematicsChunk {
  pub const CHUNK_ID: u32 = 24;
  pub const CHUNK_ID_OLD: u32 = 19;
}

// todo: Conditional read + implement chunk RW.
// todo: Conditional read + implement chunk RW.
// todo: Conditional read + implement chunk RW.
impl OgfKinematicsChunk {
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader, chunk_id: u32) -> XRayResult<Self> {
    log::info!(
      "Reading motion refs chunk: {} bytes, chunk id {} ",
      reader.read_bytes_remain(),
      chunk_id
    );

    let mut motion_refs: Vec<String> = Vec::new();

    if chunk_id == OgfKinematicsChunk::CHUNK_ID {
      for _ in 0..reader.read_u32::<T>()? {
        motion_refs.push(reader.read_w1251_string()?)
      }
    } else {
      motion_refs.push(reader.read_w1251_string()?);
    }

    reader.assert_read("Expect all data to be read from ogf motion refs chunk")?;

    Ok(Self {
      source_chunk_id: chunk_id,
      motion_refs,
    })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    if self.source_chunk_id == OgfKinematicsChunk::CHUNK_ID {
      writer.write_u32::<T>(self.motion_refs.len() as u32)?;

      for motion_ref in &self.motion_refs {
        writer.write_w1251_string(motion_ref)?;
      }
    } else {
      if self.motion_refs.len() != 1 {
        return Err(XRayError::new_unexpected_error(
          "Motions ref chunk writing error, expected vector with 1 value",
        ));
      }

      writer.write_w1251_string(self.motion_refs.first().unwrap())?;
    }

    Ok(())
  }
}
