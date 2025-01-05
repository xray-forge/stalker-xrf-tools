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
use crate::export::file_import::{read_ini_optional_field, read_ltx_field};
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
  pub const TIME_LIMIT_CHUNK_ID: u32 = 8;
  pub const TIME_LIMIT_OLD_CHUNK_ID: u32 = 9;
  pub const SOURCE_TEXT_CHUNK_ID: u32 = 32;
  pub const COLLISION_CHUNK_ID: u32 = 33;
  pub const VELOCITY_SCALE_CHUNK_ID: u32 = 34;
  pub const DESCRIPTION_CHUNK_ID: u32 = 35;
  pub const EDITOR_DATA_CHUNK_ID: u32 = 36;
  pub const ROTATION_CHUNK_ID: u32 = 37;

  /// Read effects by position descriptor.
  /// Parses binary data into version chunk representation object.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let chunks: Vec<ChunkReader> = ChunkReader::read_all_from_file(reader);

    let effect: Self = {
      Self {
        version: read_u16_chunk::<T>(
          &mut find_chunk_by_id(&chunks, Self::VERSION_CHUNK_ID)
            .expect("Particle name chunk not found"),
        )
        .map_err(|error| {
          DatabaseParseError::new_database_error(format!(
            "Failed to read particle version chunk: {}",
            error
          ))
        })?,
        name: read_null_terminated_win_string_chunk(
          &mut find_chunk_by_id(&chunks, Self::NAME_CHUNK_ID)
            .expect("Particle name chunk not found"),
        )
        .map_err(|error| {
          DatabaseParseError::new_database_error(format!(
            "Failed to read particle name chunk: {}",
            error
          ))
        })?,
        max_particles: read_u32_chunk::<T>(
          &mut find_chunk_by_id(&chunks, Self::MAX_PARTICLES_CHUNK_ID)
            .expect("Particle max particles chunk not found"),
        )
        .map_err(|error| {
          DatabaseParseError::new_database_error(format!(
            "Failed to read particle max_particles chunk: {}",
            error
          ))
        })?,
        actions: ParticleAction::read_list::<T>(
          &mut find_chunk_by_id(&chunks, Self::ACTION_LIST_CHUNK_ID)
            .expect("Particle effect actions chunk not found"),
        )
        .map_err(|error| {
          DatabaseParseError::new_database_error(format!(
            "Failed to read particle actions chunk: {}",
            error
          ))
        })?,
        flags: read_u32_chunk::<T>(
          &mut find_chunk_by_id(&chunks, Self::FLAGS_CHUNK_ID)
            .expect("Particle flags chunk not found"),
        )
        .map_err(|error| {
          DatabaseParseError::new_database_error(format!(
            "Failed to read particle flags chunk: {}",
            error
          ))
        })?,
        frame: find_chunk_by_id(&chunks, Self::FRAME_CHUNK_ID).map(|mut it| {
          ParticleEffectFrame::read::<T>(&mut it)
            .expect("Invalid frame chunk data in particle effect")
        }),
        sprite: ParticleEffectSprite::read::<T>(
          &mut find_chunk_by_id(&chunks, Self::SPRITE_CHUNK_ID)
            .expect("Particle frame sprite chunk not found"),
        )
        .map_err(|error| {
          DatabaseParseError::new_database_error(format!(
            "Failed to read particle sprite chunk: {}",
            error
          ))
        })?,
        time_limit: find_chunk_by_id(&chunks, Self::TIME_LIMIT_CHUNK_ID).map(|mut it| {
          read_f32_chunk::<T>(&mut it)
            .expect("Invalid frame time limit chunk data in particle effect")
        }),
        collision: find_chunk_by_id(&chunks, Self::COLLISION_CHUNK_ID).map(|mut it| {
          ParticleEffectCollision::read::<T>(&mut it)
            .expect("Invalid collision chunk data in particle effect")
        }),
        velocity_scale: find_chunk_by_id(&chunks, Self::VELOCITY_SCALE_CHUNK_ID).map(|mut it| {
          read_f32_vector_chunk::<T>(&mut it)
            .expect("Invalid velocity scale chunk data in particle effect")
        }),
        description: find_chunk_by_id(&chunks, Self::DESCRIPTION_CHUNK_ID).map(|mut it| {
          ParticleDescription::read::<T>(&mut it)
            .expect("Invalid description chunk data in particle effect")
        }),
        rotation: find_chunk_by_id(&chunks, Self::ROTATION_CHUNK_ID).map(|mut it| {
          read_f32_vector_chunk::<T>(&mut it)
            .expect("Invalid rotation chunk data in particle effect")
        }),
        editor_data: find_chunk_by_id(&chunks, Self::EDITOR_DATA_CHUNK_ID).map(|mut it| {
          ParticleEffectEditorData::read::<T>(&mut it)
            .expect("Invalid editor data chunk in particle effect")
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
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
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
  pub fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "Particle effect section '{section_name}' should be defined in ltx file ({})",
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

    let mut action_index: usize = 0;
    let mut actions: Vec<ParticleAction> = Vec::new();

    loop {
      let action_section_name: String = Self::get_action_section(section_name, action_index);

      if ltx.has_section(&action_section_name) {
        actions.push(ParticleAction::import(&action_section_name, ltx)?);
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
      version: read_ltx_field("version", section)?,
      name: read_ltx_field("name", section)?,
      max_particles: read_ltx_field("max_particles", section)?,
      actions,
      flags: read_ltx_field("flags", section)?,
      frame: ParticleEffectFrame::import_optional(&Self::get_frame_section(section_name), ltx)?,
      sprite: ParticleEffectSprite::import(&Self::get_sprite_section(section_name), ltx)?,
      time_limit: read_ini_optional_field("time_limit", section)?,
      collision: ParticleEffectCollision::import_optional(
        &Self::get_collision_section(section_name),
        ltx,
      )?,
      velocity_scale: read_ini_optional_field("velocity_scale", section)?,
      description: ParticleDescription::import_optional(
        &Self::get_description_section(section_name),
        ltx,
      )?,
      rotation: read_ini_optional_field("rotation", section)?,
      editor_data: ParticleEffectEditorData::import_optional(
        &Self::get_editor_data_section(section_name),
        ltx,
      )?,
    })
  }

  /// Export particle effect data into provided path.
  pub fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    ltx
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
      .export(&Self::get_sprite_section(section_name), ltx)?;

    for (index, action) in self.actions.iter().enumerate() {
      action.export(&Self::get_action_section(section_name, index), ltx)?
    }

    ParticleEffectFrame::export_optional(
      self.frame.as_ref(),
      &Self::get_frame_section(section_name),
      ltx,
    )?;
    ParticleEffectCollision::export_optional(
      self.collision.as_ref(),
      &Self::get_collision_section(section_name),
      ltx,
    )?;
    ParticleDescription::export_optional(
      self.description.as_ref(),
      &Self::get_description_section(section_name),
      ltx,
    )?;
    ParticleEffectEditorData::export_optional(
      self.editor_data.as_ref(),
      &Self::get_editor_data_section(section_name),
      ltx,
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

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::meta::particle_action_type::ParticleActionType;
  use crate::data::particle::particle_action::particle_action::ParticleAction;
  use crate::data::particle::particle_action::particle_action_copy_vertex::ParticleActionCopyVertex;
  use crate::data::particle::particle_action::particle_action_damping::ParticleActionDamping;
  use crate::data::particle::particle_effect::ParticleEffect;
  use crate::data::particle::particle_effect_collision::ParticleEffectCollision;
  use crate::data::particle::particle_effect_description::ParticleDescription;
  use crate::data::particle::particle_effect_editor_data::ParticleEffectEditorData;
  use crate::data::particle::particle_effect_frame::ParticleEffectFrame;
  use crate::data::particle::particle_effect_sprite::ParticleEffectSprite;
  use crate::data::vector_3d::Vector3d;
  use crate::types::{DatabaseResult, SpawnByteOrder};
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult {
    let filename: String = String::from("read_write.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let original: ParticleEffect = ParticleEffect {
      version: 1,
      name: String::from("test-particle-effect"),
      max_particles: 5,
      actions: vec![
        ParticleAction {
          action_flags: 31,
          action_type: ParticleActionType::Damping as u32,
          data: Box::new(ParticleActionDamping {
            damping: Vector3d {
              x: 1.5,
              y: 2.5,
              z: 3.5,
            },
            v_low_sqr: 1.1,
            v_high_sqr: 1.25,
          }),
        },
        ParticleAction {
          action_flags: 453,
          action_type: ParticleActionType::CopyVertex as u32,
          data: Box::new(ParticleActionCopyVertex { copy_position: 1 }),
        },
      ],
      flags: 140,
      frame: Some(ParticleEffectFrame {
        texture_size: (450.0, 360.0),
        reserved: (45.2, 51.2),
        frame_dimension_x: 320,
        frame_count: 60,
        frame_speed: 29.7,
      }),
      sprite: ParticleEffectSprite {
        shader_name: String::from("test-shader-name"),
        texture_name: String::from("test-texture-name"),
      },
      time_limit: Some(450.1),
      collision: Some(ParticleEffectCollision {
        collide_one_minus_friction: 0.55,
        collide_resilience: 45.2535,
        collide_sqr_cutoff: 25.6313,
      }),
      velocity_scale: Some(Vector3d {
        x: 45.5,
        y: 46.6,
        z: 47.7,
      }),
      description: Some(ParticleDescription {
        creator: String::from("test-creator-name"),
        editor: String::from("test-editor-name"),
        created_time: 425,
        edit_time: 450,
      }),
      rotation: Some(Vector3d {
        x: 1.0,
        y: 4.0,
        z: 6.0,
      }),
      editor_data: Some(ParticleEffectEditorData {
        value: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
      }),
    };

    original.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 343);

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&get_relative_test_sample_file_path(
        file!(),
        &filename,
      ))?,
      0,
    )?;

    assert_eq!(bytes_written, 343);

    let file: FileSlice =
      open_test_resource_as_slice(&get_relative_test_sample_file_path(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 343 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read: ParticleEffect = ParticleEffect::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read.version, original.version);
    assert_eq!(read.name, original.name);
    assert_eq!(read.max_particles, original.max_particles);
    assert_eq!(read.flags, original.flags);
    assert_eq!(read.frame, original.frame);
    assert_eq!(read.sprite, original.sprite);
    assert_eq!(read.time_limit, original.time_limit);
    assert_eq!(read.collision, original.collision);
    assert_eq!(read.velocity_scale, original.velocity_scale);
    assert_eq!(read.description, original.description);
    assert_eq!(read.rotation, original.rotation);
    assert_eq!(read.editor_data, original.editor_data);

    assert_eq!(read.actions.len(), original.actions.len());

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> DatabaseResult {
    let original: ParticleEffect = ParticleEffect {
      version: 1,
      name: String::from("test-particle-effect"),
      max_particles: 5,
      actions: vec![
        ParticleAction {
          action_flags: 31,
          action_type: ParticleActionType::Damping as u32,
          data: Box::new(ParticleActionDamping {
            damping: Vector3d {
              x: 2.5,
              y: 3.5,
              z: 4.5,
            },
            v_low_sqr: 5.2,
            v_high_sqr: 6.5,
          }),
        },
        ParticleAction {
          action_flags: 461,
          action_type: ParticleActionType::CopyVertex as u32,
          data: Box::new(ParticleActionCopyVertex { copy_position: 0 }),
        },
      ],
      flags: 150,
      frame: Some(ParticleEffectFrame {
        texture_size: (460.0, 380.0),
        reserved: (41.2, 42.2),
        frame_dimension_x: 640,
        frame_count: 80,
        frame_speed: 29.7,
      }),
      sprite: ParticleEffectSprite {
        shader_name: String::from("test-shader-name"),
        texture_name: String::from("test-texture-name"),
      },
      time_limit: Some(460.1),
      collision: Some(ParticleEffectCollision {
        collide_one_minus_friction: 0.540,
        collide_resilience: 455.2535,
        collide_sqr_cutoff: 255.6313,
      }),
      velocity_scale: Some(Vector3d {
        x: 455.5,
        y: 465.6,
        z: 475.7,
      }),
      description: Some(ParticleDescription {
        creator: String::from("test-creator-name"),
        editor: String::from("test-editor-name"),
        created_time: 433,
        edit_time: 444,
      }),
      rotation: Some(Vector3d {
        x: 4.5,
        y: 6.5,
        z: 8.5,
      }),
      editor_data: Some(ParticleEffectEditorData {
        value: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
      }),
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(json!(original).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);

    let read: ParticleEffect = serde_json::from_str::<ParticleEffect>(&serialized).unwrap();

    assert_eq!(read.version, original.version);
    assert_eq!(read.name, original.name);
    assert_eq!(read.max_particles, original.max_particles);
    assert_eq!(read.flags, original.flags);
    assert_eq!(read.frame, original.frame);
    assert_eq!(read.sprite, original.sprite);
    assert_eq!(read.time_limit, original.time_limit);
    assert_eq!(read.collision, original.collision);
    assert_eq!(read.velocity_scale, original.velocity_scale);
    assert_eq!(read.description, original.description);
    assert_eq!(read.rotation, original.rotation);
    assert_eq!(read.editor_data, original.editor_data);

    assert_eq!(read.actions.len(), original.actions.len());

    Ok(())
  }
}
