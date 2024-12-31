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
pub struct ParticleActionTargetVelocity {
  pub velocity: Vector3d,
  pub scale: f32,
}

impl ParticleActionTargetVelocity {
  /// Read particle_action target velocity.
  pub fn read<T: ByteOrder>(
    reader: &mut ChunkReader,
  ) -> DatabaseResult<ParticleActionTargetVelocity> {
    Ok(ParticleActionTargetVelocity {
      velocity: reader.read_f32_3d_vector::<T>()?,
      scale: reader.read_f32::<T>()?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionTargetVelocity {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    todo!()
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("velocity", self.velocity.to_string())
      .set("scale", self.scale.to_string());

    Ok(())
  }
}
