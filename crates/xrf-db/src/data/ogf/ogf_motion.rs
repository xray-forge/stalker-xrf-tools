use std::io::Write;

use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xrf_chunk::{ChunkDataSource, ChunkReadWrite, ChunkReader, ChunkWriter};
use xrf_error::XrfResult;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfMotion {
  pub name: String,
  pub count: u32,
  pub flags: u8,
  pub remaining: Vec<u8>,
}

impl ChunkReadWrite for OgfMotion {
  fn read<T: ByteOrder, D: ChunkDataSource>(reader: &mut ChunkReader<D>) -> XrfResult<Self> {
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

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XrfResult {
    writer.write_w1251_string(&self.name)?;
    writer.write_u32::<T>(self.count)?;
    writer.write_u8(self.flags)?;
    writer.write_all(&self.remaining)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use xrf_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter, XRayByteOrder};
  use xrf_error::XrfResult;
  use xrf_test_utils::FileSlice;
  use xrf_test_utils::utils::{
    get_relative_test_sample_file_path, open_generated_test_resource_as_slice,
    overwrite_generated_test_resource_as_file,
  };

  use crate::data::ogf::ogf_motion::OgfMotion;

  #[test]
  fn test_read_write() -> XrfResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: OgfMotion = OgfMotion {
      name: String::from("ak74_draw"),
      count: 32,
      flags: 1,
      remaining: vec![1, 2, 3, 4, 5, 6, 7, 8],
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    // 9 name bytes + 1 terminator + 4 count bytes + 1 flags byte + 8 payload bytes.
    assert_eq!(writer.bytes_written(), 9 + 1 + 4 + 1 + 8);

    writer.flush_chunk_into::<XRayByteOrder>(&mut overwrite_generated_test_resource_as_file(&filename)?, 0)?;

    let file: FileSlice = open_generated_test_resource_as_slice(&filename)?;
    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(OgfMotion::read::<XRayByteOrder, _>(&mut reader)?, original);

    Ok(())
  }
}
