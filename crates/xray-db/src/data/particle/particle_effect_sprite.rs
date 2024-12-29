use crate::chunk::reader::ChunkReader;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleEffectSprite {
  shader_name: String,
  texture_name: String,
}

impl ParticleEffectSprite {
  /// Read effect sprite data from chunk redder.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<ParticleEffectSprite> {
    let particle_sprite: ParticleEffectSprite = ParticleEffectSprite {
      shader_name: reader.read_null_terminated_win_string()?,
      texture_name: reader.read_null_terminated_win_string()?,
    };

    assert!(
      reader.is_ended(),
      "Expect particle effect sprite chunk to be ended"
    );

    Ok(particle_sprite)
  }
}
