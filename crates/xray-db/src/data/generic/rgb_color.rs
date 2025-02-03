use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Display)]
#[serde(rename_all = "camelCase")]
#[display("{r},{g},{b}")]
pub struct RgbColor {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

impl ChunkReadWrite for RgbColor {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      r: reader.read_f32::<T>()?,
      g: reader.read_f32::<T>()?,
      b: reader.read_f32::<T>()?,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_f32::<T>(self.r)?;
    writer.write_f32::<T>(self.g)?;
    writer.write_f32::<T>(self.b)?;

    Ok(())
  }
}

impl FromStr for RgbColor {
  type Err = XRayError;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = string.split(',').map(str::trim).collect();

    if parts.len() != 3 {
      return Err(XRayError::new_parsing_error(
        "Failed to parse rgb color from string, expected 3 numbers",
      ));
    }

    Ok(Self {
      r: parts[0]
        .parse::<f32>()
        .or(Err(XRayError::new_parsing_error(
          "Failed to parse color R value",
        )))?,
      g: parts[1]
        .parse::<f32>()
        .or(Err(XRayError::new_parsing_error(
          "Failed to parse color G value",
        )))?,
      b: parts[2]
        .parse::<f32>()
        .or(Err(XRayError::new_parsing_error(
          "Failed to parse color B value",
        )))?,
    })
  }
}
