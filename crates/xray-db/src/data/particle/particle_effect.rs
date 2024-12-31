use crate::chunk::reader::ChunkReader;
use crate::chunk::utils::{
  find_chunk_by_id, read_f32_chunk, read_f32_vector_chunk, read_null_terminated_win_string_chunk,
  read_till_end_binary_chunk, read_u16_chunk, read_u32_chunk,
};
use crate::chunk::writer::ChunkWriter;
use crate::data::particle::particle_action::particle_action::ParticleAction;
use crate::data::particle::particle_effect_collision::ParticleEffectCollision;
use crate::data::particle::particle_effect_description::ParticleDescription;
use crate::data::particle::particle_effect_frame::ParticleEffectFrame;
use crate::data::particle::particle_effect_sprite::ParticleEffectSprite;
use crate::data::particle::particle_group::ParticleGroup;
use crate::data::vector_3d::Vector3d;
use crate::export::string::bytes_to_base64;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::path::Path;
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
  pub editor_data: Option<Vec<u8>>,
}

impl ParticleEffect {
  pub const META_TYPE: &'static str = "particle_effect";

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
  pub fn read<T: ByteOrder>(mut reader: ChunkReader) -> DatabaseResult<ParticleEffect> {
    let chunks: Vec<ChunkReader> = ChunkReader::read_all_from_file(&mut reader);

    let effect: ParticleEffect = {
      ParticleEffect {
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
        editor_data: find_chunk_by_id(&chunks, Self::EDITOR_DATA_CHUNK_ID)
          .map(|mut it| read_till_end_binary_chunk(&mut it).expect("Invalid editor chunk data")),
      }
    };

    assert!(
      reader.is_ended(),
      "Expect particle effect chunk to be ended"
    );

    Ok(effect)
  }

  /// Write particle effect data into chunk writer.
  pub fn write<T: ByteOrder>(self: &Self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    todo!("Implement")
  }

  /// Import particle effect data from provided path.
  pub fn import(path: &Path) -> DatabaseResult<ParticleGroup> {
    todo!("Implement");
  }

  /// Export particle effect data into provided path.
  pub fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("$type", Self::META_TYPE)
      .set("version", self.version.to_string())
      .set("name", &self.name)
      .set("actions_count", self.actions.len().to_string())
      .set("max_particles", self.max_particles.to_string())
      .set("flags", self.flags.to_string());

    self.sprite.export(&format!("{section}.sprite"), ini)?;

    let ini_section: &mut Section = ini
      .section_mut(section)
      .expect("Unexpected missing section after write operation");

    if let Some(time_limit) = &self.time_limit {
      ini_section.insert("time_limit", time_limit.to_string());
    }

    if let Some(velocity_scale) = &self.velocity_scale {
      ini_section.insert("velocity_scale", velocity_scale.to_string());
    }

    if let Some(rotation) = &self.rotation {
      ini_section.insert("rotation", rotation.to_string());
    }

    for (index, action) in self.actions.iter().enumerate() {
      action.export(&format!("{section}.action.{index}"), ini)?
    }

    if let Some(frame) = &self.frame {
      frame.export(&format!("{section}.frame"), ini)?;
    }

    if let Some(collision) = &self.collision {
      collision.export(&format!("{section}.collision"), ini)?;
    }

    if let Some(description) = &self.description {
      description.export(&format!("{section}.description"), ini)?;
    }

    if let Some(editor_data) = &self.editor_data {
      ini
        .with_section(format!("{section}.editor_data"))
        .set("value", bytes_to_base64(&editor_data));
    }

    Ok(())
  }
}
