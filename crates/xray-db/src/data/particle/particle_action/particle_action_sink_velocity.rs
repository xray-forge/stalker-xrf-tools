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
pub struct ParticleActionSinkVelocity {
  pub kill_inside: u32,
  pub velocity: ParticleDomain,
}

impl ParticleActionReader for ParticleActionSinkVelocity {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionSinkVelocity> {
    Ok(ParticleActionSinkVelocity {
      kill_inside: reader.read_u32::<T>()?,
      velocity: ParticleDomain::read::<T>(reader)?,
    })
  }

  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini
      .section(section_name)
      .unwrap_or_else(|| panic!("Particle action '{section_name}' should be defined in ltx file"));

    Ok(Self {
      kill_inside: read_ini_field("kill_inside", section)?,
      velocity: read_ini_field("velocity", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionSinkVelocity {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    writer.write_u32::<ParticlesByteOrder>(self.kill_inside)?;

    self.velocity.write::<ParticlesByteOrder>(writer)?;

    Ok(())
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("kill_inside", self.kill_inside.to_string())
      .set("velocity", self.velocity.to_string());

    Ok(())
  }
}
