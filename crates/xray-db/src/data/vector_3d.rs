use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct Vector3d<T = f32> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl Vector3d<f32> {
  pub fn new(x: f32, y: f32, z: f32) -> Vector3d {
    Vector3d { x, y, z }
  }

  /// Read vector coordinates from the chunk.
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<Vector3d<f32>> {
    Ok(Vector3d {
      x: chunk.read_f32::<T>()?,
      y: chunk.read_f32::<T>()?,
      z: chunk.read_f32::<T>()?,
    })
  }

  /// Write vector coordinates into the writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_f32::<T>(self.x)?;
    writer.write_f32::<T>(self.y)?;
    writer.write_f32::<T>(self.z)?;

    Ok(())
  }

  pub fn to_string(&self) -> String {
    format!("{},{},{}", self.x, self.y, self.z)
  }
}
