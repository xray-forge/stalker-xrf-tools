use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::particle::particle_action::particle_action_generic::ParticleActionGeneric;
use crate::data::particle::particle_action::particle_action_type::ParticleActionType;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt};
use serde::{Deserialize, Serialize};
use std::path::Path;
use xray_ltx::Ltx;

/// C++ src/xrParticles/particle_actions_collection_io.cpp
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleAction {
  pub action_flags: u32,
  pub action_type: u32,
  pub data: Box<dyn ParticleActionGeneric>,
}

impl ParticleAction {
  pub const META_TYPE: &'static str = "particle_action";

  /// Read list of effect particle particle_action data from chunk reader.
  pub fn read_list<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Vec<ParticleAction>> {
    let mut actions: Vec<ParticleAction> = Vec::new();

    let count: u32 = reader.read_u32::<T>()?;

    for _ in 0..count {
      actions.push(ParticleAction::read::<T>(reader)?);
    }

    assert_eq!(
      actions.len(),
      count as usize,
      "Should read same count of action as declared in chunk"
    );

    assert!(
      reader.is_ended(),
      "Expect particle actions list chunk to be ended"
    );

    Ok(actions)
  }

  /// Read effect particle particle_action data from chunk reader.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleAction> {
    let action_type: u32 = reader.read_u32::<T>()?;

    let particle_action: ParticleAction = ParticleAction {
      action_flags: reader.read_u32::<T>()?,
      action_type: reader.read_u32::<T>()?,
      data: ParticleActionType::read_by_particle_type::<T>(
        reader,
        ParticleActionType::from_u32(action_type),
      )?,
    };

    Ok(particle_action)
  }

  /// Write particle effect data into chunk writer.
  pub fn write<T: ByteOrder>(self: &Self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    todo!("Implement");
    Ok(())
  }

  /// Import particle effect data from provided path.
  pub fn import(path: &Path) -> DatabaseResult<ParticleAction> {
    todo!("Implement");
  }

  /// Export particle effect data into provided path.
  pub fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("$type", Self::META_TYPE)
      .set("action_flags", self.action_flags.to_string())
      .set("action_type", self.action_type.to_string());

    self.data.export(section, ini)?;

    Ok(())
  }
}
