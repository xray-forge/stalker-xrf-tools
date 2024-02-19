use crate::chunk::chunk::Chunk;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

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
  pub fn from_chunk(chunk: &mut Chunk) -> Option<Time> {
    let has_time: u8 = chunk.read_u8().unwrap();

    if has_time == 0 {
      None
    } else {
      let year: u8 = chunk.read_u8().unwrap();
      let month: u8 = chunk.read_u8().unwrap();
      let day: u8 = chunk.read_u8().unwrap();
      let hour: u8 = chunk.read_u8().unwrap();
      let minute: u8 = chunk.read_u8().unwrap();
      let second: u8 = chunk.read_u8().unwrap();
      let millis: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();

      Some(Time {
        year,
        month,
        day,
        hour,
        minute,
        second,
        millis,
      })
    }
  }
}
