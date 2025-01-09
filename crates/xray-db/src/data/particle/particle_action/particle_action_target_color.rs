use crate::data::generic::vector_3d::Vector3d;
use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ltx_field;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionTargetColor {
  pub color: Vector3d,
  pub alpha: f32,
  pub scale: f32,
  // CS / COP only:
  pub time_from: f32,
  pub time_to: f32,
}

impl ParticleActionReader for ParticleActionTargetColor {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionTargetColor> {
    Ok(ParticleActionTargetColor {
      color: Vector3d::read::<T>(reader)?,
      alpha: reader.read_f32::<T>()?,
      scale: reader.read_f32::<T>()?,
      // CS / COP only:
      time_from: reader.read_f32::<T>()?,
      time_to: reader.read_f32::<T>()?,
    })
  }

  fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "Particle action section '{section_name}' should be defined in ltx file ({})",
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
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionTargetColor {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    self.color.write::<XRayByteOrder>(writer)?;

    writer.write_f32::<XRayByteOrder>(self.alpha)?;
    writer.write_f32::<XRayByteOrder>(self.scale)?;
    writer.write_f32::<XRayByteOrder>(self.time_from)?;
    writer.write_f32::<XRayByteOrder>(self.time_to)?;

    Ok(())
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
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
