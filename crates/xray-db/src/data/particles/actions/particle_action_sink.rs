use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::data::particles::particle_domain::ParticleDomain;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionSink {
  pub kill_inside: u32,
  pub position: ParticleDomain,
}

impl ParticleActionReader for ParticleActionSink {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<ParticleActionSink> {
    Ok(ParticleActionSink {
      kill_inside: reader.read_u32::<T>()?,
      position: reader.read_xr::<T, _>()?,
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
      kill_inside: read_ltx_field("kill_inside", section)?,
      position: read_ltx_field("position", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionSink {
  fn write(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_u32::<XRayByteOrder>(self.kill_inside)?;
    writer.write_xr::<XRayByteOrder, _>(&self.position)?;

    Ok(())
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("kill_inside", self.kill_inside.to_string())
      .set("position", self.position.to_string());

    Ok(())
  }
}
