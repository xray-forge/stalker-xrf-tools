use crate::constants::META_TYPE_FIELD;
use crate::data::meta::particle_action_type::ParticleActionType;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::error::DatabaseError;
use crate::export::file_import::read_ltx_field;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};
use xray_ltx::{Ltx, Section};

/// C++ src/xrParticles/particle_actions_collection_io.cpp
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleAction {
  pub action_flags: u32,
  pub action_type: u32,
  pub data: Box<dyn ParticleActionWriter>,
}

impl ParticleAction {
  pub const META_TYPE: &'static str = "particle_action";

  /// Read list of particle action data from chunk reader.
  pub fn read_list<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Vec<Self>> {
    let mut actions: Vec<Self> = Vec::new();

    let count: u32 = reader.read_u32::<T>()?;

    for _ in 0..count {
      actions.push(Self::read::<T>(reader).map_err(|error| {
        DatabaseError::new_parse_error(format!("Failed to read particle effect action: {}", error))
      })?);
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

  /// Read effect particle action data from chunk reader.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let action_type_raw: u32 = reader.read_u32::<T>()?;
    let action_type: ParticleActionType = ParticleActionType::from_u32(action_type_raw);

    let action: Self = Self {
      action_flags: reader.read_u32::<T>()?,
      action_type: reader.read_u32::<T>()?,
      data: ParticleActionType::read_by_particle_type::<T>(reader, action_type).map_err(
        |error| {
          DatabaseError::new_parse_error(format!(
            "Failed to read dynamic particle action data for action '{}': {}",
            action_type, error
          ))
        },
      )?,
    };

    assert_eq!(action_type_raw, action.action_type);

    Ok(action)
  }

  /// Write particle action data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_u32::<T>(self.action_type)?;

    writer.write_u32::<T>(self.action_flags)?;
    writer.write_u32::<T>(self.action_type)?;

    self.data.write(writer)?;

    Ok(())
  }

  /// Write particle action data into chunk writer.
  pub fn write_list<T: ByteOrder>(actions: &[Self], writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_u32::<T>(actions.len() as u32)?;

    for action in actions {
      action.write::<T>(writer)?;
    }

    Ok(())
  }

  /// Import particle action data from provided path.
  pub fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      DatabaseError::new_parse_error(format!(
        "Particle action section '{section_name}' should be defined in ltx file ({})",
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

    let action_type: u32 = read_ltx_field("action_type", section)?;

    Ok(Self {
      action_flags: read_ltx_field("action_flags", section)?,
      action_type: read_ltx_field("action_type", section)?,
      data: ParticleActionType::import_by_particle_type(
        ParticleActionType::from_u32(action_type),
        section_name,
        ltx,
      )?,
    })
  }

  /// Export particle action data into provided path.
  pub fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    ltx
      .with_section(section_name)
      .set(META_TYPE_FIELD, Self::META_TYPE)
      .set("action_flags", self.action_flags.to_string())
      .set("action_type", self.action_type.to_string());

    self.data.export(section_name, ltx)?;

    Ok(())
  }
}
