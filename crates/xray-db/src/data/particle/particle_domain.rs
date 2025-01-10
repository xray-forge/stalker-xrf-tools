use crate::data::generic::vector_3d::Vector3d;
use crate::error::DatabaseError;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
      coordinates: (Vector3d::read::<T>(reader)?, Vector3d::read::<T>(reader)?),
      basis: (Vector3d::read::<T>(reader)?, Vector3d::read::<T>(reader)?),
      radius1: reader.read_f32::<T>()?,
      radius2: reader.read_f32::<T>()?,
      radius1_sqr: reader.read_f32::<T>()?,
      radius2_sqr: reader.read_f32::<T>()?,
    })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_u32::<XRayByteOrder>(self.domain_type)?;

    self.coordinates.0.write::<T>(writer)?;
    self.coordinates.1.write::<T>(writer)?;
    self.basis.0.write::<T>(writer)?;
    self.basis.1.write::<T>(writer)?;

    writer.write_f32::<XRayByteOrder>(self.radius1)?;
    writer.write_f32::<XRayByteOrder>(self.radius2)?;
    writer.write_f32::<XRayByteOrder>(self.radius1_sqr)?;
    writer.write_f32::<XRayByteOrder>(self.radius2_sqr)?;

    Ok(())
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
      return Err(DatabaseError::new_parse_error(
        "Failed to parse particle domain from string, expected 17 numbers",
      ));
    }

    Ok(Self {
      domain_type: parts[0]
        .trim()
        .parse::<u32>()
        .or(Err(DatabaseError::new_parse_error(
          "Failed to parse vector domain_type value",
        )))?,
      coordinates: (
        Vector3d {
          x: parts[1]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseError::new_parse_error(
              "Failed to parse coordinates 0 vector x value",
            )))?,
          y: parts[2]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseError::new_parse_error(
              "Failed to parse coordinates 0 vector y value",
            )))?,
          z: parts[3]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseError::new_parse_error(
              "Failed to parse coordinates 0 vector z value",
            )))?,
        },
        Vector3d {
          x: parts[4]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseError::new_parse_error(
              "Failed to parse coordinates 1 vector x value",
            )))?,
          y: parts[5]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseError::new_parse_error(
              "Failed to parse coordinates 1 vector y value",
            )))?,
          z: parts[6]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseError::new_parse_error(
              "Failed to parse coordinates 1 vector z value",
            )))?,
        },
      ),
      basis: (
        Vector3d {
          x: parts[7]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseError::new_parse_error(
              "Failed to parse basis 0 vector x value",
            )))?,
          y: parts[8]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseError::new_parse_error(
              "Failed to parse basis 0 vector y value",
            )))?,
          z: parts[9]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseError::new_parse_error(
              "Failed to parse basis 0 vector z value",
            )))?,
        },
        Vector3d {
          x: parts[10]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseError::new_parse_error(
              "Failed to parse basis 1 vector x value",
            )))?,
          y: parts[11]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseError::new_parse_error(
              "Failed to parse basis 1 vector y value",
            )))?,
          z: parts[12]
            .trim()
            .parse::<f32>()
            .or(Err(DatabaseError::new_parse_error(
              "Failed to parse basis 1 vector z value",
            )))?,
        },
      ),
      radius1: parts[13]
        .trim()
        .parse::<f32>()
        .or(Err(DatabaseError::new_parse_error(
          "Failed to parse vector radius1 value",
        )))?,
      radius2: parts[14]
        .trim()
        .parse::<f32>()
        .or(Err(DatabaseError::new_parse_error(
          "Failed to parse vector radius2 value",
        )))?,
      radius1_sqr: parts[15]
        .trim()
        .parse::<f32>()
        .or(Err(DatabaseError::new_parse_error(
          "Failed to parse vector radius1_sqr value",
        )))?,
      radius2_sqr: parts[16]
        .trim()
        .parse::<f32>()
        .or(Err(DatabaseError::new_parse_error(
          "Failed to parse vector radius2_sqr value",
        )))?,
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::data::generic::vector_3d::Vector3d;
  use crate::data::particle::particle_domain::ParticleDomain;
  use crate::types::DatabaseResult;
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use std::str::FromStr;
  use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult {
    let filename: String = String::from("read_write.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let original: ParticleDomain = ParticleDomain {
      domain_type: 6,
      coordinates: (
        Vector3d {
          x: 10.36,
          y: 20.85,
          z: 30.56,
        },
        Vector3d {
          x: -12.5,
          y: -23.6,
          z: -34.7,
        },
      ),
      basis: (
        Vector3d {
          x: 20.58,
          y: 30.66,
          z: 40.75,
        },
        Vector3d {
          x: -6.53,
          y: -7.63,
          z: -8.75,
        },
      ),
      radius1: 11.0,
      radius2: 21.5,
      radius1_sqr: 1.0,
      radius2_sqr: 2.0,
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 68);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&get_relative_test_sample_file_path(
        file!(),
        &filename,
      ))?,
      0,
    )?;

    assert_eq!(bytes_written, 68);

    let file: FileSlice =
      open_test_resource_as_slice(&get_relative_test_sample_file_path(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 68 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    assert_eq!(
      ParticleDomain::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_from_to_str() -> DatabaseResult {
    let original: ParticleDomain = ParticleDomain {
      domain_type: 23,
      coordinates: (
        Vector3d {
          x: 120.36,
          y: 220.85,
          z: 320.56,
        },
        Vector3d {
          x: -132.5,
          y: -233.6,
          z: -334.7,
        },
      ),
      basis: (
        Vector3d {
          x: 240.58,
          y: 340.66,
          z: 440.75,
        },
        Vector3d {
          x: -65.53,
          y: -75.63,
          z: -85.75,
        },
      ),
      radius1: 25.0,
      radius2: 46.5,
      radius1_sqr: 2.0,
      radius2_sqr: 4.0,
    };

    assert_eq!(original.to_string(), "23,120.36,220.85,320.56,-132.5,-233.6,-334.7,240.58,340.66,440.75,-65.53,-75.63,-85.75,25,46.5,2,4");
    assert_eq!(ParticleDomain::from_str("23,120.36,220.85,320.56,-132.5,-233.6,-334.7,240.58,340.66,440.75,-65.53,-75.63,-85.75,25,46.5,2,4")?, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> DatabaseResult {
    let original: ParticleDomain = ParticleDomain {
      domain_type: 52,
      coordinates: (
        Vector3d {
          x: 410.36,
          y: 420.85,
          z: 430.56,
        },
        Vector3d {
          x: -512.5,
          y: -523.6,
          z: -534.7,
        },
      ),
      basis: (
        Vector3d {
          x: 420.58,
          y: 430.66,
          z: 440.75,
        },
        Vector3d {
          x: -56.53,
          y: -57.63,
          z: -58.75,
        },
      ),
      radius1: 546.0,
      radius2: 632.5,
      radius1_sqr: 21.0,
      radius2_sqr: 25.0,
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(json!(original).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      original,
      serde_json::from_str::<ParticleDomain>(&serialized).unwrap()
    );

    Ok(())
  }
}
