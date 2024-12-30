use crate::chunk::reader::ChunkReader;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleDescription {
  pub creator: String,
  pub editor: String,
  pub created_time: u32,
  pub edit_time: u32,
}

impl ParticleDescription {
  /// Read particle effect description data from chunk redder.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<ParticleDescription> {
    let particle_description: ParticleDescription = ParticleDescription {
      creator: reader.read_null_terminated_win_string()?,
      editor: reader.read_null_terminated_win_string()?,
      created_time: reader.read_u32::<T>()?,
      edit_time: reader.read_u32::<T>()?,
    };

    assert!(
      reader.is_ended(),
      "Expect particle description chunk to be ended"
    );

    Ok(particle_description)
  }
}
