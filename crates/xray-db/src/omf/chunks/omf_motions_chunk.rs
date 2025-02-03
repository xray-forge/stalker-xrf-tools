use crate::data::ogf::ogf_motion::OgfMotion;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{read_u32_chunk, ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct OmfMotionsChunk {
  pub motions: Vec<OgfMotion>,
}

impl OmfMotionsChunk {
  pub const CHUNK_ID: u32 = 14; // 0x1A, 0xE
}

impl ChunkReadWrite for OmfMotionsChunk {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    log::info!(
      "Reading motions chunk: {} bytes",
      reader.read_bytes_remain()
    );

    let mut chunks: Vec<ChunkReader> = reader.read_children();

    let bones_motions_count: u32 = read_u32_chunk::<T>(
      chunks
        .first_mut()
        .expect("Correct omf motions chunk with count definitions"),
    )?;

    assert_eq!(
      bones_motions_count as usize,
      chunks.len() - 1,
      "Expect matching OMF motions chunks count and count definition"
    );

    let mut motions: Vec<OgfMotion> = Vec::new();

    for chunk in &mut chunks[1..] {
      motions.push(chunk.read_xr::<T, _>()?);
    }

    assert!(
      reader.is_ended(),
      "Expect omf motions chunk to be ended, {} remain",
      reader.read_bytes_remain()
    );

    Ok(Self { motions })
  }

  fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> XRayResult {
    todo!("Implement writer")
  }
}
