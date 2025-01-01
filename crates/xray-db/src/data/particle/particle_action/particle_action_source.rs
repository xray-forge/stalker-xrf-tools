use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::data::particle::particle_domain::ParticleDomain;
use crate::data::vector_3d::Vector3d;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, ParticlesByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionSource {
  pub position: ParticleDomain,
  pub velocity: ParticleDomain,
  pub rot: ParticleDomain,
  pub size: ParticleDomain,
  pub color: ParticleDomain,
  pub alpha: f32,
  pub particle_rate: f32,
  pub age: f32,
  pub age_sigma: f32,
  pub parent_vel: Vector3d,
  pub parent_motion: f32,
}

impl ParticleActionReader for ParticleActionSource {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionSource> {
    Ok(ParticleActionSource {
      position: ParticleDomain::read::<T>(reader)?,
      velocity: ParticleDomain::read::<T>(reader)?,
      rot: ParticleDomain::read::<T>(reader)?,
      size: ParticleDomain::read::<T>(reader)?,
      color: ParticleDomain::read::<T>(reader)?,
      alpha: reader.read_f32::<T>()?,
      particle_rate: reader.read_f32::<T>()?,
      age: reader.read_f32::<T>()?,
      age_sigma: reader.read_f32::<T>()?,
      parent_vel: reader.read_f32_3d_vector::<T>()?,
      parent_motion: reader.read_f32::<T>()?,
    })
  }

  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini
      .section(section_name)
      .unwrap_or_else(|| panic!("Particle action '{section_name}' should be defined in ltx file"));

    Ok(Self {
      position: read_ini_field("position", section)?,
      velocity: read_ini_field("velocity", section)?,
      rot: read_ini_field("rot", section)?,
      size: read_ini_field("size", section)?,
      color: read_ini_field("color", section)?,
      alpha: read_ini_field("alpha", section)?,
      particle_rate: read_ini_field("particle_rate", section)?,
      age: read_ini_field("age", section)?,
      age_sigma: read_ini_field("age_sigma", section)?,
      parent_vel: read_ini_field("parent_vel", section)?,
      parent_motion: read_ini_field("parent_motion", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionSource {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    self.position.write::<ParticlesByteOrder>(writer)?;
    self.velocity.write::<ParticlesByteOrder>(writer)?;
    self.rot.write::<ParticlesByteOrder>(writer)?;
    self.size.write::<ParticlesByteOrder>(writer)?;
    self.color.write::<ParticlesByteOrder>(writer)?;

    writer.write_f32::<ParticlesByteOrder>(self.alpha)?;
    writer.write_f32::<ParticlesByteOrder>(self.particle_rate)?;
    writer.write_f32::<ParticlesByteOrder>(self.age)?;
    writer.write_f32::<ParticlesByteOrder>(self.age_sigma)?;
    writer.write_f32_3d_vector::<ParticlesByteOrder>(&self.parent_vel)?;
    writer.write_f32::<ParticlesByteOrder>(self.parent_motion)?;

    Ok(())
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("position", self.position.to_string())
      .set("velocity", self.velocity.to_string())
      .set("rot", self.rot.to_string())
      .set("size", self.size.to_string())
      .set("color", self.color.to_string())
      .set("alpha", self.alpha.to_string())
      .set("particle_rate", self.particle_rate.to_string())
      .set("age", self.age.to_string())
      .set("age_sigma", self.age_sigma.to_string())
      .set("parent_vel", self.parent_vel.to_string())
      .set("parent_motion", self.parent_motion.to_string());

    Ok(())
  }
}
