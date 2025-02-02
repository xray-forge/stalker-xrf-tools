use crate::data::generic::vector_3d::Vector3d;
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
pub struct ParticleActionTargetVelocity {
  pub velocity: Vector3d,
  pub scale: f32,
}

impl ParticleActionReader for ParticleActionTargetVelocity {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<ParticleActionTargetVelocity> {
    Ok(ParticleActionTargetVelocity {
      velocity: reader.read_xr::<T, _>()?,
      scale: reader.read_f32::<T>()?,
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
      velocity: read_ltx_field("velocity", section)?,
      scale: read_ltx_field("scale", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionTargetVelocity {
  fn write(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_xr::<XRayByteOrder, _>(&self.velocity)?;
    writer.write_f32::<XRayByteOrder>(self.scale)?;

    Ok(())
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("velocity", self.velocity.to_string())
      .set("scale", self.scale.to_string());

    Ok(())
  }
}
