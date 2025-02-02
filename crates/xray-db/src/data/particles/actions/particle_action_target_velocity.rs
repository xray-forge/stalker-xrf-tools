use crate::data::generic::vector_3d::Vector3d;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionTargetVelocity {
  pub velocity: Vector3d,
  pub scale: f32,
}

impl ChunkReadWrite for ParticleActionTargetVelocity {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<ParticleActionTargetVelocity> {
    Ok(Self {
      velocity: reader.read_xr::<T, _>()?,
      scale: reader.read_f32::<T>()?,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_xr::<T, _>(&self.velocity)?;
    writer.write_f32::<T>(self.scale)?;

    Ok(())
  }
}

impl LtxImportExport for ParticleActionTargetVelocity {
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

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("velocity", self.velocity.to_string())
      .set("scale", self.scale.to_string());

    Ok(())
  }
}
