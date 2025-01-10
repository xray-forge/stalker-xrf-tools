use crate::data::generic::vector_3d::Vector3d;
use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::error::DatabaseError;
use crate::export::file_import::read_ltx_field;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionTargetSize {
  pub size: Vector3d,
  pub scale: Vector3d,
}

impl ParticleActionReader for ParticleActionTargetSize {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionTargetSize> {
    Ok(ParticleActionTargetSize {
      size: Vector3d::read::<T>(reader)?,
      scale: Vector3d::read::<T>(reader)?,
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
      size: read_ltx_field("size", section)?,
      scale: read_ltx_field("scale", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionTargetSize {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    self.size.write::<XRayByteOrder>(writer)?;
    self.scale.write::<XRayByteOrder>(writer)?;

    Ok(())
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    ltx
      .with_section(section_name)
      .set("size", self.size.to_string())
      .set("scale", self.scale.to_string());

    Ok(())
  }
}
