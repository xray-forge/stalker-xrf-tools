use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::constants::META_TYPE_FIELD;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ltx_field;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleGroupEffectOld {
  pub name: String,
  pub on_play_child_name: String,
  pub time_0: f32,
  pub time_1: f32,
  pub flags: u32,
}

impl ParticleGroupEffectOld {
  pub const META_TYPE: &'static str = "particle_group_effect_old";

  pub const EFFECT_ACTIONS_LIMIT: usize = 10_000;

  /// Read list of old effect groups data from chunk reader.
  pub fn read_list<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Vec<Self>> {
    let mut effects: Vec<Self> = Vec::new();

    let count: u32 = reader.read_u32::<T>()?;

    for _ in 0..count {
      effects.push(Self::read::<T>(reader)?);
    }

    assert_eq!(
      effects.len(),
      count as usize,
      "Should read same count of effects as declared in chunk"
    );

    assert!(
      reader.is_ended(),
      "Expect particle effects list chunk to be ended"
    );

    Ok(effects)
  }

  /// Read old group effect from chunk reader binary data.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let particle_group = Self {
      name: reader.read_null_terminated_win_string()?,
      on_play_child_name: reader.read_null_terminated_win_string()?,
      time_0: reader.read_f32::<T>()?,
      time_1: reader.read_f32::<T>()?,
      flags: reader.read_u32::<T>()?,
    };

    Ok(particle_group)
  }

  /// Write old effects list data into the writer.
  pub fn write_list<T: ByteOrder>(effects: &[Self], writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_u32::<T>(effects.len() as u32)?;

    for effect in effects {
      effect.write::<T>(writer)?;
    }

    Ok(())
  }

  /// Write old effect data into the writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_null_terminated_win_string(&self.name)?;
    writer.write_null_terminated_win_string(&self.on_play_child_name)?;
    writer.write_f32::<T>(self.time_0)?;
    writer.write_f32::<T>(self.time_1)?;
    writer.write_u32::<T>(self.flags)?;

    Ok(())
  }

  /// Import list of particles group effect data from provided path.
  pub fn import_list(section_name: &str, ltx: &Ltx) -> DatabaseResult<Vec<Self>> {
    let mut effect_index: usize = 0;
    let mut effects: Vec<Self> = Vec::new();

    loop {
      let action_section_name: String = Self::get_effect_old_section(section_name, effect_index);

      if ltx.has_section(&action_section_name) {
        effects.push(Self::import(&action_section_name, ltx)?);
        effect_index += 1
      } else {
        break;
      }

      if effect_index >= Self::EFFECT_ACTIONS_LIMIT {
        return Err(DatabaseParseError::new_database_error(
          "Failed to parse particle effects - reached maximum nested actions limit",
        ));
      }
    }

    Ok(effects)
  }

  /// Import particles group effect data from provided path.
  pub fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "Particle group effect old section '{section_name}' should be defined in ltx file ({})",
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

    Ok(Self {
      name: read_ltx_field("name", section)?,
      on_play_child_name: read_ltx_field("on_play_child_name", section)?,
      time_0: read_ltx_field("time_0", section)?,
      time_1: read_ltx_field("time_1", section)?,
      flags: read_ltx_field("flags", section)?,
    })
  }

  /// Export list of particles group effect data into provided path.
  pub fn export_list(effects_old: &[Self], section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    for (index, effect) in effects_old.iter().enumerate() {
      effect.export(&Self::get_effect_old_section(section_name, index), ltx)?
    }

    Ok(())
  }

  /// Export particles group effect data into provided path.
  pub fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    ltx
      .with_section(section_name)
      .set(META_TYPE_FIELD, Self::META_TYPE)
      .set("name", &self.name)
      .set("on_play_child_name", &self.on_play_child_name)
      .set("time_0", self.time_0.to_string())
      .set("time_1", self.time_1.to_string())
      .set("flags", self.flags.to_string());

    Ok(())
  }
}

