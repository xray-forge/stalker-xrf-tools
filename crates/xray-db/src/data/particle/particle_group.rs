use crate::constants::META_TYPE_FIELD;
use crate::data::particle::particle_effect_description::ParticleDescription;
use crate::data::particle::particle_group_effect::ParticleGroupEffect;
use crate::data::particle::particle_group_effect_old::ParticleGroupEffectOld;
use crate::error::DatabaseError;
use crate::export::file_import::read_ltx_field;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{
  find_optional_chunk_by_id, read_f32_chunk, read_null_terminated_win_string_chunk, read_u16_chunk,
  read_u32_chunk, ChunkReader, ChunkWriter,
};
use xray_ltx::{Ltx, Section};

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

  pub const EFFECT_ACTIONS_LIMIT: usize = 10_000;

  pub const VERSION_CHUNK_ID: u32 = 1;
  pub const NAME_CHUNK_ID: u32 = 2;
  pub const FLAGS_CHUNK_ID: u32 = 3;
  pub const EFFECTS_CHUNK_ID: u32 = 4;
  pub const TIME_LIMIT_CHUNK_ID: u32 = 5;
  pub const DESCRIPTION_CHUNK_ID: u32 = 6;
  pub const EFFECTS2_CHUNK_ID: u32 = 7;

  /// Read group from chunk reader binary data.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let chunks: Vec<ChunkReader> = reader.read_children();

    let particle_group: Self = Self {
      version: read_u16_chunk::<T>(
        &mut find_optional_chunk_by_id(&chunks, Self::VERSION_CHUNK_ID)
          .expect("Particle group version chunk not found"),
      )?,
      name: read_null_terminated_win_string_chunk(
        &mut find_optional_chunk_by_id(&chunks, Self::NAME_CHUNK_ID)
          .expect("Particle group name chunk not found"),
      )?,
      flags: read_u32_chunk::<T>(
        &mut find_optional_chunk_by_id(&chunks, Self::FLAGS_CHUNK_ID)
          .expect("Particle group flags chunk not found"),
      )?,
      effects: ParticleGroupEffect::read_list::<T>(
        &mut find_optional_chunk_by_id(&chunks, Self::EFFECTS_CHUNK_ID)
          .expect("Particle group effects chunk not found"),
      )?,
      time_limit: read_f32_chunk::<T>(
        &mut find_optional_chunk_by_id(&chunks, Self::TIME_LIMIT_CHUNK_ID)
          .expect("Particle group time limit chunk not found"),
      )?,
      description: find_optional_chunk_by_id(&chunks, Self::DESCRIPTION_CHUNK_ID).map(|mut it| {
        ParticleDescription::read::<T>(&mut it).expect("Invalid description chunk data")
      }),
      effects_old: find_optional_chunk_by_id(&chunks, Self::EFFECTS2_CHUNK_ID).map(|mut it| {
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
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    let mut version_chunk_writer: ChunkWriter = ChunkWriter::new();
    version_chunk_writer.write_u16::<T>(self.version)?;
    version_chunk_writer.flush_chunk_into::<T>(writer, Self::VERSION_CHUNK_ID)?;

    let mut name_chunk_writer: ChunkWriter = ChunkWriter::new();
    name_chunk_writer.write_null_terminated_win_string(&self.name)?;
    name_chunk_writer.flush_chunk_into::<T>(writer, Self::NAME_CHUNK_ID)?;

    let mut flags_chunk_writer: ChunkWriter = ChunkWriter::new();
    flags_chunk_writer.write_u32::<T>(self.flags)?;
    flags_chunk_writer.flush_chunk_into::<T>(writer, Self::FLAGS_CHUNK_ID)?;

    let mut effects_chunk_writer: ChunkWriter = ChunkWriter::new();
    ParticleGroupEffect::write_list::<T>(&self.effects, &mut effects_chunk_writer)?;
    effects_chunk_writer.flush_chunk_into::<T>(writer, Self::EFFECTS_CHUNK_ID)?;

    let mut time_limit_chunk_writer: ChunkWriter = ChunkWriter::new();
    time_limit_chunk_writer.write_f32::<T>(self.time_limit)?;
    time_limit_chunk_writer.flush_chunk_into::<T>(writer, Self::TIME_LIMIT_CHUNK_ID)?;

    if let Some(description) = &self.description {
      let mut description_chunk_writer: ChunkWriter = ChunkWriter::new();
      description.write::<T>(&mut description_chunk_writer)?;
      description_chunk_writer.flush_chunk_into::<T>(writer, Self::DESCRIPTION_CHUNK_ID)?;
    }

    if let Some(effects_old) = &self.effects_old {
      if !effects_old.is_empty() {
        let mut effects_old_chunk_writer: ChunkWriter = ChunkWriter::new();
        ParticleGroupEffectOld::write_list::<T>(effects_old, &mut effects_old_chunk_writer)?;
        effects_old_chunk_writer.flush_chunk_into::<T>(writer, Self::DESCRIPTION_CHUNK_ID)?;
      }
    }

    Ok(())
  }

  /// Import particles group data from provided path.
  pub fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      DatabaseError::new_parse_error(format!(
        "Particle group section '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    let meta_type: String = read_ltx_field(META_TYPE_FIELD, section)?;

    assert_eq!(
      meta_type,
      Self::META_TYPE,
      "Expected corrected meta type field for '{}' importing",
      Self::META_TYPE
    );

    let effects_old: Vec<ParticleGroupEffectOld> =
      ParticleGroupEffectOld::import_list(section_name, ltx)?;

    Ok(Self {
      version: read_ltx_field("version", section)?,
      name: read_ltx_field("name", section)?,
      flags: read_ltx_field("flags", section)?,
      time_limit: read_ltx_field("time_limit", section)?,
      effects: ParticleGroupEffect::import_list(section_name, ltx)?,
      description: ParticleDescription::import_optional(
        &Self::get_description_section(section_name),
        ltx,
      )?,
      effects_old: if effects_old.is_empty() {
        None
      } else {
        Some(effects_old)
      },
    })
  }

  /// Export particles group data into provided path.
  pub fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    ltx
      .with_section(section_name)
      .set(META_TYPE_FIELD, Self::META_TYPE)
      .set("version", self.version.to_string())
      .set("name", &self.name)
      .set("flags", self.flags.to_string())
      .set("time_limit", self.time_limit.to_string());

    ParticleGroupEffect::export_list(&self.effects, section_name, ltx)?;

    if let Some(description) = &self.description {
      description.export(&Self::get_description_section(section_name), ltx)?;
    }

    if let Some(effects_old) = &self.effects_old {
      ParticleGroupEffectOld::export_list(effects_old, section_name, ltx)?;
    }

    Ok(())
  }
}

impl ParticleGroup {
  fn get_description_section(section_name: &str) -> String {
    format!("{section_name}.description")
  }
}
