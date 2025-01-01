use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, ParticlesByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionKillOld {
  pub age_limit: f32,
  pub kill_less_than: u32,
}

impl ParticleActionReader for ParticleActionKillOld {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionKillOld> {
    Ok(ParticleActionKillOld {
      age_limit: reader.read_f32::<T>()?,
      kill_less_than: reader.read_u32::<T>()?,
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
      age_limit: read_ini_field("age_limit", section)?,
      kill_less_than: read_ini_field("kill_less_than", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionKillOld {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    writer.write_f32::<ParticlesByteOrder>(self.age_limit)?;
    writer.write_u32::<ParticlesByteOrder>(self.kill_less_than)?;

    Ok(())
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("age_limit", self.age_limit.to_string())
      .set("kill_less_than", self.kill_less_than.to_string());

    Ok(())
  }
}
