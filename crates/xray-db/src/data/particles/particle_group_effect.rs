use crate::constants::META_TYPE_FIELD;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReadWriteList, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};
use xray_utils::{assert_equal, assert_length};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleGroupEffect {
  pub name: String,
  pub on_play_child_name: String,
  pub on_birth_child_name: String,
  pub on_dead_child_name: String,
  pub time_0: f32,
  pub time_1: f32,
  pub flags: u32,
}

impl ParticleGroupEffect {
  pub const META_TYPE: &'static str = "particle_group_effect";

  pub const EFFECT_ACTIONS_LIMIT: usize = 10_000;

  pub fn get_effect_section(section_name: &str, index: usize) -> String {
    format!("{section_name}.effect.{index}")
  }
}

impl ChunkReadWriteList for ParticleGroupEffect {
  /// Read list of effect groups data from chunk reader.
  fn read_list<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Vec<Self>> {
    let count: u32 = reader.read_u32::<T>()?;

    let mut effects: Vec<Self> = Vec::with_capacity(count as usize);

    for _ in 0..count {
      effects.push(Self::read::<T>(reader)?);
    }

    assert_length(
      &effects,
      count as usize,
      "Should read same count of effects as declared in chunk",
    )?;
    reader.assert_read("Expect particle effects list chunk to be ended")?;

    Ok(effects)
  }

  /// Write effects list data into the writer.
  fn write_list<T: ByteOrder>(writer: &mut ChunkWriter, list: &[Self]) -> XRayResult {
    writer.write_u32::<T>(list.len() as u32)?;

    for effect in list {
      effect.write::<T>(writer)?;
    }

    Ok(())
  }
}

impl ChunkReadWrite for ParticleGroupEffect {
  /// Read group effect from chunk reader binary data.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let particle_group = Self {
      name: reader.read_w1251_string()?,
      on_play_child_name: reader.read_w1251_string()?,
      on_birth_child_name: reader.read_w1251_string()?,
      on_dead_child_name: reader.read_w1251_string()?,
      time_0: reader.read_f32::<T>()?,
      time_1: reader.read_f32::<T>()?,
      flags: reader.read_u32::<T>()?,
    };

    Ok(particle_group)
  }

  /// Write effect data into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_w1251_string(&self.name)?;
    writer.write_w1251_string(&self.on_play_child_name)?;
    writer.write_w1251_string(&self.on_birth_child_name)?;
    writer.write_w1251_string(&self.on_dead_child_name)?;
    writer.write_f32::<T>(self.time_0)?;
    writer.write_f32::<T>(self.time_1)?;
    writer.write_u32::<T>(self.flags)?;

    Ok(())
  }
}

impl ParticleGroupEffect {
  /// Import list of particles group effect data from provided path.
  pub fn import_list(section_name: &str, ltx: &Ltx) -> XRayResult<Vec<Self>> {
    let mut effect_index: usize = 0;
    let mut effects: Vec<Self> = Vec::new();

    loop {
      let action_section_name: String = Self::get_effect_section(section_name, effect_index);

      if ltx.has_section(&action_section_name) {
        effects.push(Self::import(&action_section_name, ltx)?);
        effect_index += 1
      } else {
        break;
      }

      if effect_index >= Self::EFFECT_ACTIONS_LIMIT {
        return Err(XRayError::new_parsing_error(
          "Failed to parse particle effects - reached maximum nested actions limit",
        ));
      }
    }

    Ok(effects)
  }

  /// Import particles group effect data from provided path.
  pub fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle group effect section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    let meta_type: String = read_ltx_field(META_TYPE_FIELD, section)?;

    assert_equal(
      meta_type.as_str(),
      Self::META_TYPE,
      "Expected corrected meta type field for particle group effect importing",
    )?;

