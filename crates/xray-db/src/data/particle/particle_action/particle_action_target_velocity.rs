use crate::data::generic::vector_3d::Vector3d;
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
pub struct ParticleActionTargetVelocity {
  pub velocity: Vector3d,
  pub scale: f32,
}

impl ParticleActionReader for ParticleActionTargetVelocity {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionTargetVelocity> {
    Ok(ParticleActionTargetVelocity {
      velocity: Vector3d::read::<T>(reader)?,
      scale: reader.read_f32::<T>()?,
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
      velocity: read_ltx_field("velocity", section)?,
      scale: read_ltx_field("scale", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionTargetVelocity {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    self.velocity.write::<XRayByteOrder>(writer)?;
    writer.write_f32::<XRayByteOrder>(self.scale)?;

    Ok(())
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    ltx
      .with_section(section_name)
      .set("velocity", self.velocity.to_string())
      .set("scale", self.scale.to_string());

    Ok(())
  }
}
