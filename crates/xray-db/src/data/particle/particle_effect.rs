use crate::chunk::reader::ChunkReader;
use crate::chunk::utils::{
  find_chunk_by_id, read_f32_chunk, read_f32_vector_chunk, read_null_terminated_win_string_chunk,
  read_u16_chunk, read_u32_chunk,
};
use crate::chunk::writer::ChunkWriter;
use crate::constants::META_TYPE_FIELD;
use crate::data::particle::particle_action::particle_action::ParticleAction;
use crate::data::particle::particle_effect_collision::ParticleEffectCollision;
use crate::data::particle::particle_effect_description::ParticleDescription;
use crate::data::particle::particle_effect_editor_data::ParticleEffectEditorData;
use crate::data::particle::particle_effect_frame::ParticleEffectFrame;
use crate::data::particle::particle_effect_sprite::ParticleEffectSprite;
use crate::data::vector_3d::Vector3d;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::{read_ini_field, read_ini_optional_field};
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

/// C++ src/Layers/xrRender/ParticleEffectDef.cpp
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleEffect {
  pub version: u16,
  pub name: String,
  pub max_particles: u32,
  pub actions: Vec<ParticleAction>,
  pub flags: u32,
  pub frame: Option<ParticleEffectFrame>,
  pub sprite: ParticleEffectSprite,
  pub time_limit: Option<f32>,
  pub collision: Option<ParticleEffectCollision>,
  pub velocity_scale: Option<Vector3d>,
  pub description: Option<ParticleDescription>,
  pub rotation: Option<Vector3d>,
  pub editor_data: Option<ParticleEffectEditorData>,
}

impl ParticleEffect {
  pub const META_TYPE: &'static str = "particle_effect";

  pub const EFFECT_ACTIONS_LIMIT: usize = 10_000;

  pub const VERSION_CHUNK_ID: u32 = 1;
  pub const NAME_CHUNK_ID: u32 = 2;
  pub const MAX_PARTICLES_CHUNK_ID: u32 = 3;
  pub const ACTION_LIST_CHUNK_ID: u32 = 4;
  pub const FLAGS_CHUNK_ID: u32 = 5;
  pub const FRAME_CHUNK_ID: u32 = 6;
  pub const SPRITE_CHUNK_ID: u32 = 7;
  pub const TIME_LIMIT_OLD_CHUNK_ID: u32 = 8;
  pub const TIME_LIMIT_CHUNK_ID: u32 = 9;
  pub const SOURCE_TEXT_CHUNK_ID: u32 = 32;
  pub const COLLISION_CHUNK_ID: u32 = 33;
  pub const VELOCITY_SCALE_CHUNK_ID: u32 = 34;
  pub const DESCRIPTION_CHUNK_ID: u32 = 35;
  pub const EDITOR_DATA_CHUNK_ID: u32 = 36;
  pub const ROTATION_CHUNK_ID: u32 = 37;

  /// Read effects by position descriptor.
  /// Parses binary data into version chunk representation object.
  pub fn read<T: ByteOrder>(mut reader: ChunkReader) -> DatabaseResult<Self> {
    let chunks: Vec<ChunkReader> = ChunkReader::read_all_from_file(&mut reader);

    let effect: Self = {
      Self {
        version: read_u16_chunk::<T>(
          &mut find_chunk_by_id(&chunks, Self::VERSION_CHUNK_ID)
            .expect("Particle name chunk not found"),
        )?,
        name: read_null_terminated_win_string_chunk(
          &mut find_chunk_by_id(&chunks, Self::NAME_CHUNK_ID)
            .expect("Particle name chunk not found"),
        )?,
        max_particles: read_u32_chunk::<T>(
          &mut find_chunk_by_id(&chunks, Self::MAX_PARTICLES_CHUNK_ID)
            .expect("Particle max particles chunk not found"),
        )?,
        actions: ParticleAction::read_list::<T>(
          &mut find_chunk_by_id(&chunks, Self::ACTION_LIST_CHUNK_ID)
            .expect("Particle effect actions chunk not found"),
        )?,
        flags: read_u32_chunk::<T>(
          &mut find_chunk_by_id(&chunks, Self::MAX_PARTICLES_CHUNK_ID)
            .expect("Particle flags chunk not found"),
        )?,
        frame: find_chunk_by_id(&chunks, Self::FRAME_CHUNK_ID)
          .map(|mut it| ParticleEffectFrame::read::<T>(&mut it).expect("Invalid frame chunk data")),
        sprite: ParticleEffectSprite::read::<T>(
          &mut find_chunk_by_id(&chunks, Self::SPRITE_CHUNK_ID)
            .expect("Particle frame sprite chunk not found"),
        )?,
        time_limit: find_chunk_by_id(&chunks, Self::TIME_LIMIT_CHUNK_ID)
          .map(|mut it| read_f32_chunk::<T>(&mut it).expect("Invalid frame time limit chunk data")),
        collision: find_chunk_by_id(&chunks, Self::COLLISION_CHUNK_ID).map(|mut it| {
          ParticleEffectCollision::read::<T>(&mut it).expect("Invalid collision chunk data")
        }),
        velocity_scale: find_chunk_by_id(&chunks, Self::VELOCITY_SCALE_CHUNK_ID).map(|mut it| {
          read_f32_vector_chunk::<T>(&mut it).expect("Invalid velocity scale chunk data")
        }),
        description: find_chunk_by_id(&chunks, Self::DESCRIPTION_CHUNK_ID).map(|mut it| {
          ParticleDescription::read::<T>(&mut it).expect("Invalid description chunk data")
        }),
        rotation: find_chunk_by_id(&chunks, Self::ROTATION_CHUNK_ID)
          .map(|mut it| read_f32_vector_chunk::<T>(&mut it).expect("Invalid rotation chunk data")),
        editor_data: find_chunk_by_id(&chunks, Self::EDITOR_DATA_CHUNK_ID).map(|mut it| {
          ParticleEffectEditorData::read::<T>(&mut it).expect("Invalid editor data chunk")
        }),
      }
    };

    assert!(
      reader.is_ended(),
      "Expect particle effect chunk to be ended"
    );

    Ok(effect)
  }