    Ok(Self {
      name: read_ltx_field("name", section)?,
      on_play_child_name: read_ltx_field("on_play_child_name", section)?,
      on_birth_child_name: read_ltx_field("on_birth_child_name", section)?,
      on_dead_child_name: read_ltx_field("on_dead_child_name", section)?,
      time_0: read_ltx_field("time_0", section)?,
      time_1: read_ltx_field("time_1", section)?,
      flags: read_ltx_field("flags", section)?,
    })
  }

  /// Export list of particles group effect data into provided path.
  pub fn export_list(effects_old: &[Self], section_name: &str, ltx: &mut Ltx) -> XRayResult {
    for (index, effect) in effects_old.iter().enumerate() {
      effect.export(&Self::get_effect_section(section_name, index), ltx)?
    }

    Ok(())
  }

  /// Export particles group effect data into provided path.
  pub fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set(META_TYPE_FIELD, Self::META_TYPE)
      .set("name", &self.name)
      .set("on_play_child_name", &self.on_play_child_name)
      .set("on_birth_child_name", &self.on_birth_child_name)
      .set("on_dead_child_name", &self.on_dead_child_name)
      .set("time_0", self.time_0.to_string())
      .set("time_1", self.time_1.to_string())
      .set("flags", self.flags.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::particles::particle_group_effect::ParticleGroupEffect;
  use serde_json::to_string_pretty;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use std::path::Path;
  use xray_chunk::{ChunkReadWrite, ChunkReadWriteList, ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_ltx::Ltx;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_sample_file_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_file, overwrite_test_relative_resource_as_file,
  };
  use xray_test_utils::FileSlice;

  #[test]
  fn test_read_write_list() -> XRayResult {
    let filename: String = String::from("read_write_list.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let original: Vec<ParticleGroupEffect> = vec![
      ParticleGroupEffect {
        name: String::from("test-effect-old-1"),
        on_play_child_name: String::from("test-effect-old-on-play-child-name-1"),
        on_birth_child_name: String::from("effect-on-birth-child-name-1"),
        on_dead_child_name: String::from("effect-on-dead-child-name-1"),
        time_0: 41.5,
        time_1: 42.30,
        flags: 45,
      },
      ParticleGroupEffect {
        name: String::from("test-effect-old-2"),
        on_play_child_name: String::from("test-effect-old-on-play-child-2"),
        on_birth_child_name: String::from("effect-on-birth-child-name-2"),
        on_dead_child_name: String::from("effect-on-dead-child-name2"),
        time_0: 51.5,
        time_1: 52.30,
        flags: 46,
      },
      ParticleGroupEffect {
        name: String::from("test-effect-old-3"),
        on_play_child_name: String::from("test-effect-old-on-play-child-name-3"),
        on_birth_child_name: String::from("effect-on-birth-child-name-3"),
        on_dead_child_name: String::from("effect-on-dead-child-name-3"),
        time_0: 61.5,
        time_1: 62.30,
        flags: 47,
      },
    ];

    ParticleGroupEffect::write_list::<XRayByteOrder>(&mut writer, &original)?;

    assert_eq!(writer.bytes_written(), 370);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&get_relative_test_sample_file_path(
        file!(),
        &filename,
      ))?,
      0,
    )?;

    assert_eq!(bytes_written, 370);

    let file: FileSlice =
      open_test_resource_as_slice(&get_relative_test_sample_file_path(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 370 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    assert_eq!(
      ParticleGroupEffect::read_list::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_read_write() -> XRayResult {
    let filename: String = String::from("read_write.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let original: ParticleGroupEffect = ParticleGroupEffect {
      name: String::from("effect_name"),
      on_play_child_name: String::from("effect_on_play_child_name"),
      on_birth_child_name: String::from("effect_on_birth_child_name"),
      on_dead_child_name: String::from("effect_on_dead_child_name"),
      time_0: 568.50,
      time_1: 234.25,
      flags: 763,
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 103);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&get_relative_test_sample_file_path(
        file!(),
        &filename,
      ))?,
      0,
    )?;

    assert_eq!(bytes_written, 103);

    let file: FileSlice =
      open_test_resource_as_slice(&get_relative_test_sample_file_path(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 103 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    assert_eq!(
      ParticleGroupEffect::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "import_export.ltx");
    let mut file: File = overwrite_file(config_path)?;
    let mut ltx: Ltx = Ltx::new();

    let original: ParticleGroupEffect = ParticleGroupEffect {
      name: String::from("test-effect-old"),
      on_play_child_name: String::from("test-effect-old-on-play-child-name"),
      on_birth_child_name: String::from("test-effect-old-on-birth-child-name"),
      on_dead_child_name: String::from("test-effect-old-on-dead-child-name"),
      time_0: 1.5,
      time_1: 5.30,
      flags: 33,
    };

    original.export("data", &mut ltx)?;
    ltx.write_to(&mut file)?;

    assert_eq!(
      ParticleGroupEffect::import("data", &Ltx::read_from_path(config_path)?)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export_list() -> XRayResult {
    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "import_export_list.ltx");
    let mut file: File = overwrite_file(config_path)?;
    let mut ltx: Ltx = Ltx::new();

    let original: Vec<ParticleGroupEffect> = vec![
      ParticleGroupEffect {
        name: String::from("test-effect-old-1"),
        on_play_child_name: String::from("test-effect-old-on-play-child-name-1"),
        on_birth_child_name: String::from("effect-on-birth-child-name-1"),
        on_dead_child_name: String::from("effect-on-dead-child-name-1"),
        time_0: 54.5,
        time_1: 55.30,
        flags: 45,
      },
      ParticleGroupEffect {
        name: String::from("test-effect-old-2"),
        on_play_child_name: String::from("test-effect-old-on-play-child-2"),
        on_birth_child_name: String::from("effect-on-birth-child-name-2"),
        on_dead_child_name: String::from("effect-on-dead-child-name2"),
        time_0: 87.5,
        time_1: 88.30,
        flags: 46,
      },
      ParticleGroupEffect {
        name: String::from("test-effect-old-3"),
        on_play_child_name: String::from("test-effect-old-on-play-child-name-3"),
        on_birth_child_name: String::from("effect-on-birth-child-name-3"),
        on_dead_child_name: String::from("effect-on-dead-child-name-3"),
        time_0: 92.5,
        time_1: 93.30,
        flags: 47,
      },
    ];

    ParticleGroupEffect::export_list(&original, "data", &mut ltx)?;

    ltx.write_to(&mut file)?;

    assert_eq!(
      ParticleGroupEffect::import_list("data", &Ltx::read_from_path(config_path)?)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> XRayResult {
    let original: ParticleGroupEffect = ParticleGroupEffect {
      name: String::from("effect_old_name_serialize"),
      on_play_child_name: String::from("effect_old_on_play_child_name_serialize"),
      on_birth_child_name: String::from("effect_on_birth_child_name_serialize"),
      on_dead_child_name: String::from("effect_on_dead_child_name_serialize"),
      time_0: 7654.5,
      time_1: 2351.25,
      flags: 67423,
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(to_string_pretty(&original)?.as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      original,
      serde_json::from_str::<ParticleGroupEffect>(&serialized)?
    );

    Ok(())
  }
}
