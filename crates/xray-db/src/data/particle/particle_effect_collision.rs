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
pub struct ParticleEffectCollision {
  pub collide_one_minus_friction: f32,
  pub collide_resilience: f32,
  pub collide_sqr_cutoff: f32,
}

impl ParticleEffectCollision {
  pub const META_TYPE: &'static str = "particle_effect_collision";

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

  /// Write particle effect collision data into chunk writer.
  pub fn write<T: ByteOrder>(self: &Self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    todo!("Implement");
    Ok(())
  }

  /// Import particle effect collision data from provided path.
  pub fn import(path: &Path) -> DatabaseResult<ParticleAction> {
    todo!("Implement");
  }

  /// Export particle effect collision data into provided path.
  pub fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("$type", Self::META_TYPE)
      .set(
        "collide_one_minus_friction",
        self.collide_one_minus_friction.to_string(),
      )
      .set("collide_resilience", self.collide_resilience.to_string())
      .set("collide_sqr_cutoff", self.collide_sqr_cutoff.to_string());

    Ok(())
  }
}