  /// Write particle effect data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    let mut version_chunk_writer: ChunkWriter = ChunkWriter::new();
    version_chunk_writer.write_u16::<T>(self.version)?;
    version_chunk_writer.flush_chunk_into::<T>(writer, Self::VERSION_CHUNK_ID)?;

    let mut name_chunk_writer: ChunkWriter = ChunkWriter::new();
    name_chunk_writer.write_null_terminated_win_string(&self.name)?;
    name_chunk_writer.flush_chunk_into::<T>(writer, Self::NAME_CHUNK_ID)?;

    let mut max_particles_chunk_writer: ChunkWriter = ChunkWriter::new();
    max_particles_chunk_writer.write_u32::<T>(self.max_particles)?;
    max_particles_chunk_writer.flush_chunk_into::<T>(writer, Self::MAX_PARTICLES_CHUNK_ID)?;

    let mut actions_chunk_writer: ChunkWriter = ChunkWriter::new();
    ParticleAction::write_list::<T>(&self.actions, &mut actions_chunk_writer)?;
    actions_chunk_writer.flush_chunk_into::<T>(writer, Self::ACTION_LIST_CHUNK_ID)?;

    let mut flags_chunk_writer: ChunkWriter = ChunkWriter::new();
    flags_chunk_writer.write_u32::<T>(self.flags)?;
    flags_chunk_writer.flush_chunk_into::<T>(writer, Self::FLAGS_CHUNK_ID)?;

    if let Some(frame) = &self.frame {
      let mut frame_chunk_writer: ChunkWriter = ChunkWriter::new();
      frame.write::<T>(&mut frame_chunk_writer)?;
      frame_chunk_writer.flush_chunk_into::<T>(writer, Self::FRAME_CHUNK_ID)?;
    }

    let mut sprite_chunk_writer: ChunkWriter = ChunkWriter::new();
    self.sprite.write::<T>(&mut sprite_chunk_writer)?;
    sprite_chunk_writer.flush_chunk_into::<T>(writer, Self::SPRITE_CHUNK_ID)?;

    if let Some(time_limit) = self.time_limit {
      let mut time_limit_chunk_writer: ChunkWriter = ChunkWriter::new();
      time_limit_chunk_writer.write_f32::<T>(time_limit)?;
      time_limit_chunk_writer.flush_chunk_into::<T>(writer, Self::TIME_LIMIT_CHUNK_ID)?;
    }

    if let Some(collision) = &self.collision {
      let mut collision_chunk_writer: ChunkWriter = ChunkWriter::new();
      collision.write::<T>(&mut collision_chunk_writer)?;
      collision_chunk_writer.flush_chunk_into::<T>(writer, Self::COLLISION_CHUNK_ID)?;
    }

    if let Some(velocity_scale) = &self.velocity_scale {
      let mut velocity_scale_chunk_writer: ChunkWriter = ChunkWriter::new();
      velocity_scale_chunk_writer.write_f32_3d_vector::<T>(velocity_scale)?;
      velocity_scale_chunk_writer.flush_chunk_into::<T>(writer, Self::VELOCITY_SCALE_CHUNK_ID)?;
    }

    if let Some(description) = &self.description {
      let mut description_chunk_writer: ChunkWriter = ChunkWriter::new();
      description.write::<T>(&mut description_chunk_writer)?;
      description_chunk_writer.flush_chunk_into::<T>(writer, Self::DESCRIPTION_CHUNK_ID)?;
    }

