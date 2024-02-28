use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::export::file_export::{create_export_file, export_ini_to_file};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::Ini;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

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
  /// Writes header data in binary format to provided writer.
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
  /// Parse exported ini files and populate correct spawn file.
  pub fn import(path: &Path) -> io::Result<HeaderChunk> {
    let config: Ini = match Ini::load_from_file(path.join("header.ltx")) {
      Ok(ini) => ini,
      Err(error) => {
        return Err(io::Error::new(
          io::ErrorKind::InvalidInput,
          error.to_string(),
        ))
      }
    };

    Ok(HeaderChunk {
      version: config
        .general_section()
        .get("version")
        .expect("'version' to be in header config")
        .parse::<u32>()
        .expect("'version' to be valid u32"),
      guid: config
        .general_section()
        .get("guid")
        .expect("'guid' to be in header config")
        .parse::<u128>()
        .expect("'guid' to be valid u128"),
      graph_guid: config
        .general_section()
        .get("graph_guid")
        .expect("'graph_guid' to be in header config")
        .parse::<u128>()
        .expect("'graph_guid' to be valid u128"),
      count: config
        .general_section()
        .get("count")
        .expect("'count' to be in header config")
        .parse::<u32>()
        .expect("'count' to be valid u32"),
      level_count: config
        .general_section()
        .get("level_count")
        .expect("'level_count' to be in header config")
        .parse::<u32>()
        .expect("Level count to be valid u32"),
    })
  }

  /// Export header data into provided path.
  /// Creates ltx file config with header chunk description.
  pub fn export<T: ByteOrder>(&self, path: &Path) -> io::Result<()> {
    let header_path: PathBuf = path.join("header.ltx");

    let mut file: File = create_export_file(&header_path)?;
    let mut config: Ini = Ini::new();

    config
      .with_general_section()
      .set("version", self.version.to_string())
      .set("guid", self.guid.to_string())
      .set("graph_guid", self.graph_guid.to_string())
      .set("count", self.count.to_string())
      .set("level_count", self.level_count.to_string());

    export_ini_to_file(&config, &mut file)?;

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
