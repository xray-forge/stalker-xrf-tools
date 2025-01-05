use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::data::particle::particle_domain::ParticleDomain;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, ParticlesByteOrder};
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionRandomAcceleration {
  pub gen_acc: ParticleDomain,
}

impl ParticleActionReader for ParticleActionRandomAcceleration {
  fn read<T: ByteOrder>(
    reader: &mut ChunkReader,
  ) -> DatabaseResult<ParticleActionRandomAcceleration> {
    Ok(ParticleActionRandomAcceleration {
      gen_acc: ParticleDomain::read::<T>(reader)?,
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
      gen_acc: read_ini_field("gen_acc", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionRandomAcceleration {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    self.gen_acc.write::<ParticlesByteOrder>(writer)?;

    Ok(())
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult {
    ini
      .with_section(section)
      .set("gen_acc", self.gen_acc.to_string());

    Ok(())
  }
}
