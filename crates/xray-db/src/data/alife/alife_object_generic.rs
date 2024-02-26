use crate::chunk::writer::ChunkWriter;
use byteorder::ByteOrder;
use std::fmt::Debug;
use std::io;

pub trait AlifeObjectGeneric: Debug + Send {
  type Order: ByteOrder;

  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()>;
}
