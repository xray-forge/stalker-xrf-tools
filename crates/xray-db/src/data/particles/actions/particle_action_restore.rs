use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionRestore {
  pub time_left: f32,
}

impl ParticleActionReader for ParticleActionRestore {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<ParticleActionRestore> {
    Ok(ParticleActionRestore {
      time_left: reader.read_f32::<T>()?,
    })
  }

  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle action section '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
      time_left: read_ltx_field("time_left", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionRestore {
  fn write(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_f32::<XRayByteOrder>(self.time_left)?;

    Ok(())
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("time_left", self.time_left.to_string());

    Ok(())
  }
}
