use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::export::file::{create_export_file, export_ini_to_file, open_ini_config};
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use std::io;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct HeaderChunk {
  pub version: u32,
  pub guid: u128,
  pub graph_guid: u128,
  pub objects_count: u32,
  pub level_count: u32,
}

impl HeaderChunk {
  /// Read header chunk by position descriptor.
  /// Parses binary data into header chunk representation object.
  pub fn read_from_chunk<T: ByteOrder>(mut chunk: Chunk) -> io::Result<HeaderChunk> {
    let header: HeaderChunk = HeaderChunk {
      version: chunk.read_u32::<T>()?,
      guid: chunk.read_u128::<T>()?,
      graph_guid: chunk.read_u128::<T>()?,
      objects_count: chunk.read_u32::<T>()?,
      level_count: chunk.read_u32::<T>()?,
    };

    log::info!("Parsed header chunk, {:?} bytes", chunk.read_bytes_len());

    assert!(chunk.is_ended(), "Expect header chunk to be ended");

    Ok(header)
  }

  /// Write header data into chunk writer.
  /// Writes header data in binary format.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_u32::<T>(self.version)?;
    writer.write_u128::<T>(self.guid)?;
    writer.write_u128::<T>(self.graph_guid)?;
    writer.write_u32::<T>(self.objects_count)?;
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
      objects_count: read_ini_field("objects", props)?,
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
      .set("objects", self.objects_count.to_string())
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
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::export::file::{export_ini_to_file, open_ini_config};
  use crate::file::header_chunk::HeaderChunk;
  use crate::test::assertions::files_are_equal_by_path;
  use crate::test::utils::{
    get_test_resource_path, get_test_sample_file_directory, get_test_sample_file_sub_dir,
    get_test_sample_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use ini::Ini;
  use std::io;
  use std::path::Path;

  #[test]
  fn test_read_empty_chunk() -> io::Result<()> {
    let chunk: Chunk = Chunk::from_slice(open_test_resource_as_slice(&get_test_sample_sub_dir(
      "empty_nested_single.chunk",
    ))?)?
    .read_child_by_index(0)?;

    let header: io::Result<HeaderChunk> = HeaderChunk::read_from_chunk::<SpawnByteOrder>(chunk);

    assert!(header.is_err(), "Expected failure with empty chunk");

    Ok(())
  }

  #[test]
  fn test_read_write_simple_header() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_test_sample_file_sub_dir(file!(), "header_simple.chunk");

    let header: HeaderChunk = HeaderChunk {
      version: 20,
      guid: 2u128.pow(127),
      graph_guid: 2u128.pow(64),
      objects_count: 5050,
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

  #[test]
  fn test_import_export_object() -> io::Result<()> {
    let header: HeaderChunk = HeaderChunk {
      version: 10,
      guid: 284795912783782493,
      graph_guid: 276845268795248675,
      objects_count: 550,
      level_count: 12,
    };

    let export_folder: &Path = &get_test_resource_path(&get_test_sample_file_directory(file!()));

    header.export::<SpawnByteOrder>(export_folder)?;

    let read_header: HeaderChunk = HeaderChunk::import(export_folder)?;

    assert_eq!(read_header, header);

    Ok(())
  }
}
