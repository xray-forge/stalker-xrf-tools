use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use byteorder::ByteOrder;
use std::io;

pub struct AlifeObjectMotion {
  pub motion_name: String,
}

impl AlifeObjectInheritedReader<AlifeObjectMotion> for AlifeObjectMotion {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectMotion> {
    let motion_name: String = chunk.read_null_terminated_string()?;

    Ok(AlifeObjectMotion { motion_name })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}
