use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleEffectFrame {
  texture_size: (f32, f32),
  reserved: (f32, f32),
  frame_dimension_x: u32,
  frame_count: u32,
  frame_speed: f32,
}

impl ParticleEffectFrame {
  /// Read frame data from chunk redder.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<ParticleEffectFrame> {
    let particle_frame: ParticleEffectFrame = ParticleEffectFrame {
      texture_size: (reader.read_f32::<T>()?, reader.read_f32::<T>()?),
      reserved: (reader.read_f32::<T>()?, reader.read_f32::<T>()?),
      frame_dimension_x: reader.read_u32::<T>()?,
      frame_count: reader.read_u32::<T>()?,
      frame_speed: reader.read_f32::<T>()?,
    };

    assert!(reader.is_ended(), "Expect particle frame chunk to be ended");

    Ok(particle_frame)
  }

  /// Write frame data into the writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    writer.write_f32::<T>(self.texture_size.0)?;
    writer.write_f32::<T>(self.texture_size.1)?;
    writer.write_f32::<T>(self.reserved.0)?;
    writer.write_f32::<T>(self.reserved.1)?;
    writer.write_u32::<T>(self.frame_dimension_x)?;
    writer.write_u32::<T>(self.frame_count)?;
    writer.write_f32::<T>(self.frame_speed)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::particle::particle_effect_frame::ParticleEffectFrame;
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
  fn test_read_write_sprite() -> DatabaseResult<()> {
    let filename: String = String::from("particle_effect_frame.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let frame: ParticleEffectFrame = ParticleEffectFrame {
      texture_size: (15.0, 54.5),
      reserved: (25.5, 325.5),
      frame_dimension_x: 155,
      frame_count: 30,
      frame_speed: 2857.0,
    };

    frame.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 28);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&get_relative_test_sample_file_path(
        file!(),
        &filename,
      ))?,
      0,
    )?;

    assert_eq!(bytes_written, 28);

    let file: FileSlice =
      open_test_resource_as_slice(&get_relative_test_sample_file_path(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 28 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_frame: ParticleEffectFrame = ParticleEffectFrame::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_frame, frame);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize_object() -> DatabaseResult<()> {
    let sprite: ParticleEffectFrame = ParticleEffectFrame {
      texture_size: (74.0, 236.5),
      reserved: (263.5, 5369.5),
      frame_dimension_x: 7352,
      frame_count: 44,
      frame_speed: 1.5,
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
      serde_json::from_str::<ParticleEffectFrame>(&serialized).unwrap()
    );

    Ok(())
  }
}
