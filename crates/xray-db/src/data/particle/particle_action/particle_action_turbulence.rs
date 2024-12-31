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
pub struct ParticleActionTurbulence {
  pub frequency: f32,
  pub octaves: i32,
  pub magnitude: f32,
  pub epsilon: f32,
  pub offset: Vector3d,
}

impl ParticleActionTurbulence {
  /// Read particle_action turbulence.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionTurbulence> {
    Ok(ParticleActionTurbulence {
      frequency: reader.read_f32::<T>()?,
      octaves: reader.read_i32::<T>()?,
      magnitude: reader.read_f32::<T>()?,
      epsilon: reader.read_f32::<T>()?,
      offset: reader.read_f32_3d_vector::<T>()?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionTurbulence {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    todo!()
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("frequency", self.frequency.to_string())
      .set("octaves", self.octaves.to_string())
      .set("magnitude", self.magnitude.to_string())
      .set("epsilon", self.epsilon.to_string())
      .set("offset", self.offset.to_string());

    Ok(())
  }
}
