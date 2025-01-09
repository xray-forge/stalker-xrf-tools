use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::data::particle::particle_domain::ParticleDomain;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ltx_field;
use crate::types::{DatabaseResult, ParticlesByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};
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

  fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "Particle action section '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
      kill_inside: read_ltx_field("kill_inside", section)?,
      velocity: read_ltx_field("velocity", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionSinkVelocity {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_u32::<ParticlesByteOrder>(self.kill_inside)?;

    self.velocity.write::<ParticlesByteOrder>(writer)?;

    Ok(())
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    ltx
      .with_section(section_name)
      .set("kill_inside", self.kill_inside.to_string())
      .set("velocity", self.velocity.to_string());

    Ok(())
  }
}
