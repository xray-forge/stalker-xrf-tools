use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::constants::NIL;
use crate::error::database_parse_error::DatabaseParseError;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Time {
  pub year: u8,
  pub month: u8,
  pub day: u8,
  pub hour: u8,
  pub minute: u8,
  pub second: u8,
  pub millis: u16,
}

impl Time {
  /// Read optional time object from the chunk.
  pub fn read_optional<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Option<Self>> {
    if reader.read_u8()? == 1 {
      Ok(Some(Self::read::<T>(reader)?))
    } else {
      Ok(None)
    }
  }

  /// Write optional time object into the writer.
  pub fn write_optional<T: ByteOrder>(
    time: Option<&Self>,
    writer: &mut ChunkWriter,
  ) -> DatabaseResult {
    if time.is_some() {
      writer.write_u8(1)?;

      time.as_ref().unwrap().write::<T>(writer)?;
    } else {
      writer.write_u8(0)?;
    }

    Ok(())
  }

  /// Read time object from chunk.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let year: u8 = reader.read_u8()?;
    let month: u8 = reader.read_u8()?;
    let day: u8 = reader.read_u8()?;
    let hour: u8 = reader.read_u8()?;
    let minute: u8 = reader.read_u8()?;
    let second: u8 = reader.read_u8()?;
    let millis: u16 = reader.read_u16::<T>()?;

    Ok(Self {
      year,
      month,
      day,
      hour,
      minute,
      second,
      millis,
    })
  }

  /// Write time object into the chunk.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_u8(self.year)?;
    writer.write_u8(self.month)?;
    writer.write_u8(self.day)?;
    writer.write_u8(self.hour)?;
    writer.write_u8(self.minute)?;
    writer.write_u8(self.second)?;
    writer.write_u16::<T>(self.millis)?;

    Ok(())
  }

  /// Cast optional time object to serialized string.
  pub fn export_to_string(time: Option<&Self>) -> String {
    time
      .as_ref()
      .map_or(String::from(NIL), |value| value.to_string())
  }

  /// Import optional time from string value.
  pub fn import_from_string(value: &str) -> DatabaseResult<Option<Self>> {
    if value.trim() == NIL {
      return Ok(None);
    }

    Ok(match Self::from_str(value) {
      Ok(time) => Some(time),
      Err(_) => {
        return Err(DatabaseParseError::new_database_error(
          "Failed to parse time",
        ))
      }
    })
  }
}

impl Display for Time {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      formatter,
      "{},{},{},{},{},{},{}",
      self.year, self.month, self.day, self.hour, self.minute, self.second, self.millis
    )
  }
}

impl FromStr for Time {
  type Err = DatabaseParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = s.split(',').map(|it| it.trim()).collect();

    if parts.len() != 7 {
      return Err(DatabaseParseError::new(
        "Failed to parse time object from string",
      ));
    }

    Ok(Self {
      year: parts[0]
        .parse()
        .or(Err(DatabaseParseError::new("Failed to parse year value")))?,
      month: parts[1]
        .parse()
        .or(Err(DatabaseParseError::new("Failed to parse month value")))?,
      day: parts[2]
        .parse()
        .or(Err(DatabaseParseError::new("Failed to parse day value")))?,
      hour: parts[3]
        .parse()
        .or(Err(DatabaseParseError::new("Failed to parse hour value")))?,
      minute: parts[4]
        .parse()
        .or(Err(DatabaseParseError::new("Failed to parse minute value")))?,
      second: parts[5]
        .parse()
        .or(Err(DatabaseParseError::new("Failed to parse second value")))?,
      millis: parts[6]
        .parse()
        .or(Err(DatabaseParseError::new("Failed to parse millis value")))?,
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::time::Time;
  use crate::types::{DatabaseResult, SpawnByteOrder};
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use std::str::FromStr;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: Time = Time {
      year: 22,
      month: 10,
      day: 24,
      hour: 20,
      minute: 30,
      second: 50,
      millis: 250,
    };

    original.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 8);

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 8);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 8 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(Time::read::<SpawnByteOrder>(&mut reader)?, original);

    Ok(())
  }

  #[test]
  fn test_read_write_optional_some() -> DatabaseResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_relative_test_sample_file_path(file!(), "read_write_optional_some.chunk");

    let original: Time = Time {
      year: 22,
      month: 10,
      day: 24,
      hour: 20,
      minute: 30,
      second: 50,
      millis: 250,
    };

    Time::write_optional::<SpawnByteOrder>(Some(&original), &mut writer)?;

    assert_eq!(writer.bytes_written(), 9);

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 9);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 9 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      Time::read_optional::<SpawnByteOrder>(&mut reader)?,
      Some(original)
    );

    Ok(())
  }

  #[test]
  fn test_read_write_optional_none() -> DatabaseResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_relative_test_sample_file_path(file!(), "read_write_optional_none.chunk");

    Time::write_optional::<SpawnByteOrder>(None, &mut writer)?;

    assert_eq!(writer.bytes_written(), 1);

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 1);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 1 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(Time::read_optional::<SpawnByteOrder>(&mut reader)?, None);

    Ok(())
  }

  #[test]
  fn test_import_export_to_str() -> DatabaseResult {
    let original: Time = Time {
      year: 20,
      month: 6,
      day: 1,
      hour: 15,
      minute: 15,
      second: 23,
      millis: 100,
    };

    assert_eq!(
      Time::export_to_string(Some(&original)),
      "20,6,1,15,15,23,100"
    );
    assert_eq!(
      Time::import_from_string("20,6,1,15,15,23,100")?,
      Some(original)
    );
    assert_eq!(Time::export_to_string(None), "nil");
    assert_eq!(Time::import_from_string("nil")?, None);

    Ok(())
  }

  #[test]
  fn test_from_to_str() -> DatabaseResult {
    let original: Time = Time {
      year: 22,
      month: 6,
      day: 1,
      hour: 15,
      minute: 15,
      second: 23,
      millis: 100,
    };

    assert_eq!(original.to_string(), "22,6,1,15,15,23,100");
    assert_eq!(Time::from_str("22,6,1,15,15,23,100").unwrap(), original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> DatabaseResult {
    let original: Time = Time {
      year: 22,
      month: 6,
      day: 1,
      hour: 15,
      minute: 16,
      second: 45,
      millis: 100,
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(json!(original).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(original, serde_json::from_str::<Time>(&serialized).unwrap());

    Ok(())
  }
}
