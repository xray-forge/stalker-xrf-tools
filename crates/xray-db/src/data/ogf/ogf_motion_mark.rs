use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;
use xray_utils::assert_length;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfMotionMark {
  pub name: String,
  pub intervals: Vec<(f32, f32)>,
}

impl ChunkReadWrite for OgfMotionMark {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let name: String = reader.read_w1251_rn_string()?;

    let count: u32 = reader.read_u32::<T>()?;
    let mut intervals: Vec<(f32, f32)> = Vec::with_capacity(count as usize);

    for _ in 0..count {
      intervals.push((reader.read_f32::<T>()?, reader.read_f32::<T>()?));
    }

    assert_length(
      &intervals,
      count as usize,
      "Expected correct count of OGF mark intervals to be read",
    )?;

    Ok(Self { name, intervals })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_w1251_rn_string(&self.name)?;
    writer.write_u32::<T>(self.intervals.len() as u32)?;

    for (from, to) in &self.intervals {
      writer.write_f32::<T>(*from)?;
      writer.write_f32::<T>(*to)?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_test_utils::FileSlice;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice, overwrite_test_relative_resource_as_file,
  };

  use crate::data::ogf::ogf_motion_mark::OgfMotionMark;

  #[test]
  fn test_read_write() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: OgfMotionMark = OgfMotionMark {
      name: String::from("Left"),
      intervals: vec![(0.25, 0.75), (1.5, 2.0)],
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    // 4 name bytes + 2 terminator bytes + 4 count bytes + 4 interval floats.
    assert_eq!(writer.bytes_written(), 4 + 2 + 4 + 16);

    writer.flush_chunk_into::<XRayByteOrder>(&mut overwrite_test_relative_resource_as_file(&filename)?, 0)?;

    let file: FileSlice = open_test_resource_as_slice(&filename)?;
    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(OgfMotionMark::read::<XRayByteOrder>(&mut reader)?, original);

    Ok(())
  }

  #[test]
  fn test_read_write_without_intervals() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write_without_intervals.chunk");

    let original: OgfMotionMark = OgfMotionMark {
      name: String::from("Right"),
      intervals: Vec::new(),
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    writer.flush_chunk_into::<XRayByteOrder>(&mut overwrite_test_relative_resource_as_file(&filename)?, 0)?;

    let file: FileSlice = open_test_resource_as_slice(&filename)?;
    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(OgfMotionMark::read::<XRayByteOrder>(&mut reader)?, original);

    Ok(())
  }
}
