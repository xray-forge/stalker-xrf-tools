use crate::chunk::reader::ChunkReader;
use crate::data::particle::effect_action::particle_action_generic::ParticleActionGeneric;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionMove {}

impl ParticleActionMove {
  /// Read effect_action move.
  pub fn read<T: ByteOrder>(_: &mut ChunkReader) -> io::Result<ParticleActionMove> {
    // No data.
    Ok(ParticleActionMove {})
  }
}

#[typetag::serde]
impl ParticleActionGeneric for ParticleActionMove {}
