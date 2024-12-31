use crate::chunk::reader::ChunkReader;
use crate::data::vector_3d::Vector3d;
use crate::error::database_error::DatabaseError;
use crate::error::database_parse_error::DatabaseParseError;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleDomain {
  pub domain_type: u32,
  pub coordinates: (Vector3d, Vector3d),
  pub basis: (Vector3d, Vector3d),
  pub radius1: f32,
  pub radius2: f32,
  pub radius1_sqr: f32,
  pub radius2_sqr: f32,
}

impl ParticleDomain {
  /// Read particle domain from chunk reader.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      domain_type: reader.read_u32::<T>()?,
      coordinates: (
        reader.read_f32_3d_vector::<T>()?,
        reader.read_f32_3d_vector::<T>()?,
      ),
      basis: (
        reader.read_f32_3d_vector::<T>()?,
        reader.read_f32_3d_vector::<T>()?,
      ),
      radius1: reader.read_f32::<T>()?,
      radius2: reader.read_f32::<T>()?,
      radius1_sqr: reader.read_f32::<T>()?,
      radius2_sqr: reader.read_f32::<T>()?,
    })
  }
}

impl Display for ParticleDomain {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      formatter,
      "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
      self.domain_type,
      self.coordinates.0.x,
      self.coordinates.0.y,
      self.coordinates.0.z,
      self.coordinates.1.x,
      self.coordinates.1.y,
      self.coordinates.1.z,
      self.basis.0.x,
      self.basis.0.y,
      self.basis.0.z,
      self.basis.1.x,
      self.basis.1.y,
      self.basis.1.z,
      self.radius1,
      self.radius2,
      self.radius1_sqr,
      self.radius2_sqr,
    )
  }
}

impl FromStr for ParticleDomain {
  type Err = DatabaseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = s.split(',').collect();

    if parts.len() != 17 {
      return Err(DatabaseParseError::new_database_error(
        "Failed to parse particle domain from string, expected 17 numbers",
      ));
    }

    Ok(Self {
      domain_type: parts[0].trim().parse::<u32>().or(Err(
        DatabaseParseError::new_database_error("Failed to parse vector domain_type value"),
      ))?,
      coordinates: (
        Vector3d {
          x: parts[1]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseParseError::new_database_error(
              "Failed to parse coordinates 0 vector x value",
            )))?,
          y: parts[2]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseParseError::new_database_error(
              "Failed to parse coordinates 0 vector y value",
            )))?,
          z: parts[3]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseParseError::new_database_error(
              "Failed to parse coordinates 0 vector z value",
            )))?,
        },
        Vector3d {
          x: parts[4]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseParseError::new_database_error(
              "Failed to parse coordinates 1 vector x value",
            )))?,
          y: parts[5]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseParseError::new_database_error(
              "Failed to parse coordinates 1 vector y value",
            )))?,
          z: parts[6]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseParseError::new_database_error(
              "Failed to parse coordinates 1 vector z value",
            )))?,
        },
      ),
      basis: (
        Vector3d {
          x: parts[7]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseParseError::new_database_error(
              "Failed to parse basis 0 vector x value",
            )))?,
          y: parts[8]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseParseError::new_database_error(
              "Failed to parse basis 0 vector y value",
            )))?,
          z: parts[9]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseParseError::new_database_error(
              "Failed to parse basis 0 vector z value",
            )))?,
        },
        Vector3d {
          x: parts[10]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseParseError::new_database_error(
              "Failed to parse basis 1 vector x value",
            )))?,
          y: parts[11]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseParseError::new_database_error(
              "Failed to parse basis 1 vector y value",
            )))?,
          z: parts[12]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseParseError::new_database_error(
              "Failed to parse basis 1 vector z value",
            )))?,
        },
      ),
      radius1: parts[13]
        .trim()
        .parse::<f32>()
        .or(Err(DatabaseParseError::new_database_error(
          "Failed to parse vector radius1 value",
        )))?,
      radius2: parts[14]
        .trim()
        .parse::<f32>()
        .or(Err(DatabaseParseError::new_database_error(
          "Failed to parse vector radius2 value",
        )))?,
      radius1_sqr: parts[15].trim().parse::<f32>().or(Err(
        DatabaseParseError::new_database_error("Failed to parse vector radius1_sqr value"),
      ))?,
      radius2_sqr: parts[16].trim().parse::<f32>().or(Err(
        DatabaseParseError::new_database_error("Failed to parse vector radius2_sqr value"),
      ))?,
    })
  }
}
