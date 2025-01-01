use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::data::particle::particle_domain::ParticleDomain;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, ParticlesByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionExplosion {
  pub center: ParticleDomain,
  pub velocity: f32,
  pub magnitude: f32,
  pub st_dev: f32,
  pub age: f32,
  pub epsilon: f32,
}

impl ParticleActionReader for ParticleActionExplosion {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      center: ParticleDomain::read::<T>(reader)?,
      velocity: reader.read_f32::<T>()?,
      magnitude: reader.read_f32::<T>()?,
      st_dev: reader.read_f32::<T>()?,
      age: reader.read_f32::<T>()?,
      epsilon: reader.read_f32::<T>()?,
    })
  }

  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "Particle action section '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
      center: read_ini_field("center", section)?,
      velocity: read_ini_field("velocity", section)?,
      magnitude: read_ini_field("magnitude", section)?,
      st_dev: read_ini_field("st_dev", section)?,
      age: read_ini_field("age", section)?,
      epsilon: read_ini_field("epsilon", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionExplosion {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    self.center.write::<ParticlesByteOrder>(writer)?;

    writer.write_f32::<ParticlesByteOrder>(self.velocity)?;
    writer.write_f32::<ParticlesByteOrder>(self.magnitude)?;
    writer.write_f32::<ParticlesByteOrder>(self.st_dev)?;
    writer.write_f32::<ParticlesByteOrder>(self.age)?;
    writer.write_f32::<ParticlesByteOrder>(self.epsilon)?;

    Ok(())
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("center", self.center.to_string())
      .set("velocity", self.velocity.to_string())
      .set("magnitude", self.magnitude.to_string())
      .set("st_dev", self.st_dev.to_string())
      .set("age", self.age.to_string())
      .set("epsilon", self.epsilon.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::meta::particle_action_reader::ParticleActionReader;
  use crate::data::meta::particle_action_writer::ParticleActionWriter;
  use crate::data::particle::particle_action::particle_action_explosion::ParticleActionExplosion;
  use crate::data::particle::particle_domain::ParticleDomain;
  use crate::data::vector_3d::Vector3d;
  use crate::export::file::open_ini_config;
  use crate::types::{DatabaseResult, SpawnByteOrder};
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use xray_ltx::Ltx;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_resource_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: ParticleActionExplosion = ParticleActionExplosion {
      center: ParticleDomain {
        domain_type: 5,
        coordinates: (
          Vector3d {
            x: -10.36,
            y: -20.85,
            z: -30.56,
          },
          Vector3d {
            x: 12.5,
            y: 23.6,
            z: 34.7,
          },
        ),
        basis: (
          Vector3d {
            x: -20.58,
            y: -30.66,
            z: -40.75,
          },
          Vector3d {
            x: 6.53,
            y: 7.63,
            z: 8.75,
          },
        ),
        radius1: 15.0,
        radius2: 25.0,
        radius1_sqr: 10.0,
        radius2_sqr: 5.0,
      },
      velocity: 36.3,
      magnitude: 20.0,
      st_dev: 0.2,
      age: 430.0,
      epsilon: 0.0001,
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 88);

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 88);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 88 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      ParticleActionExplosion::read::<SpawnByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> DatabaseResult<()> {
    let ltx_filename: String = get_relative_test_sample_file_path(file!(), "import_export.ini");
    let mut ltx: Ltx = Ltx::new();

    let original: ParticleActionExplosion = ParticleActionExplosion {
      center: ParticleDomain {
        domain_type: 0,
        coordinates: (
          Vector3d {
            x: -3.36,
            y: -3.85,
            z: -3.56,
          },
          Vector3d {
            x: 12.5,
            y: 12.6,
            z: 12.7,
          },
        ),
        basis: (
          Vector3d {
            x: -24.58,
            y: -24.66,
            z: -24.75,
          },
          Vector3d {
            x: 25.53,
            y: 25.63,
            z: 25.75,
          },
        ),
        radius1: 11.0,
        radius2: 23.4,
        radius1_sqr: 12.2,
        radius2_sqr: 6.1,
      },
      velocity: 30.5,
      magnitude: 1.0,
      st_dev: 0.14,
      age: 427.0,
      epsilon: 0.00001,
    };

    original.export("data", &mut ltx)?;

    ltx.write_to(&mut overwrite_test_relative_resource_as_file(
      &ltx_filename,
    )?)?;

    let source: Ltx = open_ini_config(&get_absolute_test_resource_path(&ltx_filename))?;

    assert_eq!(ParticleActionExplosion::import("data", &source)?, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> DatabaseResult<()> {
    let original: ParticleActionExplosion = ParticleActionExplosion {
      center: ParticleDomain {
        domain_type: 0,
        coordinates: (
          Vector3d {
            x: -4.36,
            y: -4.85,
            z: -4.56,
          },
          Vector3d {
            x: 11.5,
            y: 11.6,
            z: 11.7,
          },
        ),
        basis: (
          Vector3d {
            x: -23.58,
            y: -23.66,
            z: -23.75,
          },
          Vector3d {
            x: 26.53,
            y: 26.63,
            z: 26.75,
          },
        ),
        radius1: 13.0,
        radius2: 26.4,
        radius1_sqr: 13.2,
        radius2_sqr: 5.1,
      },
      velocity: 36.422,
      magnitude: 1.2,
      st_dev: 0.12,
      age: 544.0,
      epsilon: 0.00001,
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
      serde_json::from_str::<ParticleActionExplosion>(&serialized).unwrap()
    );

    Ok(())
  }
}
