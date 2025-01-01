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
pub struct ParticleActionDamping {
  pub damping: Vector3d,
  pub v_low_sqr: f32,
  pub v_high_sqr: f32,
}

impl ParticleActionReader for ParticleActionDamping {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionDamping> {
    Ok(ParticleActionDamping {
      damping: reader.read_f32_3d_vector::<T>()?,
      v_low_sqr: reader.read_f32::<T>()?,
      v_high_sqr: reader.read_f32::<T>()?,
    })
  }

  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini
      .section(section_name)
      .unwrap_or_else(|| panic!("Particle action '{section_name}' should be defined in ltx file"));

    Ok(Self {
      damping: read_ini_field("damping", section)?,
      v_low_sqr: read_ini_field("v_low_sqr", section)?,
      v_high_sqr: read_ini_field("v_high_sqr", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionDamping {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    writer.write_f32_3d_vector::<ParticlesByteOrder>(&self.damping)?;
    writer.write_f32::<ParticlesByteOrder>(self.v_low_sqr)?;
    writer.write_f32::<ParticlesByteOrder>(self.v_high_sqr)?;

    Ok(())
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("damping", self.damping.to_string())
      .set("v_low_sqr", self.v_low_sqr.to_string())
      .set("v_high_sqr", self.v_high_sqr.to_string());

    Ok(())
  }
}
