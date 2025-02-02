use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::data::particles::particle_domain::ParticleDomain;
use crate::file_import::read_ltx_field;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionRandomVelocity {
  pub gen_vel: ParticleDomain,
}

impl ParticleActionReader for ParticleActionRandomVelocity {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      gen_vel: reader.read_xr::<T, _>()?,
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
      gen_vel: read_ltx_field("gen_vel", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionRandomVelocity {
  fn write(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_xr::<XRayByteOrder, _>(&self.gen_vel)?;

    Ok(())
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("gen_vel", self.gen_vel.to_string());

    Ok(())
  }
}
