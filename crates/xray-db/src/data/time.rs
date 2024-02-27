use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, PartialEq)]
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
  pub fn read_optional_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<Option<Time>> {
    if chunk.read_u8()? == 1 {
      Ok(Some(Time::read_from_chunk::<T>(chunk)?))
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
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<Time> {
    let year: u8 = chunk.read_u8()?;
    let month: u8 = chunk.read_u8()?;
    let day: u8 = chunk.read_u8()?;
    let hour: u8 = chunk.read_u8()?;
    let minute: u8 = chunk.read_u8()?;
    let second: u8 = chunk.read_u8()?;
    let millis: u16 = chunk.read_u16::<T>()?;

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

  pub fn to_string(&self) -> String {
    String::from("todo")
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::time::Time;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_time_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_test_chunk_file_sub_dir(file!(), &String::from("time.chunk"));

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
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 8);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 8 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_time: Time = Time::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_time, time);

    Ok(())
  }
}
