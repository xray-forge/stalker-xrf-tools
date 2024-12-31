use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::particle::particle_action::particle_action::ParticleAction;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::path::Path;
use xray_ltx::Ltx;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleDescription {
  pub creator: String,
  pub editor: String,
  pub created_time: u32,
  pub edit_time: u32,
}

impl ParticleDescription {
  pub const META_TYPE: &'static str = "particle_description";

  /// Read particle effect description data from chunk redder.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let particle_description: Self = Self {
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

  /// Write particle effect description data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    todo!("Implement");
    Ok(())
  }

  /// Import particle effect description data from provided path.
  pub fn import(path: &Path) -> DatabaseResult<ParticleAction> {
    todo!("Implement");
  }

  /// Export particle effect description data into provided path.
  pub fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    todo!("Implement");

    Ok(())
  }
}
