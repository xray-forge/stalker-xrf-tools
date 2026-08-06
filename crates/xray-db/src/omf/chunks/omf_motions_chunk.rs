use crate::data::ogf::ogf_motion::OgfMotion;
use byteorder::{ByteOrder, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::Write;
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter, read_u32_chunk};
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

    let mut chunks: Vec<ChunkReader> = reader.read_children()?;

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

  /// Write motions as nested chunks, where leading chunk 0 stores motions count and
  /// following chunks 1..=N store motions themselves.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    let mut count_writer: ChunkWriter = ChunkWriter::new();

    count_writer.write_u32::<T>(self.motions.len() as u32)?;
    writer.write_all(count_writer.flush_chunk_into_buffer::<T>(0)?.as_slice())?;

    for (index, motion) in self.motions.iter().enumerate() {
      let mut motion_writer: ChunkWriter = ChunkWriter::new();

      motion.write::<T>(&mut motion_writer)?;

      writer.write_all(
        motion_writer
          .flush_chunk_into_buffer::<T>(index as u32 + 1)?
          .as_slice(),
      )?;
    }

    log::info!(
      "Written motions chunk, {} bytes, {} motions",
      writer.bytes_written(),
      self.motions.len()
    );

    Ok(())
  }
}
