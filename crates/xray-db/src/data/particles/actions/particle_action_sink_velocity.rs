use crate::data::particles::particle_domain::ParticleDomain;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionSinkVelocity {
  pub kill_inside: u32,
  pub velocity: ParticleDomain,
}

impl ChunkReadWrite for ParticleActionSinkVelocity {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<ParticleActionSinkVelocity> {
    Ok(Self {
      kill_inside: reader.read_u32::<T>()?,
      velocity: reader.read_xr::<T, _>()?,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_u32::<T>(self.kill_inside)?;
    writer.write_xr::<T, _>(&self.velocity)?;

    Ok(())
  }
}

impl LtxImportExport for ParticleActionSinkVelocity {
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle action section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      kill_inside: read_ltx_field("kill_inside", section)?,
      velocity: read_ltx_field("velocity", section)?,
    })
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("kill_inside", self.kill_inside.to_string())
      .set("velocity", self.velocity.to_string());

    Ok(())
  }
}
