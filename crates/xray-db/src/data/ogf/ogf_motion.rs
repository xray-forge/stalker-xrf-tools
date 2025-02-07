use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfMotion {
  pub name: String,
  pub count: u32,
  pub flags: u8,
  pub remaining: Vec<u8>,
}

impl ChunkReadWrite for OgfMotion {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let name: String = reader.read_w1251_string()?;
    let count: u32 = reader.read_u32::<T>()?;
    let flags: u8 = reader.read_u8()?;
    let remaining: Vec<u8> = reader.read_remaining()?;

    reader.assert_read("Chunk data should be read for OgfMotion")?;

    Ok(Self {
      name,
      count,
      flags,
      remaining,
    })
  }

  fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> XRayResult {
    todo!("Implement")
  }
}
