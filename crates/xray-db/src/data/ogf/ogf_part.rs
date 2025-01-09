use crate::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfPart {
  pub name: String,
  pub bones: Vec<(String, u32)>, // name + index.
}

impl OgfPart {
  pub fn read_list<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Vec<Self>> {
    let mut parts: Vec<Self> = Vec::new();

    for _ in 0..reader.read_u16::<T>()? {
      parts.push(Self::read::<T>(reader)?);
    }

    Ok(parts)
  }

  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let name: String = reader.read_null_terminated_win_string()?;
    let mut bones: Vec<(String, u32)> = Vec::new();

    for _ in 0..reader.read_u16::<T>()? {
      bones.push((
        reader.read_null_terminated_win_string()?,
        reader.read_u32::<T>()?,
      ));
    }

    Ok(Self { name, bones })
  }

  pub fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> DatabaseResult {
    todo!("Implement")
  }
}
