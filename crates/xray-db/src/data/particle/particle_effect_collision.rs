use crate::chunk::reader::ChunkReader;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleEffectCollision {
  pub collide_one_minus_friction: f32,
  pub collide_resilience: f32,
  pub collide_sqr_cutoff: f32,
}

impl ParticleEffectCollision {
  /// Read particle effect collision data from chunk reader.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleEffectCollision> {
    let particle_collision: ParticleEffectCollision = ParticleEffectCollision {
      collide_one_minus_friction: reader.read_f32::<T>()?,
      collide_resilience: reader.read_f32::<T>()?,
      collide_sqr_cutoff: reader.read_f32::<T>()?,
    };

    assert!(
      reader.is_ended(),
      "Expect particle collision chunk to be ended"
    );

    Ok(particle_collision)
  }
}
