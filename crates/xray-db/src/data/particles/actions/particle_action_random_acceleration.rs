use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::data::particles::particle_domain::ParticleDomain;
use crate::export::file_import::read_ltx_field;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionRandomAcceleration {
  pub gen_acc: ParticleDomain,
}

impl ParticleActionReader for ParticleActionRandomAcceleration {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<ParticleActionRandomAcceleration> {
    Ok(ParticleActionRandomAcceleration {
      gen_acc: ParticleDomain::read::<T>(reader)?,
    })
  }

  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle action section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      gen_acc: read_ltx_field("gen_acc", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionRandomAcceleration {
  fn write(&self, writer: &mut ChunkWriter) -> XRayResult {
    self.gen_acc.write::<XRayByteOrder>(writer)?;

    Ok(())
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("gen_acc", self.gen_acc.to_string());

    Ok(())
  }
}
