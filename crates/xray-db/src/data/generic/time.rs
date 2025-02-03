use crate::constants::NIL;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use xray_chunk::{ChunkReadWrite, ChunkReadWriteOptional, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Display)]
#[serde(rename_all = "camelCase")]
#[display("{year},{month},{day},{hour},{minute},{second},{millis}")]
pub struct Time {
  pub year: u8,
  pub month: u8,
  pub day: u8,
  pub hour: u8,
  pub minute: u8,
  pub second: u8,
  pub millis: u16,
}

impl ChunkReadWriteOptional for Time {
  /// Read optional time object from the chunk.
  fn read_optional<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Option<Self>> {
    if reader.read_u8()? == 1 {
      Ok(Some(Self::read::<T>(reader)?))
    } else {
      Ok(None)
    }
  }

  /// Write optional time object into the writer.
  fn write_optional<T: ByteOrder>(writer: &mut ChunkWriter, time: Option<&Self>) -> XRayResult {
    if time.is_some() {
      writer.write_u8(1)?;

      time.as_ref().unwrap().write::<T>(writer)?;
    } else {
      writer.write_u8(0)?;
    }

    Ok(())
  }
}

impl ChunkReadWrite for Time {
  /// Read time object from chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
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
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_u8(self.year)?;
    writer.write_u8(self.month)?;
    writer.write_u8(self.day)?;
    writer.write_u8(self.hour)?;
    writer.write_u8(self.minute)?;
    writer.write_u8(self.second)?;
    writer.write_u16::<T>(self.millis)?;

    Ok(())
  }
}

impl Time {
  /// Cast optional time object to serialized string.
  pub fn export_to_string(time: Option<&Self>) -> String {
    time
      .as_ref()
      .map_or(String::from(NIL), |value| value.to_string())
  }

  /// Import optional time from string value.
  pub fn from_str_optional(value: &str) -> XRayResult<Option<Self>> {
    if value.trim() == NIL {
      return Ok(None);
    }

    Self::from_str(value).map(Some)
  }
}

impl FromStr for Time {
  type Err = XRayError;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = string.split(',').map(str::trim).collect();

    if parts.len() != 7 {
      return Err(XRayError::new_parsing_error(
        "Failed to parse time object from string",
      ));
    }

    Ok(Self {
      year: parts[0].parse().or(Err(XRayError::new_parsing_error(
        "Failed to parse years value",
      )))?,
      month: parts[1].parse().or(Err(XRayError::new_parsing_error(
        "Failed to parse months value",
      )))?,
      day: parts[2].parse().or(Err(XRayError::new_parsing_error(
        "Failed to parse days value",
      )))?,
      hour: parts[3].parse().or(Err(XRayError::new_parsing_error(
        "Failed to parse hours value",
      )))?,
      minute: parts[4].parse().or(Err(XRayError::new_parsing_error(
        "Failed to parse minutes value",
      )))?,
      second: parts[5].parse().or(Err(XRayError::new_parsing_error(
        "Failed to parse seconds value",
      )))?,
      millis: parts[6].parse().or(Err(XRayError::new_parsing_error(
        "Failed to parse millis value",
      )))?,
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::data::generic::time::Time;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use std::str::FromStr;
  use xray_chunk::{
    ChunkReadWrite, ChunkReadWriteOptional, ChunkReader, ChunkWriter, XRayByteOrder,
  };
  use xray_error::XRayResult;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };
  use xray_test_utils::FileSlice;

  #[test]
  fn test_read_write() -> XRayResult {
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

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 8);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 8);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 8 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(Time::read::<XRayByteOrder>(&mut reader)?, original);

    Ok(())
  }

  #[test]
  fn test_read_write_optional_some() -> XRayResult {
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

    writer.write_xr_optional::<XRayByteOrder, _>(Some(&original))?;

    assert_eq!(writer.bytes_written(), 9);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 9);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 9 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      Time::read_optional::<XRayByteOrder>(&mut reader)?,
      Some(original)
    );

    Ok(())
  }

  #[test]
  fn test_read_write_optional_none() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_relative_test_sample_file_path(file!(), "read_write_optional_none.chunk");

    Time::write_optional::<XRayByteOrder>(&mut writer, None)?;

    assert_eq!(writer.bytes_written(), 1);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 1);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 1 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(Time::read_optional::<XRayByteOrder>(&mut reader)?, None);

    Ok(())
  }

  #[test]
  fn test_import_export_to_str() -> XRayResult {
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
      Time::from_str_optional("20,6,1,15,15,23,100")?,
      Some(original)
    );
    assert_eq!(Time::export_to_string(None), "nil");
    assert_eq!(Time::from_str_optional("nil")?, None);

    Ok(())
  }

  #[test]
  fn test_from_to_str() -> XRayResult {
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
  fn test_serialize_deserialize() -> XRayResult {
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
