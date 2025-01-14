use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::error::DatabaseError;
use crate::export::file_import::read_ltx_field;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionSpeedLimit {
  pub min_speed: f32,
  pub max_speed: f32,
}

impl ParticleActionReader for ParticleActionSpeedLimit {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionSpeedLimit> {
    Ok(ParticleActionSpeedLimit {
      min_speed: reader.read_f32::<T>()?,
      max_speed: reader.read_f32::<T>()?,
    })
  }

  fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      DatabaseError::new_parse_error(format!(
        "Particle action section '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
      min_speed: read_ltx_field("min_speed", section)?,
      max_speed: read_ltx_field("max_speed", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionSpeedLimit {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_f32::<XRayByteOrder>(self.min_speed)?;
    writer.write_f32::<XRayByteOrder>(self.max_speed)?;

    Ok(())
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    ltx
      .with_section(section_name)
      .set("min_speed", self.min_speed.to_string())
      .set("max_speed", self.max_speed.to_string());

    Ok(())
  }
}
