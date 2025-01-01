use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::particle::particle_action::particle_action_generic::ParticleActionGeneric;
use crate::data::particle::particle_action::particle_action_reader::ParticleActionReader;
use crate::data::particle::particle_domain::ParticleDomain;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, ParticlesByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionAvoid {
  pub position: ParticleDomain,
  pub look_ahead: f32,
  pub magnitude: f32,
  pub epsilon: f32,
}

impl ParticleActionReader for ParticleActionAvoid {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      position: ParticleDomain::read::<T>(reader)?,
      look_ahead: reader.read_f32::<T>()?,
      magnitude: reader.read_f32::<T>()?,
      epsilon: reader.read_f32::<T>()?,
    })
  }

  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini
      .section(section_name)
      .unwrap_or_else(|| panic!("Particle action '{section_name}' should be defined in ltx file"));

    Ok(Self {
      position: read_ini_field("position", section)?,
      look_ahead: read_ini_field("look_ahead", section)?,
      magnitude: read_ini_field("magnitude", section)?,
      epsilon: read_ini_field("epsilon", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionAvoid {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    self.position.write::<ParticlesByteOrder>(writer)?;

    writer.write_f32::<ParticlesByteOrder>(self.look_ahead)?;
    writer.write_f32::<ParticlesByteOrder>(self.magnitude)?;
    writer.write_f32::<ParticlesByteOrder>(self.epsilon)?;

    Ok(())
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("position", self.position.to_string())
      .set("look_ahead", self.look_ahead.to_string())
      .set("magnitude", self.magnitude.to_string())
      .set("epsilon", self.epsilon.to_string());

    Ok(())
  }
}
