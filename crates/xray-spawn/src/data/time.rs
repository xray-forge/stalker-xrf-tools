use crate::chunk::chunk::Chunk;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;
use std::io;

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
  pub fn read_from_chunk(chunk: &mut Chunk) -> io::Result<Option<Time>> {
    let has_time: u8 = chunk.read_u8()?;

    if has_time == 0 {
      Ok(None)
    } else {
      let year: u8 = chunk.read_u8()?;
      let month: u8 = chunk.read_u8()?;
      let day: u8 = chunk.read_u8()?;
      let hour: u8 = chunk.read_u8()?;
      let minute: u8 = chunk.read_u8()?;
      let second: u8 = chunk.read_u8()?;
      let millis: u16 = chunk.read_u16::<SpawnByteOrder>()?;

      Ok(Some(Time {
        year,
        month,
        day,
        hour,
        minute,
        second,
        millis,
      }))
    }
  }
}
