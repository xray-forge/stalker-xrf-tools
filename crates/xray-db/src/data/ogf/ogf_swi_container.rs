use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;

/// Link to a sliding window table stored outside the visual, `OGF_SWICONTAINER` (chunk 20).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfSwiContainer {
  pub ext_swib_index: u32,
}

impl ChunkReadWrite for OgfSwiContainer {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let container: Self = Self {
      ext_swib_index: reader.read_u32::<T>()?,
    };

    reader.assert_read("Expect all data to be read from ogf swi container")?;

    Ok(container)
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_u32::<T>(self.ext_swib_index)?;

    Ok(())
  }
}
