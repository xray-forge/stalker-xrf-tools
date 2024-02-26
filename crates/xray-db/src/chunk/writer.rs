use crate::data::shape::Shape;
use crate::types::{Matrix3d, Sphere3d, U32Bytes, Vector3d};
use byteorder::{ByteOrder, WriteBytesExt};
use std::fs::File;
use std::io;
use std::io::Write;

pub struct ChunkWriter {
  pub buffer: Vec<u8>,
}

impl ChunkWriter {
  pub fn new() -> ChunkWriter {
    ChunkWriter { buffer: Vec::new() }
  }

  /// Flush all the written data as chunk into the file.
  pub fn flush_chunk_into_file<T: ByteOrder>(
    &mut self,
    file: &mut File,
    index: u32,
  ) -> io::Result<usize> {
    self.buffer.flush()?;

    file.write_u32::<T>(index)?;
    file.write_u32::<T>(self.buffer.len() as u32)?;
    file.write(self.buffer.as_slice())
  }

  /// Flush all the written data as chunk into the file.
  pub fn flush_chunk_into_buffer<T: ByteOrder>(&mut self, index: usize) -> io::Result<Vec<u8>> {
    self.buffer.flush()?;

    let mut buffer: Vec<u8> = Vec::new();

    buffer.write_u32::<T>(index as u32)?;
    buffer.write_u32::<T>(self.buffer.len() as u32)?;
    buffer.write_all(self.buffer.as_slice())?;

    Ok(buffer)
  }

  /// Flush all the written data as chunk into the file.
  pub fn flush_raw_into_buffer(&mut self) -> io::Result<Vec<u8>> {
    self.buffer.flush()?;

    let mut buffer: Vec<u8> = Vec::new();

    buffer.write_all(self.buffer.as_slice())?;

    Ok(buffer)
  }

  /// Get count of bytes written into internal buffer.
  pub fn bytes_written(&self) -> usize {
    self.buffer.len()
  }

  /// Write null terminated string.
  pub fn write_null_terminated_string(&mut self, value: &String) -> io::Result<usize> {
    Ok(self.write(value.as_bytes())? + self.write(&[0u8])?)
  }

  /// Write three float values.
  pub fn write_f32_3d_vector<T: ByteOrder>(&mut self, value: &Vector3d<f32>) -> io::Result<()> {
    self.write_f32::<T>(value.0)?;
    self.write_f32::<T>(value.1)?;
    self.write_f32::<T>(value.2)?;

    Ok(())
  }

  /// Write 4 bytes value as 4 separate byte entries.
  pub fn write_u32_bytes(&mut self, value: &U32Bytes) -> io::Result<()> {
    self.write_u8(value.0)?;
    self.write_u8(value.1)?;
    self.write_u8(value.2)?;
    self.write_u8(value.3)?;

    Ok(())
  }

  /// Write shapes data.
  pub fn write_shape_description<T: ByteOrder>(&mut self, shapes: &Vec<Shape>) -> io::Result<()> {
    self.write_u8(shapes.len() as u8)?;

    for shape in shapes {
      match shape {
        Shape::Sphere(sphere_object) => {
          self.write_u8(0)?;
          self.write_sphere::<T>(sphere_object)?;
        }
        Shape::Box(box_object) => {
          self.write_u8(1)?;
          self.write_matrix::<T>(box_object)?;
        }
      }
    }

    Ok(())
  }

  pub fn write_sphere<T: ByteOrder>(&mut self, sphere: &Sphere3d) -> io::Result<()> {
    self.write_f32_3d_vector::<T>(&sphere.0)?;
    self.write_f32::<T>(sphere.1)?;

    Ok(())
  }

  pub fn write_matrix<T: ByteOrder>(&mut self, matrix: &Matrix3d) -> io::Result<()> {
    self.write_f32_3d_vector::<T>(&matrix.0)?;
    self.write_f32_3d_vector::<T>(&matrix.1)?;
    self.write_f32_3d_vector::<T>(&matrix.2)?;
    self.write_f32_3d_vector::<T>(&matrix.3)?;

    Ok(())
  }

  /// Write serialized vector into vector, where u32 count N is followed by N u16 entries.
  pub fn write_u16_vector<T: ByteOrder>(&mut self, vector: &Vec<u16>) -> io::Result<()> {
    self.write_u32::<T>(vector.len() as u32)?;

    for it in vector {
      self.write_u16::<T>(*it)?;
    }

    Ok(())
  }
}

impl Write for ChunkWriter {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    self.buffer.write(buf)
  }

  fn flush(&mut self) -> io::Result<()> {
    self.buffer.flush()
  }
}
