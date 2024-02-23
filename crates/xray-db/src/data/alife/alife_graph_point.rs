use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

pub struct AlifeGraphPoint {
  pub connection_point_name: String,
  pub connection_level_name: String,
  pub location0: u8,
  pub location1: u8,
  pub location2: u8,
  pub location3: u8,
}

impl AlifeObjectInheritedReader<AlifeGraphPoint> for AlifeGraphPoint {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeGraphPoint> {
    let connection_point_name: String = chunk.read_null_terminated_string()?;
    let connection_level_name: String = chunk.read_null_terminated_string()?;
    let location0: u8 = chunk.read_u8()?;
    let location1: u8 = chunk.read_u8()?;
    let location2: u8 = chunk.read_u8()?;
    let location3: u8 = chunk.read_u8()?;

    Ok(AlifeGraphPoint {
      connection_point_name,
      connection_level_name,
      location0,
      location1,
      location2,
      location3,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeGraphPoint {}
