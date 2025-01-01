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

#[derive(Debug, Serialize, Deserialize)]
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
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionExplosion> {
    Ok(ParticleActionExplosion {
      center: ParticleDomain::read::<T>(reader)?,
      velocity: reader.read_f32::<T>()?,
      magnitude: reader.read_f32::<T>()?,
      st_dev: reader.read_f32::<T>()?,
      age: reader.read_f32::<T>()?,
      epsilon: reader.read_f32::<T>()?,
    })
  }

  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini.section("header").ok_or_else(|| {
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