impl ParticleGroupEffectOld {
  pub fn get_effect_old_section(section_name: &str, index: usize) -> String {
    format!("{section_name}.effect_old.{index}")
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::particle::particle_group_effect_old::ParticleGroupEffectOld;
  use crate::export::file::open_ltx_config;
  use crate::types::{DatabaseResult, ParticlesByteOrder};
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use std::path::Path;
  use xray_ltx::Ltx;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_sample_file_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_file, overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write_list() -> DatabaseResult {
    let filename: String = String::from("read_write_list.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let original: Vec<ParticleGroupEffectOld> = vec![
      ParticleGroupEffectOld {
        name: String::from("test-effect-old-1"),
        on_play_child_name: String::from("test-effect-old-on-play-child-name-1"),
        time_0: 41.5,
        time_1: 42.30,
        flags: 45,
      },
      ParticleGroupEffectOld {
        name: String::from("test-effect-old-2"),
        on_play_child_name: String::from("test-effect-old-on-play-child-2"),
        time_0: 51.5,
        time_1: 52.30,
        flags: 46,
      },
      ParticleGroupEffectOld {
        name: String::from("test-effect-old-3"),
        on_play_child_name: String::from("test-effect-old-on-play-child-name-3"),
        time_0: 61.5,
        time_1: 62.30,
        flags: 47,
      },
    ];

    ParticleGroupEffectOld::write_list::<ParticlesByteOrder>(&original, &mut writer)?;

    assert_eq!(writer.bytes_written(), 200);

    let bytes_written: usize = writer.flush_chunk_into::<ParticlesByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&get_relative_test_sample_file_path(
        file!(),
        &filename,
      ))?,
      0,
    )?;

    assert_eq!(bytes_written, 200);

    let file: FileSlice =
      open_test_resource_as_slice(&get_relative_test_sample_file_path(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 200 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    assert_eq!(
      ParticleGroupEffectOld::read_list::<ParticlesByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_read_write() -> DatabaseResult {
    let filename: String = String::from("read_write.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let original: ParticleGroupEffectOld = ParticleGroupEffectOld {
      name: String::from("effect_old_name"),
      on_play_child_name: String::from("effect_old_on_play_child_name"),
      time_0: 150.50,
      time_1: 250.50,
      flags: 1392,
    };

    original.write::<ParticlesByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 58);

    let bytes_written: usize = writer.flush_chunk_into::<ParticlesByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&get_relative_test_sample_file_path(
        file!(),
        &filename,
      ))?,
      0,
    )?;

    assert_eq!(bytes_written, 58);

    let file: FileSlice =
      open_test_resource_as_slice(&get_relative_test_sample_file_path(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 58 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    assert_eq!(
      ParticleGroupEffectOld::read::<ParticlesByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> DatabaseResult {
    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "import_export.ltx");
    let mut file: File = overwrite_file(config_path)?;
    let mut ltx: Ltx = Ltx::new();

    let original: ParticleGroupEffectOld = ParticleGroupEffectOld {
      name: String::from("test-effect-old"),
      on_play_child_name: String::from("test-effect-old-on-play-child-name"),
      time_0: 1.5,
      time_1: 5.30,
      flags: 33,
    };

    original.export("data", &mut ltx)?;
    ltx.write_to(&mut file)?;

    assert_eq!(
      ParticleGroupEffectOld::import("data", &open_ltx_config(config_path)?)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export_list() -> DatabaseResult {
    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "import_export_list.ltx");
    let mut file: File = overwrite_file(config_path)?;
    let mut ltx: Ltx = Ltx::new();

    let original: Vec<ParticleGroupEffectOld> = vec![
      ParticleGroupEffectOld {
        name: String::from("test-effect-old-1"),
        on_play_child_name: String::from("test-effect-old-on-play-child-name-1"),
        time_0: 5.5,
        time_1: 6.30,
        flags: 45,
      },
      ParticleGroupEffectOld {
        name: String::from("test-effect-old-2"),
        on_play_child_name: String::from("test-effect-old-on-play-child-2"),
        time_0: 5.5,
        time_1: 6.30,
        flags: 46,
      },
      ParticleGroupEffectOld {
        name: String::from("test-effect-old-3"),
        on_play_child_name: String::from("test-effect-old-on-play-child-name-3"),
        time_0: 364.5,
        time_1: 554.30,
        flags: 47,
      },
    ];

    ParticleGroupEffectOld::export_list(&original, "data", &mut ltx)?;

    ltx.write_to(&mut file)?;

    assert_eq!(
      ParticleGroupEffectOld::import_list("data", &open_ltx_config(config_path)?)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> DatabaseResult {
    let original: ParticleGroupEffectOld = ParticleGroupEffectOld {
      name: String::from("effect_old_name_serialize"),
      on_play_child_name: String::from("effect_old_on_play_child_name_serialize"),
      time_0: 126.5,
      time_1: 567.5,
      flags: 3765,
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(json!(original).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      original,
      serde_json::from_str::<ParticleGroupEffectOld>(&serialized).unwrap()
    );

    Ok(())
  }
}
