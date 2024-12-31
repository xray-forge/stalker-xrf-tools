use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use xray_ltx::Ltx;

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

  /// Read list of effect groups data from chunk reader.
  pub fn read_list<T: ByteOrder>(
    reader: &mut ChunkReader,
  ) -> DatabaseResult<Vec<ParticleGroupEffect>> {
    let mut effects: Vec<ParticleGroupEffect> = Vec::new();

    let count: u32 = reader.read_u32::<T>()?;

    for _ in 0..count {
      effects.push(ParticleGroupEffect::read::<T>(reader)?);
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

  /// Read group effect from chunk reader binary data.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleGroupEffect> {
    let particle_group = ParticleGroupEffect {
      name: reader.read_null_terminated_win_string()?,
      on_play_child_name: reader.read_null_terminated_win_string()?,
      on_birth_child_name: reader.read_null_terminated_win_string()?,
      on_dead_child_name: reader.read_null_terminated_win_string()?,
      time_0: reader.read_f32::<T>()?,
      time_1: reader.read_f32::<T>()?,
      flags: reader.read_u32::<T>()?,
    };

    Ok(particle_group)
  }

  /// Write effects list data into the writer.
  pub fn write_list<T: ByteOrder>(
    effects: &Vec<ParticleGroupEffect>,
    writer: &mut ChunkWriter,
  ) -> DatabaseResult<()> {
    writer.write_u32::<T>(effects.len() as u32)?;

    for effect in effects {
      effect.write::<T>(writer)?;
    }

    Ok(())
  }

  /// Write effect data into the writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    writer.write_null_terminated_win_string(&self.name)?;
    writer.write_null_terminated_win_string(&self.on_play_child_name)?;
    writer.write_null_terminated_win_string(&self.on_birth_child_name)?;
    writer.write_null_terminated_win_string(&self.on_dead_child_name)?;
    writer.write_f32::<T>(self.time_0)?;
    writer.write_f32::<T>(self.time_1)?;
    writer.write_u32::<T>(self.flags)?;

    Ok(())
  }

  /// Import particles group effect data from provided path.
  pub fn import(path: &Path) -> DatabaseResult<ParticleGroupEffect> {
    todo!("Implement");
  }

  /// Export particles group effect data into provided path.
  pub fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set("$type", Self::META_TYPE)
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
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::particle::particle_group_effect::ParticleGroupEffect;
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
  fn test_read_write_effect() -> DatabaseResult<()> {
    let filename: String = String::from("particle_group_effect.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let effect: ParticleGroupEffect = ParticleGroupEffect {
      name: String::from("effect_name"),
      on_play_child_name: String::from("effect_on_play_child_name"),
      on_birth_child_name: String::from("effect_on_birth_child_name"),
      on_dead_child_name: String::from("effect_on_dead_child_name"),
      time_0: 568.50,
      time_1: 234.25,
      flags: 763,
    };

    effect.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 103);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
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

    let read_effect: ParticleGroupEffect =
      ParticleGroupEffect::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_effect, effect);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize_object() -> DatabaseResult<()> {
    let effect: ParticleGroupEffect = ParticleGroupEffect {
      name: String::from("effect_old_name_serialize"),
      on_play_child_name: String::from("effect_old_on_play_child_name_serialize"),
      on_birth_child_name: String::from("effect_on_birth_child_name_serialize"),
      on_dead_child_name: String::from("effect_on_dead_child_name_serialize"),
      time_0: 7654.5,
      time_1: 2351.25,
      flags: 67423,
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialized.json"),
    )?;

    file.write_all(json!(effect).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      effect,
      serde_json::from_str::<ParticleGroupEffect>(&serialized).unwrap()
    );

    Ok(())
  }
}
