use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::export::file_export::{create_export_file, export_ini_to_file};
use crate::export::file_import::{open_ini_config, read_ini_field};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use std::io;
use std::path::Path;

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
  /// Parses binary data into header chunk representation object.
  pub fn read_from_chunk<T: ByteOrder>(mut chunk: Chunk) -> io::Result<HeaderChunk> {
    let version: u32 = chunk.read_u32::<T>()?;
    let guid: u128 = chunk.read_u128::<T>()?;
    let graph_guid: u128 = chunk.read_u128::<T>()?;
    let count: u32 = chunk.read_u32::<T>()?;
    let level_count: u32 = chunk.read_u32::<T>()?;

    log::info!("Parsed header chunk, {:?} bytes", chunk.read_bytes_len());

    assert!(chunk.is_ended(), "Expect header chunk to be ended");

    Ok(HeaderChunk {
      version,
      guid,
      graph_guid,
      count,
      level_count,
    })
  }

  /// Write header data into chunk writer.
  /// Writes header data in binary format.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_u32::<T>(self.version)?;
    writer.write_u128::<T>(self.guid)?;
    writer.write_u128::<T>(self.graph_guid)?;
    writer.write_u32::<T>(self.count)?;
    writer.write_u32::<T>(self.level_count)?;

    log::info!("Written header chunk, {:?} bytes", writer.bytes_written());

    Ok(())
  }

  /// Import header data from provided path.
  /// Parse ini files and populate spawn file.
  pub fn import(path: &Path) -> io::Result<HeaderChunk> {
    let config: Ini = open_ini_config(&path.join("header.ltx"))?;
    let props: &Properties = config
      .section(Some("header"))
      .expect("Patrol section 'header' should be defined in ltx file");

    Ok(HeaderChunk {
      version: read_ini_field("version", props)?,
      guid: read_ini_field("guid", props)?,
      graph_guid: read_ini_field("graph_guid", props)?,
      count: read_ini_field("count", props)?,
      level_count: read_ini_field("level_count", props)?,
    })
  }

  /// Export header data into provided path.
  /// Creates ltx file config with header chunk description.
  pub fn export<T: ByteOrder>(&self, path: &Path) -> io::Result<()> {
    let mut config: Ini = Ini::new();

    config
      .with_section(Some("header"))
      .set("version", self.version.to_string())
      .set("guid", self.guid.to_string())
      .set("graph_guid", self.graph_guid.to_string())
      .set("count", self.count.to_string())
      .set("level_count", self.level_count.to_string());

    export_ini_to_file(&config, &mut create_export_file(&path.join("header.ltx"))?)?;

    log::info!("Exported header chunk");

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
    let chunk: Chunk = Chunk::from_slice(open_test_resource_as_slice(&get_test_chunk_sub_dir(
      &String::from("empty_nested_single.chunk"),
    ))?)?
    .read_child_by_index(0)?;

    let header: io::Result<HeaderChunk> = HeaderChunk::read_from_chunk::<SpawnByteOrder>(chunk);

    assert!(header.is_err(), "Expected failure with empty chunk");

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

    let chunk: Chunk = Chunk::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    assert_eq!(
      HeaderChunk::read_from_chunk::<SpawnByteOrder>(chunk)?,
      header
    );

    Ok(())
  }
}
