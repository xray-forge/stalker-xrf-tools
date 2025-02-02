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
pub struct ParticleActionTargetColor {
  pub color: Vector3d,
  pub alpha: f32,
  pub scale: f32,
  // CS / COP only:
  pub time_from: f32,
  pub time_to: f32,
}

impl ChunkReadWrite for ParticleActionTargetColor {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<ParticleActionTargetColor> {
    Ok(Self {
      color: reader.read_xr::<T, _>()?,
      alpha: reader.read_f32::<T>()?,
      scale: reader.read_f32::<T>()?,
      // CS / COP only:
      time_from: reader.read_f32::<T>()?,
      time_to: reader.read_f32::<T>()?,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_xr::<T, _>(&self.color)?;
    writer.write_f32::<T>(self.alpha)?;
    writer.write_f32::<T>(self.scale)?;
    writer.write_f32::<T>(self.time_from)?;
    writer.write_f32::<T>(self.time_to)?;

    Ok(())
  }
}

impl LtxImportExport for ParticleActionTargetColor {
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle action section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      color: read_ltx_field("color", section)?,
      alpha: read_ltx_field("alpha", section)?,
      scale: read_ltx_field("scale", section)?,
      time_from: read_ltx_field("time_from", section)?,
      time_to: read_ltx_field("time_to", section)?,
    })
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("color", self.color.to_string())
      .set("alpha", self.alpha.to_string())
      .set("scale", self.scale.to_string())
      .set("time_from", self.time_from.to_string())
      .set("time_to", self.time_to.to_string());

    Ok(())
  }
}
