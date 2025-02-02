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
pub struct ParticleActionVortex {
  pub center: Vector3d,
  pub axis: Vector3d,
  pub magnitude: f32,
  pub epsilon: f32,
  pub max_radius: f32,
}

impl ParticleActionReader for ParticleActionVortex {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<ParticleActionVortex> {
    Ok(ParticleActionVortex {
      center: reader.read_xr::<T, _>()?,
      axis: reader.read_xr::<T, _>()?,
      magnitude: reader.read_f32::<T>()?,
      epsilon: reader.read_f32::<T>()?,
      max_radius: reader.read_f32::<T>()?,
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
      center: read_ltx_field("center", section)?,
      axis: read_ltx_field("axis", section)?,
      magnitude: read_ltx_field("magnitude", section)?,
      epsilon: read_ltx_field("epsilon", section)?,
      max_radius: read_ltx_field("max_radius", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionVortex {
  fn write(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_xr::<XRayByteOrder, _>(&self.center)?;
    writer.write_xr::<XRayByteOrder, _>(&self.axis)?;
    writer.write_f32::<XRayByteOrder>(self.magnitude)?;
    writer.write_f32::<XRayByteOrder>(self.epsilon)?;
    writer.write_f32::<XRayByteOrder>(self.max_radius)?;

    Ok(())
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("center", self.center.to_string())
      .set("axis", self.axis.to_string())
      .set("magnitude", self.magnitude.to_string())
      .set("epsilon", self.epsilon.to_string())
      .set("max_radius", self.max_radius.to_string());

    Ok(())
  }
}
