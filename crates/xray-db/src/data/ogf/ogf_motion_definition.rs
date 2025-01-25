use crate::data::ogf::ogf_motion_mark::OgfMotionMark;
use crate::{DatabaseError, DatabaseResult};
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{assert_chunk_vector_read, ChunkReader, ChunkWriter};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfMotionDefinition {
  pub name: String,
  pub flags: u32,
  pub bone_or_part: u16,
  pub motion: u16,
  pub speed: f32,
  pub power: f32,
  pub accrue: f32,
  pub falloff: f32,
  pub marks: Vec<OgfMotionMark>,
}

impl OgfMotionDefinition {
  pub fn read_list<T: ByteOrder>(
    reader: &mut ChunkReader,
    version: u16,
  ) -> DatabaseResult<Vec<Self>> {
    let count: u16 = reader.read_u16::<T>()?;
    let mut definitions: Vec<Self> = Vec::with_capacity(count as usize);

    for _ in 0..count {
      definitions.push(Self::read::<T>(reader, version).map_err(|error| {
        DatabaseError::new_read_error(format!("Failed to read ogf motion: {error}"))
      })?);
    }

    assert_chunk_vector_read(
      &definitions,
      count as usize,
      "Expected correct count of OGF motions to be read",
    )?;

    Ok(definitions)
  }

  pub fn read<T: ByteOrder>(reader: &mut ChunkReader, version: u16) -> DatabaseResult<Self> {
    let name: String = reader.read_null_terminated_win_string()?;
    let flags: u32 = reader.read_u32::<T>()?;
    let bone_or_part: u16 = reader.read_u16::<T>()?;
    let motion: u16 = reader.read_u16::<T>()?;
    let speed: f32 = reader.read_f32::<T>()?;
    let power: f32 = reader.read_f32::<T>()?;
    let accrue: f32 = reader.read_f32::<T>()?;
    let falloff: f32 = reader.read_f32::<T>()?;

    let marks: Vec<OgfMotionMark> = if version == 4 {
      let count: u32 = reader.read_u32::<T>()?;
      let mut marks: Vec<OgfMotionMark> = Vec::with_capacity(count as usize);

      for _ in 0..count {
        marks.push(OgfMotionMark::read::<T>(reader).map_err(|error| {
          DatabaseError::new_read_error(format!("Failed to read ogf motion mark: {error}"))
        })?);
      }

      assert_chunk_vector_read(
        &marks,
        count as usize,
        "Expected correct count of OGF motion marks to be read",
      )?;

      marks
    } else {
      Vec::new()
    };

    let motion: Self = Self {
      name,
      flags,
      bone_or_part,
      motion,
      speed,
      power,
      accrue,
      falloff,
      marks,
    };

    Ok(motion)
  }

  pub fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> DatabaseResult {
    todo!("Implement")
  }
}
