use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::particle::particle_action::particle_action_generic::ParticleActionGeneric;
use crate::data::particle::particle_action::particle_action_reader::ParticleActionReader;
use crate::data::vector_3d::Vector3d;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, ParticlesByteOrder};
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionTargetSize {
  pub size: Vector3d,
  pub scale: Vector3d,
}

impl ParticleActionReader for ParticleActionTargetSize {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionTargetSize> {
    Ok(ParticleActionTargetSize {
      size: reader.read_f32_3d_vector::<T>()?,
      scale: reader.read_f32_3d_vector::<T>()?,
    })
  }

  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini
      .section(section_name)
      .unwrap_or_else(|| panic!("Particle action '{section_name}' should be defined in ltx file"));

    Ok(Self {
      size: read_ini_field("size", section)?,
      scale: read_ini_field("scale", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionTargetSize {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    writer.write_f32_3d_vector::<ParticlesByteOrder>(&self.size)?;
    writer.write_f32_3d_vector::<ParticlesByteOrder>(&self.scale)?;

    Ok(())
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("size", self.size.to_string())
      .set("scale", self.scale.to_string());

    Ok(())
  }
}
