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
pub struct ParticleActionSink {
  pub kill_inside: u32,
  pub position: ParticleDomain,
}

impl ParticleActionReader for ParticleActionSink {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionSink> {
    Ok(ParticleActionSink {
      kill_inside: reader.read_u32::<T>()?,
      position: ParticleDomain::read::<T>(reader)?,
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
      kill_inside: read_ini_field("kill_inside", section)?,
      position: read_ini_field("position", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionSink {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_u32::<ParticlesByteOrder>(self.kill_inside)?;

    self.position.write::<ParticlesByteOrder>(writer)?;

    Ok(())
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult {
    ini
      .with_section(section)
      .set("kill_inside", self.kill_inside.to_string())
      .set("position", self.position.to_string());

    Ok(())
  }
}
