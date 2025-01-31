use crate::constants::META_TYPE_FIELD;
use crate::export::file::create_export_file;
use crate::export::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::path::Path;
use xray_chunk::{ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticlesHeaderChunk {
  pub version: u16,
}

impl ParticlesHeaderChunk {
  pub const META_TYPE: &'static str = "particles_header";
  pub const CHUNK_ID: u32 = 1;

  /// Read version chunk by position descriptor.
  /// Parses binary data into version chunk representation object.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let header_chunk: Self = Self {
      version: reader.read_u16::<T>()?,
    };

    log::info!("Read header chunk, {} bytes", reader.read_bytes_len());

    if header_chunk.version != 1 {
      return Err(XRayError::new_not_implemented_error(
        "Unknown version in particles header chunk, expected v1 only",
      ));
    }

    assert!(reader.is_ended(), "Expect version chunk to be ended");

    Ok(header_chunk)
  }

  /// Write particle header into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_u16::<T>(self.version)?;

    log::info!("Written header chunk, {} bytes", writer.bytes_written());

    Ok(())
  }

  /// Import header data from provided path.
  /// Parse ltx files and populate spawn file.
  pub fn import(path: &Path) -> XRayResult<Self> {
    log::info!("Importing particles header: {}", path.display());

    let ltx: Ltx = Ltx::read_from_path(&path.join("header.ltx"))?;
    let section: &Section = ltx
      .section("header")
      .expect("Patrol section 'header' should be defined in ltx file");

    let meta_type: String = read_ltx_field(META_TYPE_FIELD, section)?;
    let header_chunk: Self = Self {
      version: read_ltx_field("version", section)?,
    };

    assert_eq!(
      meta_type,
      Self::META_TYPE,
      "Expect type metadata to be set as {meta_type}"
    );
    assert_eq!(header_chunk.version, 1, "Expect version chunk to be 1");

    Ok(header_chunk)
  }

  /// Export header data into provided path.
  /// Creates ltx file config with header chunk description.
  pub fn export(&self, path: &Path) -> XRayResult {
    let mut ltx: Ltx = Ltx::new();

    ltx
      .with_section("header")
      .set(META_TYPE_FIELD, Self::META_TYPE)
      .set("version", self.version.to_string());

    ltx.write_to(&mut create_export_file(&path.join("header.ltx"))?)?;

    log::info!("Exported header chunk");

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::particles_file::chunks::particles_header_chunk::ParticlesHeaderChunk;
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use std::path::Path;
  use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_resource_path, get_relative_test_sample_file_directory,
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write_incorrect() -> XRayResult {
    let filename: String =
      get_relative_test_sample_file_path(file!(), "read_write_incorrect.chunk");

    let original: ParticlesHeaderChunk = ParticlesHeaderChunk { version: 2 };

    let mut writer: ChunkWriter = ChunkWriter::new();

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 2);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
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
      ParticlesHeaderChunk::read::<XRayByteOrder>(&mut reader)
        .unwrap_err()
        .to_string(),
      "Not implemented error: Unknown version in particles header chunk, expected v1 only",
    );

    Ok(())
  }

  #[test]
  fn test_read_write() -> XRayResult {
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: ParticlesHeaderChunk = ParticlesHeaderChunk { version: 1 };

    let mut writer: ChunkWriter = ChunkWriter::new();

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 2);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
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
      ParticlesHeaderChunk::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let original: ParticlesHeaderChunk = ParticlesHeaderChunk { version: 1 };

    let export_folder: &Path =
      &get_absolute_test_resource_path(&get_relative_test_sample_file_directory(file!()));

    original.export(export_folder)?;

    let read: ParticlesHeaderChunk = ParticlesHeaderChunk::import(export_folder)?;

    assert_eq!(read, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> XRayResult {
    let original: ParticlesHeaderChunk = ParticlesHeaderChunk { version: 1 };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(json!(original).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      original,
      serde_json::from_str::<ParticlesHeaderChunk>(&serialized).unwrap()
    );

    Ok(())
  }
}
