use crate::export::file::{create_export_file, open_ltx_config};
use crate::export::file_import::read_ltx_field;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::path::Path;
use uuid::Uuid;
use xray_chunk::{ChunkReader, ChunkWriter};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnHeaderChunk {
  pub version: u32,
  pub guid: Uuid,
  pub graph_guid: Uuid,
  pub objects_count: u32,
  pub levels_count: u32,
}

impl SpawnHeaderChunk {
  pub const CHUNK_ID: u32 = 0;

  /// Read header chunk by position descriptor.
  /// Parses binary data into header chunk representation object.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    log::info!("Parsing header chunk, {} bytes", reader.read_bytes_remain());

    let header: Self = Self {
      version: reader.read_u32::<T>()?,
      guid: Uuid::from_u128(reader.read_u128::<T>()?),
      graph_guid: Uuid::from_u128(reader.read_u128::<T>()?),
      objects_count: reader.read_u32::<T>()?,
      levels_count: reader.read_u32::<T>()?,
    };

    assert!(
      reader.is_ended(),
      "Expect header chunk to be ended, {} remain",
      reader.read_bytes_remain()
    );

    Ok(header)
  }

  /// Write header data into chunk writer.
  /// Writes header data in binary format.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_u32::<T>(self.version)?;
    writer.write_u128::<T>(self.guid.as_u128())?;
    writer.write_u128::<T>(self.graph_guid.as_u128())?;
    writer.write_u32::<T>(self.objects_count)?;
    writer.write_u32::<T>(self.levels_count)?;

    log::info!("Written header chunk, {} bytes", writer.bytes_written());

    Ok(())
  }

  /// Import header data from provided path.
  /// Parse ltx files and populate spawn file.
  pub fn import(path: &Path) -> DatabaseResult<Self> {
    let ltx: Ltx = open_ltx_config(&path.join("header.ltx"))?;
    let section: &Section = ltx
      .section("header")
      .expect("Patrol section 'header' should be defined in ltx file");

    Ok(Self {
      version: read_ltx_field("version", section)?,
      guid: read_ltx_field("guid", section)?,
      graph_guid: read_ltx_field("graph_guid", section)?,
      objects_count: read_ltx_field("objects", section)?,
      levels_count: read_ltx_field("level_count", section)?,
    })
  }

  /// Export header data into provided path.
  /// Creates ltx file config with header chunk description.
  pub fn export(&self, path: &Path) -> DatabaseResult {
    let mut ltx: Ltx = Ltx::new();

    ltx
      .with_section("header")
      .set("version", self.version.to_string())
      .set("guid", self.guid.to_string())
      .set("graph_guid", self.graph_guid.to_string())
      .set("objects", self.objects_count.to_string())
      .set("level_count", self.levels_count.to_string());

    ltx.write_to(&mut create_export_file(&path.join("header.ltx"))?)?;

    log::info!("Exported header chunk");

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::spawn_file::chunks::spawn_header_chunk::SpawnHeaderChunk;
  use crate::types::DatabaseResult;
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use std::path::Path;
  use uuid::{uuid, Uuid};
  use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_resource_path, get_relative_test_sample_file_directory,
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_empty() -> DatabaseResult {
    let mut reader: ChunkReader = ChunkReader::from_slice(open_test_resource_as_slice(
      &get_relative_test_sample_file_path(file!(), "read_empty.chunk"),
    )?)?
    .read_child_by_index(0)?;

    let original: DatabaseResult<SpawnHeaderChunk> =
      SpawnHeaderChunk::read::<XRayByteOrder>(&mut reader);

    assert!(original.is_err(), "Expected failure with empty chunk");

    Ok(())
  }

  #[test]
  fn test_read_write() -> DatabaseResult {
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: SpawnHeaderChunk = SpawnHeaderChunk {
      version: 20,
      guid: Uuid::from_u128(2u128.pow(127)),
      graph_guid: Uuid::from_u128(2u128.pow(64)),
      objects_count: 5050,
      levels_count: 12,
    };

    let mut writer: ChunkWriter = ChunkWriter::new();

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 44);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 44);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 52);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    assert_eq!(
      SpawnHeaderChunk::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> DatabaseResult {
    let original: SpawnHeaderChunk = SpawnHeaderChunk {
      version: 10,
      guid: uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
      graph_guid: uuid!("78e55023-10b1-426f-9247-bb680e5fe0d9"),
      objects_count: 550,
      levels_count: 12,
    };

    let export_folder: &Path =
      &get_absolute_test_resource_path(&get_relative_test_sample_file_directory(file!()));

    original.export(export_folder)?;

    assert_eq!(SpawnHeaderChunk::import(export_folder)?, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> DatabaseResult {
    let original: SpawnHeaderChunk = SpawnHeaderChunk {
      version: 12,
      guid: uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
      graph_guid: uuid!("67e55023-10b1-426f-9247-bb680e5fe0c8"),
      objects_count: 6432,
      levels_count: 31,
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(json!(original).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      original,
      serde_json::from_str::<SpawnHeaderChunk>(&serialized).unwrap()
    );

    Ok(())
  }
}
