use crate::chunk::writer::ChunkWriter;
use byteorder::ByteOrder;
use std::io;

pub trait AlifeObjectGeneric {
  type Order: ByteOrder;

  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()>;
}
