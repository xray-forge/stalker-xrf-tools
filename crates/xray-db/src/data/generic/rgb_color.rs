use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::str::FromStr;
use xray_error::{XRayError, XRayResult};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Display)]
#[serde(rename_all = "camelCase")]
#[display("{r},{g},{b}")]
pub struct RgbColor {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

impl RgbColor {
  pub fn read<T: ByteOrder>(reader: &mut dyn Read) -> XRayResult<Self> {
    Ok(Self {
      r: reader.read_f32::<T>()?,
      g: reader.read_f32::<T>()?,
      b: reader.read_f32::<T>()?,
    })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut dyn Write) -> XRayResult {
    writer.write_f32::<T>(self.r)?;
    writer.write_f32::<T>(self.g)?;
    writer.write_f32::<T>(self.b)?;

    Ok(())
  }
}

impl FromStr for RgbColor {
  type Err = XRayError;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = string.split(',').collect();

    if parts.len() != 3 {
      return Err(XRayError::new_parsing_error(
        "Failed to parse rgb color from string, expected 3 numbers",
      ));
    }

    Ok(Self {
      r: parts[0]
        .trim()
        .parse::<f32>()
        .or(Err(XRayError::new_parsing_error(
          "Failed to parse color R value",
        )))?,
      g: parts[1]
        .trim()
        .parse::<f32>()
        .or(Err(XRayError::new_parsing_error(
          "Failed to parse color G value",
        )))?,
      b: parts[2]
        .trim()
        .parse::<f32>()
        .or(Err(XRayError::new_parsing_error(
          "Failed to parse color B value",
        )))?,
    })
  }
}
