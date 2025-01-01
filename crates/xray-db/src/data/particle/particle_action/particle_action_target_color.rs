use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::particle::particle_action::particle_action_generic::ParticleActionGeneric;
use crate::data::particle::particle_action::particle_action_reader::ParticleActionReader;
use crate::data::vector_3d::Vector3d;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, ParticlesByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
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
      color: reader.read_f32_3d_vector::<T>()?,
      alpha: reader.read_f32::<T>()?,
      scale: reader.read_f32::<T>()?,
      // CS / COP only:
      time_from: reader.read_f32::<T>()?,
      time_to: reader.read_f32::<T>()?,
    })
  }

  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini
      .section(section_name)
      .unwrap_or_else(|| panic!("Particle action '{section_name}' should be defined in ltx file"));

    Ok(Self {
      color: read_ini_field("color", section)?,
      alpha: read_ini_field("alpha", section)?,
      scale: read_ini_field("scale", section)?,
      time_from: read_ini_field("time_from", section)?,
      time_to: read_ini_field("time_to", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionTargetColor {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    writer.write_f32_3d_vector::<ParticlesByteOrder>(&self.color)?;
    writer.write_f32::<ParticlesByteOrder>(self.alpha)?;
    writer.write_f32::<ParticlesByteOrder>(self.scale)?;
    writer.write_f32::<ParticlesByteOrder>(self.time_from)?;
    writer.write_f32::<ParticlesByteOrder>(self.time_to)?;

    Ok(())
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("color", self.color.to_string())
      .set("alpha", self.alpha.to_string())
      .set("scale", self.scale.to_string())
      .set("time_from", self.time_from.to_string())
      .set("time_to", self.time_to.to_string());

    Ok(())
  }
}
