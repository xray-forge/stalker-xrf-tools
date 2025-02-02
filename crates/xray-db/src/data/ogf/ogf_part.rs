use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{assert_chunk_vector_read, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfPart {
  pub name: String,
  pub bones: Vec<(String, u32)>, // name + index.
}

impl OgfPart {
  pub fn read_list<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Vec<Self>> {
    let count: u16 = reader.read_u16::<T>()?;
    let mut parts: Vec<Self> = Vec::with_capacity(count as usize);

    for _ in 0..count {
      parts.push(
        Self::read::<T>(reader).map_err(|error| {
          XRayError::new_read_error(format!("Failed to read ogf part: {error}"))
        })?,
      );
    }

    assert_chunk_vector_read(
      &parts,
      count as usize,
      "Expected correct count of OGF parts to be read",
    )?;

    Ok(parts)
  }

  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let name: String = reader.read_w1251_string()?;
    let count: u16 = reader.read_u16::<T>()?;

    let mut bones: Vec<(String, u32)> = Vec::with_capacity(count as usize);

    for _ in 0..count {
      bones.push((reader.read_w1251_string()?, reader.read_u32::<T>()?));
    }

    Ok(Self { name, bones })
  }

  pub fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> XRayResult {
    todo!("Implement")
  }
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
