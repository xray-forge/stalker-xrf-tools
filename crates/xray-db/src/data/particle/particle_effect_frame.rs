use crate::chunk::reader::ChunkReader;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleEffectFrame {
  texture_size: (f32, f32),
  reserved: (f32, f32),
  frame_dimension_x: u32,
  frame_count: u32,
  frame_speed: f32,
}

impl ParticleEffectFrame {
  /// Read particle effect frame data from chunk redder.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<ParticleEffectFrame> {
    let particle_frame: ParticleEffectFrame = ParticleEffectFrame {
      texture_size: (reader.read_f32::<T>()?, reader.read_f32::<T>()?),
      reserved: (reader.read_f32::<T>()?, reader.read_f32::<T>()?),
      frame_dimension_x: reader.read_u32::<T>()?,
      frame_count: reader.read_u32::<T>()?,
      frame_speed: reader.read_f32::<T>()?,
    };

    assert!(reader.is_ended(), "Expect particle frame chunk to be ended");

    Ok(particle_frame)
  }
}
