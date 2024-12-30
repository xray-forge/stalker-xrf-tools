use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::error::database_not_implemented_error::DatabaseNotImplementedError;
use crate::export::file::{create_export_file, open_ini_config};
use crate::export::file_import::read_ini_field;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::path::Path;
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticlesHeaderChunk {
  pub version: u16,
}

impl ParticlesHeaderChunk {
  pub const CHUNK_ID: u32 = 1;

  /// Read version chunk by position descriptor.
  /// Parses binary data into version chunk representation object.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticlesHeaderChunk> {
    let header_chunk: ParticlesHeaderChunk = ParticlesHeaderChunk {
      version: reader.read_u16::<T>()?,
    };

    log::info!("Read version chunk, {:?} bytes", reader.read_bytes_len());

    if header_chunk.version != 1 {
      return Err(DatabaseNotImplementedError::new_database_error(
        "Unknown version in particles header chunk, expected v1 only",
      ));
    }

    assert!(reader.is_ended(), "Expect version chunk to be ended");

    Ok(header_chunk)
  }

  /// Write particle header into chunk writer.
  pub fn write<T: ByteOrder>(self: &Self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    writer.write_u16::<T>(self.version)?;

    Ok(())
  }

  /// Import header data from provided path.
  /// Parse ini files and populate spawn file.
  pub fn import(path: &Path) -> DatabaseResult<ParticlesHeaderChunk> {
    let config: Ltx = open_ini_config(&path.join("header.ltx"))?;
    let section: &Section = config
      .section("header")
      .expect("Patrol section 'header' should be defined in ltx file");

    let header_chunk: ParticlesHeaderChunk = ParticlesHeaderChunk {
      version: read_ini_field("version", section)?,
    };

    assert_eq!(header_chunk.version, 1, "Expect version chunk to be 1");

    Ok(header_chunk)
  }

  /// Export header data into provided path.
  /// Creates ltx file config with header chunk description.
  pub fn export<T: ByteOrder>(&self, path: &Path) -> DatabaseResult<()> {
    let mut ltx: Ltx = Ltx::new();

    ltx
      .with_section("header")
      .set("version", self.version.to_string());

    ltx.write_to(&mut create_export_file(&path.join("header.ltx"))?)?;

    log::info!("Exported header chunk");

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::particles_file::particles_header_chunk::ParticlesHeaderChunk;
  use crate::types::{DatabaseResult, SpawnByteOrder};
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use std::path::Path;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_resource_path, get_relative_test_sample_file_directory,
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write_incorrect() -> DatabaseResult<()> {
    let filename: String =
      get_relative_test_sample_file_path(file!(), "read_write_incorrect.chunk");

    let header: ParticlesHeaderChunk = ParticlesHeaderChunk { version: 2 };

    let mut writer: ChunkWriter = ChunkWriter::new();

    header.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 2);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 2);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 2 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    assert_eq!(
      ParticlesHeaderChunk::read::<SpawnByteOrder>(&mut reader).map_err(|error| error.to_string()),
      Err(String::from(
        "Database not implemented error: Unknown version in particles header chunk, expected v1 only",
      ))
    );

    Ok(())
  }

  #[test]
  fn test_read_write() -> DatabaseResult<()> {
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let header: ParticlesHeaderChunk = ParticlesHeaderChunk { version: 1 };

    let mut writer: ChunkWriter = ChunkWriter::new();

    header.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 2);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 2);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 2 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    assert_eq!(
      ParticlesHeaderChunk::read::<SpawnByteOrder>(&mut reader)?,
      header
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> DatabaseResult<()> {
    let header: ParticlesHeaderChunk = ParticlesHeaderChunk { version: 1 };

    let export_folder: &Path =
      &get_absolute_test_resource_path(&get_relative_test_sample_file_directory(file!()));

    header.export::<SpawnByteOrder>(export_folder)?;

    let read_header: ParticlesHeaderChunk = ParticlesHeaderChunk::import(export_folder)?;

    assert_eq!(read_header, header);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> DatabaseResult<()> {
    let header: ParticlesHeaderChunk = ParticlesHeaderChunk { version: 1 };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(json!(header).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      header,
      serde_json::from_str::<ParticlesHeaderChunk>(&serialized).unwrap()
    );

    Ok(())
  }
}
