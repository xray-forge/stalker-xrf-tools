use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Debug, PartialEq)]
pub struct HeaderChunk {
  pub version: u32,
  pub guid: u128,
  pub graph_guid: u128,
  pub count: u32,
  pub level_count: u32,
}

impl HeaderChunk {
  /// Read header chunk by position descriptor.
  pub fn read_from_chunk<T: ByteOrder>(mut chunk: Chunk) -> io::Result<HeaderChunk> {
    let version: u32 = chunk.read_u32::<T>()?;
    let guid: u128 = chunk.read_u128::<T>()?;
    let graph_guid: u128 = chunk.read_u128::<T>()?;
    let count: u32 = chunk.read_u32::<T>()?;
    let level_count: u32 = chunk.read_u32::<T>()?;

    log::info!("Parsed header chunk, {:?} bytes", chunk.read_bytes_len());

    assert!(chunk.is_ended(), "Expect header chunk to be ended.");

    Ok(HeaderChunk {
      version,
      guid,
      graph_guid,
      count,
      level_count,
    })
  }

  /// Write header data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_u32::<T>(self.version)?;
    writer.write_u128::<T>(self.guid)?;
    writer.write_u128::<T>(self.graph_guid)?;
    writer.write_u32::<T>(self.count)?;
    writer.write_u32::<T>(self.level_count)?;

    log::info!("Written header chunk, {:?} bytes", writer.bytes_written());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::file::header_chunk::HeaderChunk;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, get_test_chunk_sub_dir, open_test_resource_as_slice,
    overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_empty_chunk() -> io::Result<()> {
    let chunk: Chunk = Chunk::from_file(open_test_resource_as_slice(&get_test_chunk_sub_dir(
      &String::from("empty_nested_single.chunk"),
    ))?)?
    .read_child_by_index(0)?;

    let header: io::Result<HeaderChunk> = HeaderChunk::read_from_chunk::<SpawnByteOrder>(chunk);

    assert!(header.is_err(), "Expected failure with empty chunk.");

    Ok(())
  }

  #[test]
  fn test_read_write_simple_header() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_test_chunk_file_sub_dir(file!(), &String::from("header_simple.chunk"));

    let header: HeaderChunk = HeaderChunk {
      version: 20,
      guid: 2u128.pow(127),
      graph_guid: 2u128.pow(64),
      count: 5050,
      level_count: 12,
    };

    header.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 44);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 44);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 52);

    let chunk: Chunk = Chunk::from_file(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    assert_eq!(
      HeaderChunk::read_from_chunk::<SpawnByteOrder>(chunk)?,
      header
    );

    Ok(())
  }
}
