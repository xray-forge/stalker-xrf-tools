use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io::Read;
use xray_chunk::{assert_chunk_read, ChunkReader, ChunkWriter};
use xray_error::XRayResult;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfMotion {
  pub name: String,
  pub count: u32,
  pub flags: u8,
  pub remaining: Vec<u8>,
}

impl OgfMotion {
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let name: String = reader.read_null_terminated_win_string()?;
    let count: u32 = reader.read_u32::<T>()?;
    let flags: u8 = reader.read_u8()?;
    let mut remaining: Vec<u8> = Vec::new();

    reader.read_to_end(&mut remaining)?;

    assert_chunk_read(reader, "Chunk data should be read for OgfMotion")?;

    Ok(Self {
      name,
      count,
      flags,
      remaining,
    })
  }

  pub fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> XRayResult {
    todo!("Implement")
  }
}
