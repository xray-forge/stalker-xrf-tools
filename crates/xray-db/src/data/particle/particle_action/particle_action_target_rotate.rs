use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::data::vector_3d::Vector3d;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, ParticlesByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionTargetRotate {
  pub rot: Vector3d,
  pub scale: f32,
}

impl ParticleActionReader for ParticleActionTargetRotate {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionTargetRotate> {
    Ok(ParticleActionTargetRotate {
      rot: reader.read_f32_3d_vector::<T>()?,
      scale: reader.read_f32::<T>()?,
    })
  }

  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "Particle action section '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
      rot: read_ini_field("rot", section)?,
      scale: read_ini_field("scale", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionTargetRotate {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_f32_3d_vector::<ParticlesByteOrder>(&self.rot)?;
    writer.write_f32::<ParticlesByteOrder>(self.scale)?;

    Ok(())
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult {
    ini
      .with_section(section)
      .set("rot", self.rot.to_string())
      .set("scale", self.scale.to_string());

    Ok(())
  }
}
