use crate::{DatabaseError, DatabaseResult, OgfFile};
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkIterator, ChunkReader, ChunkWriter};

#[derive(Debug, Serialize, Deserialize)]
pub struct OgfChildrenChunk {
  pub nested: Vec<OgfFile>,
}

impl OgfChildrenChunk {
  pub const CHUNK_ID: u32 = 9;

  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    log::info!(
      "Reading children chunk: {} bytes",
      reader.read_bytes_remain()
    );

    let mut children: Vec<OgfFile> = Vec::new();

    for (index, mut object_reader) in (0..).zip(ChunkIterator::new(reader)) {
      if object_reader.id != index {
        return Err(DatabaseError::new_not_expected_error(format!(
          "Invalid data in OGF children chunk, expected index {}, got {}",
          index, object_reader.id
        )));
      }

      children.push(OgfFile::read_from_chunk::<T>(&mut object_reader)?);
    }

    assert!(
      reader.is_ended(),
      "Expect all data to be read from ogf children, {} remain",
      reader.read_bytes_remain()
    );

    Ok(Self { nested: children })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    for (index, _child) in self.nested.iter().enumerate() {
      let mut child_writer: ChunkWriter = ChunkWriter::new();

      // todo: Child write.

      child_writer.flush_chunk_into::<T>(writer, index as u32)?;
    }

    todo!("Implement OGF file writer here");
  }
}
