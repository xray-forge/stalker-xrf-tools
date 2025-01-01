use crate::chunk::reader::ChunkReader;
use crate::chunk::utils::{
  find_chunk_by_id, read_f32_chunk, read_null_terminated_win_string_chunk, read_u16_chunk,
  read_u32_chunk,
};
use crate::chunk::writer::ChunkWriter;
use crate::constants::META_TYPE_FIELD;
use crate::data::particle::particle_effect_description::ParticleDescription;
use crate::data::particle::particle_group_effect::ParticleGroupEffect;
use crate::data::particle::particle_group_effect_old::ParticleGroupEffectOld;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_ltx::Ltx;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleGroup {
  pub version: u16,
  pub name: String,
  pub flags: u32,
  pub time_limit: f32,
  pub effects: Vec<ParticleGroupEffect>,
  pub description: Option<ParticleDescription>,
  pub effects_old: Option<Vec<ParticleGroupEffectOld>>,
}

impl ParticleGroup {
  pub const META_TYPE: &'static str = "particle_group";

  pub const CHUNK_VERSION: u32 = 1;
  pub const CHUNK_NAME: u32 = 2;
  pub const CHUNK_FLAGS: u32 = 3;
  pub const CHUNK_EFFECTS: u32 = 4;
  pub const CHUNK_TIMELIMIT: u32 = 5;
  pub const CHUNK_DESCRIPTION: u32 = 6;
  pub const CHUNK_EFFECTS2: u32 = 7;

  /// Read group from chunk reader binary data.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let chunks: Vec<ChunkReader> = ChunkReader::read_all_from_file(reader);

    let particle_group = Self {
      version: read_u16_chunk::<T>(
        &mut find_chunk_by_id(&chunks, Self::CHUNK_VERSION)
          .expect("Particle group version chunk not found"),
      )?,
      name: read_null_terminated_win_string_chunk(
        &mut find_chunk_by_id(&chunks, Self::CHUNK_NAME)
          .expect("Particle group name chunk not found"),
      )?,
      flags: read_u32_chunk::<T>(
        &mut find_chunk_by_id(&chunks, Self::CHUNK_FLAGS)
          .expect("Particle group flags chunk not found"),
      )?,
      effects: ParticleGroupEffect::read_list::<T>(
        &mut find_chunk_by_id(&chunks, Self::CHUNK_EFFECTS)
          .expect("Particle group effects chunk not found"),
      )?,
      time_limit: read_f32_chunk::<T>(
        &mut find_chunk_by_id(&chunks, Self::CHUNK_TIMELIMIT)
          .expect("Particle group time limit chunk not found"),
      )?,
      description: find_chunk_by_id(&chunks, Self::CHUNK_DESCRIPTION).map(|mut it| {
        ParticleDescription::read::<T>(&mut it).expect("Invalid description chunk data")
      }),
      effects_old: find_chunk_by_id(&chunks, Self::CHUNK_DESCRIPTION).map(|mut it| {
        ParticleGroupEffectOld::read_list::<T>(&mut it)
          .expect("Invalid old group effects chunk data")
      }),
    };

    assert!(reader.is_ended(), "Expect groups chunk to be ended");
    assert_eq!(
      particle_group.version, 3,
      "Only version 3 of group chunks is supported"
    );

    Ok(particle_group)
  }

  /// Write particle group data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    todo!("Implement")
  }

  /// Import particles group data from provided path.
  pub fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    todo!("Implement");
  }

  /// Export particles group data into provided path.
  pub fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set(META_TYPE_FIELD, Self::META_TYPE)
      .set("version", self.version.to_string())
      .set("name", &self.name)
      .set("flags", self.flags.to_string())
      .set("time_limit", self.time_limit.to_string());

    for (index, effect) in self.effects.iter().enumerate() {
      effect.export(&format!("{section}.effect.{index}"), ini)?;
    }

    if let Some(description) = &self.description {
      description.export(&format!("{section}.description"), ini)?;
    }

    if let Some(effect_old) = &self.effects_old {
      for (index, effect) in effect_old.iter().enumerate() {
        effect.export(&format!("{section}.effect_old.{index}"), ini)?
      }
    }

    Ok(())
  }
}
