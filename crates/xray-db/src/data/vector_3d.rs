use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vector3d<T = f32> {
  pub x: T,
  pub y: T,
  pub z: T,
}

#[derive(Debug)]
pub enum Vector3dError {
  ParsingError(String),
}

impl Vector3d<f32> {
  pub fn new(x: f32, y: f32, z: f32) -> Vector3d {
    Vector3d { x, y, z }
  }

  /// Read vector coordinates from the chunk.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<Vector3d<f32>> {
    Ok(Vector3d {
      x: reader.read_f32::<T>()?,
      y: reader.read_f32::<T>()?,
      z: reader.read_f32::<T>()?,
    })
  }

  /// Write vector coordinates into the writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_f32::<T>(self.x)?;
    writer.write_f32::<T>(self.y)?;
    writer.write_f32::<T>(self.z)?;

    Ok(())
  }
}

impl Display for Vector3d<f32> {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "{},{},{}", self.x, self.y, self.z)
  }
}

impl FromStr for Vector3d<f32> {
  type Err = Vector3dError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = s.split(',').collect();

    if parts.len() != 3 {
      return Err(Vector3dError::ParsingError(String::from(
        "Failed to parse 3d vector from string, expected 3 numbers",
      )));
    }

    Ok(Vector3d {
      x: parts[0]
        .trim()
        .parse::<f32>()
        .or(Err(Vector3dError::ParsingError(String::from(
          "Failed to parse vector X value",
        ))))?,
      y: parts[1]
        .trim()
        .parse::<f32>()
        .or(Err(Vector3dError::ParsingError(String::from(
          "Failed to parse vector Y value",
        ))))?,
      z: parts[2]
        .trim()
        .parse::<f32>()
        .or(Err(Vector3dError::ParsingError(String::from(
          "Failed to parse vector Z value",
        ))))?,
    })
  }
}
