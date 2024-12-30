use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleEffectSprite {
  shader_name: String,
  texture_name: String,
}

impl ParticleEffectSprite {
  /// Read effect sprite data from chunk redder.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<ParticleEffectSprite> {
    let particle_sprite: ParticleEffectSprite = ParticleEffectSprite {
      shader_name: reader.read_null_terminated_win_string()?,
      texture_name: reader.read_null_terminated_win_string()?,
    };

    assert!(
      reader.is_ended(),
      "Expect particle effect sprite chunk to be ended"
    );

    Ok(particle_sprite)
  }

  /// Write sprite data into the writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_null_terminated_win_string(&self.shader_name)?;
    writer.write_null_terminated_win_string(&self.texture_name)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::particle::particle_effect_sprite::ParticleEffectSprite;
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io;
  use std::io::{Seek, SeekFrom, Write};
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write_sprite() -> io::Result<()> {
    let filename: String = String::from("particle_effect_sprite.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let sprite: ParticleEffectSprite = ParticleEffectSprite {
      shader_name: String::from("shader_name"),
      texture_name: String::from("texture_name"),
    };

    sprite.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 25);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&get_relative_test_sample_file_path(
        file!(),
        &filename,
      ))?,
      0,
    )?;

    assert_eq!(bytes_written, 25);

    let file: FileSlice =
      open_test_resource_as_slice(&get_relative_test_sample_file_path(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 25 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_sprite: ParticleEffectSprite =
      ParticleEffectSprite::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_sprite, sprite);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize_object() -> io::Result<()> {
    let sprite: ParticleEffectSprite = ParticleEffectSprite {
      shader_name: String::from("shader_name"),
      texture_name: String::from("texture_name"),
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialized.json"),
    )?;

    file.write_all(json!(sprite).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      sprite,
      serde_json::from_str::<ParticleEffectSprite>(&serialized)?
    );

    Ok(())
  }
}
