use crate::chunk::chunk::Chunk;
use crate::data::alife_object_base::{AlifeObjectGeneric, AlifeObjectInheritedReader};
use byteorder::ReadBytesExt;

pub struct AlifeGraphPoint {
  pub connection_point_name: String,
  pub connection_level_name: String,
  pub location0: u8,
  pub location1: u8,
  pub location2: u8,
  pub location3: u8,
}

impl AlifeObjectInheritedReader<AlifeGraphPoint> for AlifeGraphPoint {
  fn from_chunk(chunk: &mut Chunk) -> AlifeGraphPoint {
    let connection_point_name: String = chunk.read_null_terminated_string().unwrap();
    let connection_level_name: String = chunk.read_null_terminated_string().unwrap();
    let location0: u8 = chunk.read_u8().unwrap();
    let location1: u8 = chunk.read_u8().unwrap();
    let location2: u8 = chunk.read_u8().unwrap();
    let location3: u8 = chunk.read_u8().unwrap();

    AlifeGraphPoint {
      connection_point_name,
      connection_level_name,
      location0,
      location1,
      location2,
      location3,
    }
  }
}

impl AlifeObjectGeneric for AlifeGraphPoint {}
