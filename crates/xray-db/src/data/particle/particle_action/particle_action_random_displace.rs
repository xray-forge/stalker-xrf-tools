use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::particle::particle_action::particle_action_generic::ParticleActionGeneric;
use crate::data::particle::particle_action::particle_action_reader::ParticleActionReader;
use crate::data::particle::particle_domain::ParticleDomain;
use crate::export::file_import::read_ini_field;
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

  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini
      .section(section_name)
      .unwrap_or_else(|| panic!("Particle action '{section_name}' should be defined in ltx file"));

    Ok(Self {
      gen_disp: read_ini_field("gen_disp", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionRandomDisplace {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    self.gen_disp.write::<ParticlesByteOrder>(writer)?;

    Ok(())
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("gen_disp", self.gen_disp.to_string());

    Ok(())
  }
}
