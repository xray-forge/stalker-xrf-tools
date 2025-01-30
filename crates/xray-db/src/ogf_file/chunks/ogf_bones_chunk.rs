use crate::data::ogf::ogf_bone::OgfBone;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{assert_chunk_read, assert_chunk_vector_read, ChunkReader, ChunkWriter};
use xray_error::XRayResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct OgfBonesChunk {
  pub bones: Vec<OgfBone>,
}

impl OgfBonesChunk {
  pub const CHUNK_ID: u32 = 13;

  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    log::info!("Reading bones chunk: {} bytes", reader.read_bytes_remain());

    let count: u32 = reader.read_u32::<T>()?;
    let mut bones: Vec<OgfBone> = Vec::with_capacity(count as usize);

    for _ in 0..count {
      bones.push(OgfBone::read::<T>(reader)?);
    }

    assert_chunk_read(reader, "Expect all data to be read from ogf bones chunk")?;
    assert_chunk_vector_read(
      &bones,
      count as usize,
      "Expected correct count of bones to be read",
    )?;

    Ok(Self { bones })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_u32::<T>(self.bones.len() as u32)?;

    for bone in &self.bones {
      bone.write::<T>(writer)?;
    }

    Ok(())
  }
}

impl OgfBonesChunk {
  pub fn get_bone_names(&self) -> Vec<&str> {
    self
      .bones
      .iter()
      .map(|it| it.name.as_str())
      .collect::<Vec<_>>()
  }
}
