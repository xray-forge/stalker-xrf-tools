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
pub struct ParticleActionVortex {
  pub center: Vector3d,
  pub axis: Vector3d,
  pub magnitude: f32,
  pub epsilon: f32,
  pub max_radius: f32,
}

impl ParticleActionReader for ParticleActionVortex {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionVortex> {
    Ok(ParticleActionVortex {
      center: Vector3d::read::<T>(reader)?,
      axis: Vector3d::read::<T>(reader)?,
      magnitude: reader.read_f32::<T>()?,
      epsilon: reader.read_f32::<T>()?,
      max_radius: reader.read_f32::<T>()?,
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
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    self.center.write::<XRayByteOrder>(writer)?;
    self.axis.write::<XRayByteOrder>(writer)?;

    writer.write_f32::<XRayByteOrder>(self.magnitude)?;
    writer.write_f32::<XRayByteOrder>(self.epsilon)?;
    writer.write_f32::<XRayByteOrder>(self.max_radius)?;

    Ok(())
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
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
