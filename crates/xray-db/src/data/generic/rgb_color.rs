use crate::error::DatabaseError;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::{Read, Write};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RgbColor {
  pub r: f32,
  pub g: f32,
  pub b: f32,
}

impl RgbColor {
  pub fn new(r: f32, g: f32, b: f32) -> Self {
    Self { r, g, b }
  }

  pub fn read<T: ByteOrder>(reader: &mut dyn Read) -> DatabaseResult<Self> {
    Ok(Self {
      r: reader.read_f32::<T>()?,
      g: reader.read_f32::<T>()?,
      b: reader.read_f32::<T>()?,
    })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut dyn Write) -> DatabaseResult {
    writer.write_f32::<T>(self.r)?;
    writer.write_f32::<T>(self.g)?;
    writer.write_f32::<T>(self.b)?;

    Ok(())
  }
}

impl Display for RgbColor {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(formatter, "{},{},{}", self.r, self.g, self.b)
  }
}

impl FromStr for RgbColor {
  type Err = DatabaseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = s.split(',').collect();

    if parts.len() != 3 {
      return Err(DatabaseError::new_parse_error(
        "Failed to parse rgb color from string, expected 3 numbers",
      ));
    }

    Ok(Self {
      r: parts[0]
        .trim()
        .parse::<f32>()
        .or(Err(DatabaseError::new_parse_error(
          "Failed to parse color R value",
        )))?,
      g: parts[1]
        .trim()
        .parse::<f32>()
        .or(Err(DatabaseError::new_parse_error(
          "Failed to parse color G value",
        )))?,
      b: parts[2]
        .trim()
        .parse::<f32>()
        .or(Err(DatabaseError::new_parse_error(
          "Failed to parse color B value",
        )))?,
    })
  }
}
