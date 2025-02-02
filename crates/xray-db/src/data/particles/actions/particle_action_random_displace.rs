use crate::data::particles::particle_domain::ParticleDomain;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionRandomDisplace {
  pub gen_disp: ParticleDomain,
}

impl ChunkReadWrite for ParticleActionRandomDisplace {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<ParticleActionRandomDisplace> {
    Ok(Self {
      gen_disp: reader.read_xr::<T, _>()?,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_xr::<T, _>(&self.gen_disp)?;

    Ok(())
  }
}

impl LtxImportExport for ParticleActionRandomDisplace {
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle action section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      gen_disp: read_ltx_field("gen_disp", section)?,
    })
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("gen_disp", self.gen_disp.to_string());

    Ok(())
  }
}
