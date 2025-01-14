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
pub struct ParticleActionTurbulence {
  pub frequency: f32,
  pub octaves: i32,
  pub magnitude: f32,
  pub epsilon: f32,
  pub offset: Vector3d,
}

impl ParticleActionReader for ParticleActionTurbulence {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionTurbulence> {
    Ok(ParticleActionTurbulence {
      frequency: reader.read_f32::<T>()?,
      octaves: reader.read_i32::<T>()?,
      magnitude: reader.read_f32::<T>()?,
      epsilon: reader.read_f32::<T>()?,
      offset: Vector3d::read::<T>(reader)?,
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
      frequency: read_ltx_field("frequency", section)?,
      octaves: read_ltx_field("octaves", section)?,
      magnitude: read_ltx_field("magnitude", section)?,
      epsilon: read_ltx_field("epsilon", section)?,
      offset: read_ltx_field("offset", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionTurbulence {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_f32::<XRayByteOrder>(self.frequency)?;
    writer.write_i32::<XRayByteOrder>(self.octaves)?;
    writer.write_f32::<XRayByteOrder>(self.magnitude)?;
    writer.write_f32::<XRayByteOrder>(self.epsilon)?;

    self.offset.write::<XRayByteOrder>(writer)?;

    Ok(())
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    ltx
      .with_section(section_name)
      .set("frequency", self.frequency.to_string())
      .set("octaves", self.octaves.to_string())
      .set("magnitude", self.magnitude.to_string())
      .set("epsilon", self.epsilon.to_string())
      .set("offset", self.offset.to_string());

    Ok(())
  }
}
