use crate::chunk::reader::ChunkReader;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleGroupEffect {
  pub name: String,
  pub on_play_child_name: String,
  pub on_birth_child_name: String,
  pub on_dead_child_name: String,
  pub time_0: f32,
  pub time_1: f32,
  pub flags: u32,
}

impl ParticleGroupEffect {
  /// Read list of effect groups data from chunk reader.
  pub fn read_list<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<Vec<ParticleGroupEffect>> {
    let mut effects: Vec<ParticleGroupEffect> = Vec::new();

    let count: u32 = reader.read_u32::<T>()?;

    for _ in 0..count {
      effects.push(ParticleGroupEffect::read::<T>(reader)?);
    }

    assert_eq!(
      effects.len(),
      count as usize,
      "Should read same count of effects as declared in chunk"
    );

    assert!(
      reader.is_ended(),
      "Expect particle effects list chunk to be ended"
    );

    Ok(effects)
  }

  /// Read group effect from chunk reader binary data.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<ParticleGroupEffect> {
    let particle_group = ParticleGroupEffect {
      name: reader.read_null_terminated_win_string()?,
      on_play_child_name: reader.read_null_terminated_win_string()?,
      on_birth_child_name: reader.read_null_terminated_win_string()?,
      on_dead_child_name: reader.read_null_terminated_win_string()?,
      time_0: reader.read_f32::<T>()?,
      time_1: reader.read_f32::<T>()?,
      flags: reader.read_u32::<T>()?,
    };

    Ok(particle_group)
  }
}
