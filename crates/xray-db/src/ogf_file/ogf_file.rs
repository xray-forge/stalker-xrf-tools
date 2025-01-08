use crate::chunk::reader::ChunkReader;
use crate::chunk::utils::find_chunk_by_id;
use crate::ogf_file::chunks::ogf_description_chunk::OgfDescriptionChunk;
use crate::ogf_file::chunks::ogf_header_chunk::OgfHeaderChunk;
use crate::DatabaseResult;
use byteorder::ByteOrder;
use fileslice::FileSlice;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

/// FMesh in c++ codebase.
#[derive(Debug, Serialize, Deserialize)]
pub struct OgfFile {
  pub header: OgfHeaderChunk,
  pub description: Option<OgfDescriptionChunk>,
}

impl OgfFile {
  pub fn read_from_path<T: ByteOrder>(path: &Path) -> DatabaseResult<Self> {
    Self::read_from_file::<T>(File::open(path)?)
  }

  pub fn read_from_file<T: ByteOrder>(file: File) -> DatabaseResult<Self> {
    let mut reader: ChunkReader = ChunkReader::from_slice(FileSlice::new(file))?;
    let chunks: Vec<ChunkReader> = ChunkReader::read_all_from_file(&mut reader);

    log::info!(
      "Reading ogf file, {} chunks, {} bytes",
      chunks.len(),
      reader.read_bytes_len(),
    );

    for chunk in &chunks {
      log::info!(
        "Ogf chunk: {} ({:#x}) - {} bytes",
        chunk.id,
        chunk.id,
        chunk.size
      );
    }

    Self::read_from_chunks::<T>(&chunks)
  }

  pub fn read_from_chunks<T: ByteOrder>(chunks: &[ChunkReader]) -> DatabaseResult<Self> {
    Ok(Self {
      header: OgfHeaderChunk::read::<T>(
        &mut find_chunk_by_id(chunks, OgfHeaderChunk::CHUNK_ID)
          .expect("OGF header chunk not found"),
      )?,
      description: find_chunk_by_id(chunks, OgfDescriptionChunk::CHUNK_ID).map(|mut it| {
        OgfDescriptionChunk::read::<T>(&mut it).expect("OGF description chunk is invalid")
      }),
    })
  }
}
