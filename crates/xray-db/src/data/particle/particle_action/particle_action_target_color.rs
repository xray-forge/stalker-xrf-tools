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
pub struct ParticleActionTargetColor {
  pub color: Vector3d,
  pub alpha: f32,
  pub scale: f32,
  // CS / COP only:
  pub time_from: f32,
  pub time_to: f32,
}

impl ParticleActionTargetColor {
  /// Read particle_action target color.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionTargetColor> {
    Ok(ParticleActionTargetColor {
      color: reader.read_f32_3d_vector::<T>()?,
      alpha: reader.read_f32::<T>()?,
      scale: reader.read_f32::<T>()?,
      // CS / COP only:
      time_from: reader.read_f32::<T>()?,
      time_to: reader.read_f32::<T>()?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionTargetColor {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    todo!()
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
