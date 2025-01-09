use crate::error::database_error::DatabaseError;
use crate::error::database_parse_error::DatabaseParseError;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::{Read, Write};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Vector3d<T = f32> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl Vector3d<f32> {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }

  /// Read vector coordinates from the chunk.
  pub fn read<T: ByteOrder>(reader: &mut dyn Read) -> DatabaseResult<Self> {
    Ok(Self {
      x: reader.read_f32::<T>()?,
      y: reader.read_f32::<T>()?,
      z: reader.read_f32::<T>()?,
    })
  }

  /// Write vector coordinates into the writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut dyn Write) -> DatabaseResult {
    writer.write_f32::<T>(self.x)?;
    writer.write_f32::<T>(self.y)?;
    writer.write_f32::<T>(self.z)?;

    Ok(())
  }
}

impl From<(f32, f32, f32)> for Vector3d<f32> {
  fn from(value: (f32, f32, f32)) -> Self {
    Vector3d::new(value.0, value.1, value.2)
  }
}

impl Display for Vector3d<f32> {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "{},{},{}", self.x, self.y, self.z)
  }
}

impl FromStr for Vector3d<f32> {
  type Err = DatabaseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = s.split(',').collect();

    if parts.len() != 3 {
      return Err(DatabaseParseError::new_database_error(
        "Failed to parse 3d vector from string, expected 3 numbers",
      ));
    }

    Ok(Self {
      x: parts[0]
        .trim()
        .parse::<f32>()
        .or(Err(DatabaseParseError::new_database_error(
          "Failed to parse vector X value",
        )))?,
      y: parts[1]
        .trim()
        .parse::<f32>()
        .or(Err(DatabaseParseError::new_database_error(
          "Failed to parse vector Y value",
        )))?,
      z: parts[2]
        .trim()
        .parse::<f32>()
        .or(Err(DatabaseParseError::new_database_error(
          "Failed to parse vector Z value",
        )))?,
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::data::generic::vector_3d::Vector3d;
  use crate::types::{DatabaseResult, SpawnByteOrder};
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use std::str::FromStr;
  use xray_chunk::{ChunkReader, ChunkWriter};
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult {
    let filename: String = String::from("read_write.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let original: Vector3d = Vector3d {
      x: 1.5,
      y: 2.7,
      z: 3.2,
    };

    original.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 12);

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&get_relative_test_sample_file_path(
        file!(),
        &filename,
      ))?,
      0,
    )?;

    assert_eq!(bytes_written, 12);

    let file: FileSlice =
      open_test_resource_as_slice(&get_relative_test_sample_file_path(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 12 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    assert_eq!(Vector3d::read::<SpawnByteOrder>(&mut reader)?, original);

    Ok(())
  }

  #[test]
  fn test_from_to_str() -> DatabaseResult {
    let original: Vector3d = Vector3d {
      x: 10.5,
      y: 20.7,
      z: 30.2,
    };

    assert_eq!(original.to_string(), "10.5,20.7,30.2");
    assert_eq!(Vector3d::from_str("10.5,20.7,30.2")?, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> DatabaseResult {
    let original: Vector3d = Vector3d {
      x: 10.5,
      y: 20.7,
      z: 30.2,
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
      serde_json::from_str::<Vector3d>(&serialized).unwrap()
    );

    Ok(())
  }
}
