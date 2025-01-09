use crate::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfMotion {
  pub name: String,
  pub count: u32,
  pub flags: u8,
}

impl OgfMotion {
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let name: String = reader.read_null_terminated_win_string()?;
    let count: u32 = reader.read_u32::<T>()?;
    let flags: u8 = reader.read_u8()?;

    if reader.read_bytes_remain() > 0 {
      // todo: Read all data
      // todo: Check is ended

      log::warn!("Some data remains and was not read for motion {name} ({flags})")
    }

    Ok(Self { name, count, flags })
  }

  pub fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> DatabaseResult {
    todo!("Implement")
  }
}
