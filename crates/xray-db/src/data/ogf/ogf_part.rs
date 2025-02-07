use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReadWriteList, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_utils::assert_length;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfPart {
  pub name: String,
  pub bones: Vec<(String, u32)>, // name + index.
}

impl OgfPart {
  pub fn get_bones(&self) -> Vec<&str> {
    self
      .bones
      .iter()
      .map(|it| it.0.as_str())
      .collect::<Vec<_>>()
  }
}

impl ChunkReadWriteList for OgfPart {
  fn read_list<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Vec<Self>> {
    let count: u16 = reader.read_u16::<T>()?;
    let mut parts: Vec<Self> = Vec::with_capacity(count as usize);

    for _ in 0..count {
      parts.push(
        Self::read::<T>(reader).map_err(|error| {
          XRayError::new_read_error(format!("Failed to read ogf part: {error}"))
        })?,
      );
    }

    assert_length(
      &parts,
      count as usize,
      "Expected correct count of OGF parts to be read",
    )?;

    Ok(parts)
  }

  fn write_list<T: ByteOrder>(_: &mut ChunkWriter, _: &[Self]) -> XRayResult {
    todo!()
  }
}

impl ChunkReadWrite for OgfPart {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let name: String = reader.read_w1251_string()?;
    let count: u16 = reader.read_u16::<T>()?;

    let mut bones: Vec<(String, u32)> = Vec::with_capacity(count as usize);

    for _ in 0..count {
      bones.push((reader.read_w1251_string()?, reader.read_u32::<T>()?));
    }

    Ok(Self { name, bones })
  }

  fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> XRayResult {
    todo!("Implement")
  }
}
