use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::constants::NIL;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::io;
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

#[derive(Debug)]
pub enum TimeError {
  ParsingError(String),
}

impl Time {
  /// Read optional time object from the chunk.
  pub fn read_optional<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<Option<Time>> {
    if reader.read_u8()? == 1 {
      Ok(Some(Time::read::<T>(reader)?))
    } else {
      Ok(None)
    }
  }

  /// Write optional time object into the writer.
  pub fn write_optional<T: ByteOrder>(
    time: &Option<Time>,
    writer: &mut ChunkWriter,
  ) -> io::Result<()> {
    if time.is_some() {
      writer.write_u8(1)?;

      time.as_ref().unwrap().write::<T>(writer)?;
    } else {
      writer.write_u8(0)?;
    }

    Ok(())
  }

  /// Read time object from chunk.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<Time> {
    let year: u8 = reader.read_u8()?;
    let month: u8 = reader.read_u8()?;
    let day: u8 = reader.read_u8()?;
    let hour: u8 = reader.read_u8()?;
    let minute: u8 = reader.read_u8()?;
    let second: u8 = reader.read_u8()?;
    let millis: u16 = reader.read_u16::<T>()?;

    Ok(Time {
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
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
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
  pub fn export_to_string(time: &Option<Time>) -> String {
    time.as_ref().map_or(String::from(NIL), |t| t.to_string())
  }

  /// Import optional time from string value.
  pub fn import_from_string(s: &str) -> io::Result<Option<Time>> {
    if s.trim() == NIL {
      return Ok(None);
    }

    Ok(match Time::from_str(s) {
      Ok(time) => Some(time),
      Err(_) => {
        return Err(io::Error::new(
          io::ErrorKind::InvalidInput,
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
      self.year, self.minute, self.day, self.hour, self.minute, self.second, self.millis
    )
  }
}

impl FromStr for Time {
  type Err = TimeError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = s.split(',').map(|it| it.trim()).collect();

    if parts.len() != 7 {
      return Err(TimeError::ParsingError(String::from(
        "Failed to parse time object from string",
      )));
    }

    Ok(Time {
      year: parts[0]
        .parse()
        .or(Err(TimeError::ParsingError(String::from(
          "Failed to parse year value",
        ))))?,
      month: parts[1]
        .parse()
        .or(Err(TimeError::ParsingError(String::from(
          "Failed to parse month value",
        ))))?,
      day: parts[2]
        .parse()
        .or(Err(TimeError::ParsingError(String::from(
          "Failed to parse day value",
        ))))?,
      hour: parts[3]
        .parse()
        .or(Err(TimeError::ParsingError(String::from(
          "Failed to parse hour value",
        ))))?,
      minute: parts[4]
        .parse()
        .or(Err(TimeError::ParsingError(String::from(
          "Failed to parse minute value",
        ))))?,
      second: parts[5]
        .parse()
        .or(Err(TimeError::ParsingError(String::from(
          "Failed to parse second value",
        ))))?,
      millis: parts[6]
        .parse()
        .or(Err(TimeError::ParsingError(String::from(
          "Failed to parse millis value",
        ))))?,
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::time::Time;
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write_time_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "time.chunk");

    let time: Time = Time {
      year: 22,
      month: 10,
      day: 24,
      hour: 20,
      minute: 30,
      second: 50,
      millis: 250,
    };

    time.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 8);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 8);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 8 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_time: Time = Time::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_time, time);

    Ok(())
  }
}