    if let Some(rotation) = &self.rotation {
      let mut rotation_chunk_writer: ChunkWriter = ChunkWriter::new();
      rotation_chunk_writer.write_f32_3d_vector::<T>(rotation)?;
      rotation_chunk_writer.flush_chunk_into::<T>(writer, Self::ROTATION_CHUNK_ID)?;
    }

    if let Some(editor_data) = &self.editor_data {
      let mut editor_data_chunk_writer: ChunkWriter = ChunkWriter::new();
      editor_data.write::<T>(&mut editor_data_chunk_writer)?;
      editor_data_chunk_writer.flush_chunk_into::<T>(writer, Self::EDITOR_DATA_CHUNK_ID)?;
    }

    Ok(())
  }

  /// Import particle effect data from provided path.
  pub fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "Particle effect section '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    let meta_type: String = read_ini_field(META_TYPE_FIELD, section)?;

    assert_eq!(
      meta_type,
      Self::META_TYPE,
      "Expected corrected meta type field for '{}' importing",
      Self::META_TYPE
    );

    let mut action_index: usize = 0;
    let mut actions: Vec<ParticleAction> = Vec::new();

    loop {
      let action_section_name: String = Self::get_action_section(section_name, action_index);

      if ini.has_section(&action_section_name) {
        actions.push(ParticleAction::import(section_name, ini)?);
        action_index += 1
      } else {
        break;
      }

      if action_index >= Self::EFFECT_ACTIONS_LIMIT {
        return Err(DatabaseParseError::new_database_error(
          "Failed to parse particle effect - reached maximum nested actions limit",
        ));
      }
    }

    Ok(Self {
      version: read_ini_field("version", section)?,
      name: read_ini_field("name", section)?,
      max_particles: read_ini_field("max_particles", section)?,
      actions,
      flags: read_ini_field("flags", section)?,
      frame: ParticleEffectFrame::import_optional(&Self::get_frame_section(section_name), ini)?,
      sprite: ParticleEffectSprite::import(&Self::get_sprite_section(section_name), ini)?,
      time_limit: read_ini_optional_field("time_limit", section)?,
      collision: ParticleEffectCollision::import_optional(
        &Self::get_collision_section(section_name),
        ini,
      )?,
      velocity_scale: read_ini_optional_field("velocity_scale", section)?,
      description: ParticleDescription::import_optional(
        &Self::get_description_section(section_name),
        ini,
      )?,
      rotation: read_ini_optional_field("rotation", section)?,
      editor_data: ParticleEffectEditorData::import_optional(
        &Self::get_editor_data_section(section_name),
        ini,
      )?,
    })
  }

  /// Export particle effect data into provided path.
  pub fn export(&self, section_name: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section_name)
      .set(META_TYPE_FIELD, Self::META_TYPE)
      .set("version", self.version.to_string())
      .set("name", &self.name)
      .set("actions_count", self.actions.len().to_string())
      .set("max_particles", self.max_particles.to_string())
      .set("flags", self.flags.to_string())
      .set_optional("time_limit", self.time_limit.map(|it| it.to_string()))
      .set_optional("rotation", self.rotation.as_ref().map(|it| it.to_string()))
      .set_optional(
        "velocity_scale",
        self.velocity_scale.as_ref().map(|it| it.to_string()),
      );

    self
      .sprite
      .export(&Self::get_sprite_section(section_name), ini)?;

    for (index, action) in self.actions.iter().enumerate() {
      action.export(&Self::get_action_section(section_name, index), ini)?
    }

    ParticleEffectFrame::export_optional(
      self.frame.as_ref(),
      &Self::get_frame_section(section_name),
      ini,
    )?;
    ParticleEffectCollision::export_optional(
      self.collision.as_ref(),
      &Self::get_collision_section(section_name),
      ini,
    )?;
    ParticleDescription::export_optional(
      self.description.as_ref(),
      &Self::get_description_section(section_name),
      ini,
    )?;
    ParticleEffectEditorData::export_optional(
      self.editor_data.as_ref(),
      &Self::get_editor_data_section(section_name),
      ini,
    )?;

    Ok(())
  }
}

impl ParticleEffect {
  fn get_action_section(section_name: &str, index: usize) -> String {
    format!("{section_name}.action.{index}")
  }

  fn get_sprite_section(section_name: &str) -> String {
    format!("{section_name}.sprite")
  }

  fn get_frame_section(section_name: &str) -> String {
    format!("{section_name}.frame")
  }

  fn get_collision_section(section_name: &str) -> String {
    format!("{section_name}.collision")
  }

  fn get_description_section(section_name: &str) -> String {
    format!("{section_name}.description")
  }

  fn get_editor_data_section(section_name: &str) -> String {
    format!("{section_name}.editor_data")
  }
}
