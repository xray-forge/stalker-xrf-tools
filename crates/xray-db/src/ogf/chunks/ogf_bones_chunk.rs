use crate::data::ogf::ogf_bone::OgfBone;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;
use xray_utils::assert_length;

#[derive(Debug, Serialize, Deserialize)]
pub struct OgfBonesChunk {
  pub bones: Vec<OgfBone>,
}

impl OgfBonesChunk {
  pub const CHUNK_ID: u32 = 13;

  pub fn get_bone_names(&self) -> Vec<&str> {
    self
      .bones
      .iter()
      .map(|it| it.name.as_str())
      .collect::<Vec<_>>()
  }
}

impl ChunkReadWrite for OgfBonesChunk {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    log::info!("Reading bones chunk: {} bytes", reader.read_bytes_remain());

    let count: u32 = reader.read_u32::<T>()?;
    let mut bones: Vec<OgfBone> = Vec::with_capacity(count as usize);

    for _ in 0..count {
      bones.push(reader.read_xr::<T, _>()?);
    }

    reader.assert_read("Expect all data to be read from ogf bones chunk")?;
    assert_length(
      &bones,
      count as usize,
      "Expected correct count of bones to be read",
    )?;

    Ok(Self { bones })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_u32::<T>(self.bones.len() as u32)?;

    for bone in &self.bones {
      writer.write_xr::<T, _>(bone)?
    }

    Ok(())
  }
}
