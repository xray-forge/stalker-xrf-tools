use crate::chunk::reader::ChunkReader;
use crate::chunk::utils::read_u32_chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::ogf::ogf_motion::OgfMotion;
use crate::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OmfMotionsChunk {
  pub motions: Vec<OgfMotion>,
}

impl OmfMotionsChunk {
  pub const CHUNK_ID: u32 = 14; // 0x1A, 0xE

  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    log::info!(
      "Reading motions chunk: {:?} bytes",
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
      motions.push(OgfMotion::read::<T>(chunk)?);
    }

    assert!(
      reader.is_ended(),
      "Expect omf motions chunk to be ended, {} remain",
      reader.read_bytes_remain()
    );

    Ok(Self { motions })
  }

  pub fn write<T: ByteOrder>(&self, _: &mut ChunkWriter) -> DatabaseResult {
    Ok(())
  }
}
