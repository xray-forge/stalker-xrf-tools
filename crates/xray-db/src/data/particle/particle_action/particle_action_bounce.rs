use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::data::particle::particle_domain::ParticleDomain;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, ParticlesByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionBounce {
  pub position: ParticleDomain,
  pub one_minus_friction: f32,
  pub resilience: f32,
  pub cutoff_sqr: f32,
}

impl ParticleActionReader for ParticleActionBounce {
  /// Read particle_action bounce.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionBounce> {
    Ok(ParticleActionBounce {
      position: ParticleDomain::read::<T>(reader)?,
      one_minus_friction: reader.read_f32::<T>()?,
      resilience: reader.read_f32::<T>()?,
      cutoff_sqr: reader.read_f32::<T>()?,
    })
  }

  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini
      .section(section_name)
      .unwrap_or_else(|| panic!("Particle action '{section_name}' should be defined in ltx file"));

    Ok(Self {
      position: read_ini_field("position", section)?,
      one_minus_friction: read_ini_field("one_minus_friction", section)?,
      resilience: read_ini_field("resilience", section)?,
      cutoff_sqr: read_ini_field("cutoff_sqr", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionBounce {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    self.position.write::<ParticlesByteOrder>(writer)?;

    writer.write_f32::<ParticlesByteOrder>(self.one_minus_friction)?;
    writer.write_f32::<ParticlesByteOrder>(self.resilience)?;
    writer.write_f32::<ParticlesByteOrder>(self.cutoff_sqr)?;

    Ok(())
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("position", self.position.to_string())
      .set("one_minus_friction", self.one_minus_friction.to_string())
      .set("resilience", self.resilience.to_string())
      .set("cutoff_sqr", self.cutoff_sqr.to_string());

    Ok(())
  }
}
