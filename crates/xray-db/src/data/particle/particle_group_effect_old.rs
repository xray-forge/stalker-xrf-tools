use crate::chunk::reader::ChunkReader;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleGroupEffectOld {
  pub name: String,
  pub on_play_child_name: String,
  pub time_0: f32,
  pub time_1: f32,
  pub flags: u32,
}

impl ParticleGroupEffectOld {
  /// Read list of old effect groups data from chunk reader.
  pub fn read_list<T: ByteOrder>(
    reader: &mut ChunkReader,
  ) -> io::Result<Vec<ParticleGroupEffectOld>> {
    let mut effects: Vec<ParticleGroupEffectOld> = Vec::new();

    let count: u32 = reader.read_u32::<T>()?;

    for _ in 0..count {
      effects.push(ParticleGroupEffectOld::read::<T>(reader)?);
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

  /// Read old group effect from chunk reader binary data.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<ParticleGroupEffectOld> {
    let particle_group = ParticleGroupEffectOld {
      name: reader.read_null_terminated_win_string()?,
      on_play_child_name: reader.read_null_terminated_win_string()?,
      time_0: reader.read_f32::<T>()?,
      time_1: reader.read_f32::<T>()?,
      flags: reader.read_u32::<T>()?,
    };

    Ok(particle_group)
  }
}
