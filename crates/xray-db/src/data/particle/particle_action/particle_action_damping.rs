use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::particle::particle_action::particle_action_generic::ParticleActionGeneric;
use crate::data::vector_3d::Vector3d;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::Ltx;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionDamping {
  pub damping: Vector3d,
  pub v_low_sqr: f32,
  pub v_high_sqr: f32,
}

impl ParticleActionDamping {
  /// Read particle_action damping.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionDamping> {
    Ok(ParticleActionDamping {
      damping: reader.read_f32_3d_vector::<T>()?,
      v_low_sqr: reader.read_f32::<T>()?,
      v_high_sqr: reader.read_f32::<T>()?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionDamping {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    todo!()
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
