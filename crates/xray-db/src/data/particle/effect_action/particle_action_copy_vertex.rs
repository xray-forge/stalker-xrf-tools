use crate::chunk::reader::ChunkReader;
use crate::data::particle::effect_action::particle_action_generic::ParticleActionGeneric;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionCopyVertex {
  pub copy_pos: u32,
}

impl ParticleActionCopyVertex {
  /// Read effect_action copy vertex.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleActionCopyVertex> {
    // No data here.
    Ok(ParticleActionCopyVertex {
      copy_pos: reader.read_u32::<T>()?,
    })
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionCopyVertex {}
