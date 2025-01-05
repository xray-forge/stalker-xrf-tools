use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::data::particle::particle_domain::ParticleDomain;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ltx_field;
use crate::types::{DatabaseResult, ParticlesByteOrder};
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionRandomDisplace {
  pub gen_disp: ParticleDomain,
}

impl ParticleActionReader for ParticleActionRandomDisplace {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionRandomDisplace> {
    Ok(ParticleActionRandomDisplace {
      gen_disp: ParticleDomain::read::<T>(reader)?,
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
      gen_disp: read_ltx_field("gen_disp", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionRandomDisplace {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    self.gen_disp.write::<ParticlesByteOrder>(writer)?;

    Ok(())
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    ltx
      .with_section(section_name)
      .set("gen_disp", self.gen_disp.to_string());

    Ok(())
  }
}
