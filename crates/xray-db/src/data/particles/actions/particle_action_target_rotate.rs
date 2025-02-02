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
pub struct ParticleActionTargetRotate {
  pub rot: Vector3d,
  pub scale: f32,
}

impl ChunkReadWrite for ParticleActionTargetRotate {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<ParticleActionTargetRotate> {
    Ok(Self {
      rot: reader.read_xr::<T, _>()?,
      scale: reader.read_f32::<T>()?,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_xr::<T, _>(&self.rot)?;
    writer.write_f32::<T>(self.scale)?;

    Ok(())
  }
}

impl LtxImportExport for ParticleActionTargetRotate {
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle action section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      rot: read_ltx_field("rot", section)?,
      scale: read_ltx_field("scale", section)?,
    })
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("rot", self.rot.to_string())
      .set("scale", self.scale.to_string());

    Ok(())
  }
}
