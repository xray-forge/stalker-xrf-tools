use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::export::file::{create_export_file, export_ini_to_file, open_ini_config};
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;
use uuid::Uuid;
use xray_ltx::{Ltx, Properties};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HeaderChunk {
  #[serde(rename = "version")]
  pub version: u32,
  #[serde(rename = "guid")]
  pub guid: Uuid,
  #[serde(rename = "graphGuid")]
  pub graph_guid: Uuid,
  #[serde(rename = "objectsCount")]
  pub objects_count: u32,
  #[serde(rename = "levelCount")]
  pub levels_count: u32,
}

impl HeaderChunk {
  /// Read header chunk by position descriptor.
  /// Parses binary data into header chunk representation object.
  pub fn read<T: ByteOrder>(mut reader: ChunkReader) -> io::Result<HeaderChunk> {
    let header: HeaderChunk = HeaderChunk {
      version: reader.read_u32::<T>()?,
      guid: Uuid::from_u128(reader.read_u128::<T>()?),
      graph_guid: Uuid::from_u128(reader.read_u128::<T>()?),
      objects_count: reader.read_u32::<T>()?,
      levels_count: reader.read_u32::<T>()?,
    };

    log::info!("Parsed header chunk, {:?} bytes", reader.read_bytes_len());

    assert!(reader.is_ended(), "Expect header chunk to be ended");

    Ok(header)
  }

  /// Write header data into chunk writer.
  /// Writes header data in binary format.
  pub fn write<T: ByteOrder>(&self, mut writer: ChunkWriter) -> io::Result<ChunkWriter> {
    writer.write_u32::<T>(self.version)?;
    writer.write_u128::<T>(self.guid.as_u128())?;
    writer.write_u128::<T>(self.graph_guid.as_u128())?;
    writer.write_u32::<T>(self.objects_count)?;
    writer.write_u32::<T>(self.levels_count)?;

    log::info!("Written header chunk, {:?} bytes", writer.bytes_written());

    Ok(writer)
  }

  /// Import header data from provided path.
  /// Parse ini files and populate spawn file.
  pub fn import(path: &Path) -> io::Result<HeaderChunk> {
    let config: Ltx = open_ini_config(&path.join("header.ltx"))?;
    let props: &Properties = config
      .section("header")
      .expect("Patrol section 'header' should be defined in ltx file");

    Ok(HeaderChunk {
      version: read_ini_field("version", props)?,
      guid: read_ini_field("guid", props)?,
      graph_guid: read_ini_field("graph_guid", props)?,
      objects_count: read_ini_field("objects", props)?,
      levels_count: read_ini_field("level_count", props)?,
    })
  }

  /// Export header data into provided path.
  /// Creates ltx file config with header chunk description.
  pub fn export<T: ByteOrder>(&self, path: &Path) -> io::Result<()> {
    let mut config: Ltx = Ltx::new();

    config
      .with_section("header")
      .set("version", self.version.to_string())
      .set("guid", self.guid.to_string())
      .set("graph_guid", self.graph_guid.to_string())
      .set("objects", self.objects_count.to_string())
      .set("level_count", self.levels_count.to_string());

    export_ini_to_file(&config, &mut create_export_file(&path.join("header.ltx"))?)?;

    log::info!("Exported header chunk");

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::file::header_chunk::HeaderChunk;
  use crate::test::file::read_file_as_string;
  use crate::test::utils::{
    get_absolute_test_resource_path, get_relative_test_sample_file_directory,
    get_relative_test_sample_file_path, get_relative_test_sample_sub_dir,
    open_test_resource_as_slice, overwrite_test_relative_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io;
  use std::io::{Seek, SeekFrom, Write};
  use std::path::Path;
  use uuid::{uuid, Uuid};

  #[test]
  fn test_read_empty_chunk() -> io::Result<()> {
    let reader: ChunkReader = ChunkReader::from_slice(open_test_resource_as_slice(
      &get_relative_test_sample_sub_dir("empty_nested_single.chunk"),
    )?)?
    .read_child_by_index(0)?;

    let header: io::Result<HeaderChunk> = HeaderChunk::read::<SpawnByteOrder>(reader);

    assert!(header.is_err(), "Expected failure with empty chunk");

    Ok(())
  }

  #[test]
  fn test_read_write_simple_header() -> io::Result<()> {
    let filename: String = get_relative_test_sample_file_path(file!(), "header_simple.chunk");

    let header: HeaderChunk = HeaderChunk {
      version: 20,
      guid: Uuid::from_u128(2u128.pow(127)),
      graph_guid: Uuid::from_u128(2u128.pow(64)),
      objects_count: 5050,
      levels_count: 12,
    };

    let mut writer: ChunkWriter = header.write::<SpawnByteOrder>(ChunkWriter::new())?;

    assert_eq!(writer.bytes_written(), 44);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 44);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 52);

    let reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    assert_eq!(HeaderChunk::read::<SpawnByteOrder>(reader)?, header);

    Ok(())
  }

  #[test]
  fn test_import_export_object() -> io::Result<()> {
    let header: HeaderChunk = HeaderChunk {
      version: 10,
      guid: uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
      graph_guid: uuid!("78e55023-10b1-426f-9247-bb680e5fe0d9"),
      objects_count: 550,
      levels_count: 12,
    };

    let export_folder: &Path =
      &get_absolute_test_resource_path(&get_relative_test_sample_file_directory(file!()));

    header.export::<SpawnByteOrder>(export_folder)?;

    let read_header: HeaderChunk = HeaderChunk::import(export_folder)?;

    assert_eq!(read_header, header);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize_object() -> io::Result<()> {
    let header: HeaderChunk = HeaderChunk {
      version: 12,
      guid: uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
      graph_guid: uuid!("67e55023-10b1-426f-9247-bb680e5fe0c8"),
      objects_count: 6432,
      levels_count: 31,
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialized.json"),
    )?;

    file.write_all(json!(header).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(header, serde_json::from_str::<HeaderChunk>(&serialized)?);

    Ok(())
  }
}
