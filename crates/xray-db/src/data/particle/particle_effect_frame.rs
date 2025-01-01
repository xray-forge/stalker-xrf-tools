use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::constants::META_TYPE_FIELD;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ini_field;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

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
  pub const META_TYPE: &'static str = "particle_effect_frame";

  /// Read frame data from chunk redder.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let particle_frame: Self = Self {
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

  /// Import particle effect frame data from provided path.
  pub fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "Particle group '{section_name}' should be defined in ltx file ({})",
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

    let texture_size: Vec<String> = read_ini_field::<String>("texture_size", section)?
      .split(',')
      .map(String::from)
      .collect();
    let reserved: Vec<String> = read_ini_field::<String>("reserved", section)?
      .split(',')
      .map(String::from)
      .collect();

    Ok(Self {
      texture_size: (
        texture_size[0]
          .trim()
          .parse::<f32>()
          .or(Err(DatabaseParseError::new_database_error(
            "Failed to parse texture_size W value",
          )))?,
        texture_size[1]
          .trim()
          .parse::<f32>()
          .or(Err(DatabaseParseError::new_database_error(
            "Failed to parse texture_size H value",
          )))?,
      ),
      reserved: (
        reserved[0]
          .trim()
          .parse::<f32>()
          .or(Err(DatabaseParseError::new_database_error(
            "Failed to parse reserved X value",
          )))?,
        reserved[1]
          .trim()
          .parse::<f32>()
          .or(Err(DatabaseParseError::new_database_error(
            "Failed to parse reserved Y value",
          )))?,
      ),
      frame_dimension_x: read_ini_field("frame_dimension_x", section)?,
      frame_count: read_ini_field("frame_count", section)?,
      frame_speed: read_ini_field("frame_speed", section)?,
    })
  }

  /// Export particle effect frame data into provided path.
  pub fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set(META_TYPE_FIELD, Self::META_TYPE)
      .set(
        "texture_size",
        format!("{},{}", self.texture_size.0, self.texture_size.1),
      )
      .set(
        "reserved",
        format!("{},{}", self.reserved.0, self.reserved.1),
      )
      .set("frame_dimension_x", self.frame_dimension_x.to_string())
      .set("frame_count", self.frame_count.to_string())
      .set("frame_speed", self.frame_speed.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::particle::particle_effect_frame::ParticleEffectFrame;
  use crate::export::file::open_ini_config;
  use crate::types::{DatabaseResult, SpawnByteOrder};
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
  fn test_read_write() -> DatabaseResult<()> {
    let filename: String = String::from("read_write.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let original: ParticleEffectFrame = ParticleEffectFrame {
      texture_size: (15.0, 54.5),
      reserved: (25.5, 325.5),
      frame_dimension_x: 155,
      frame_count: 30,
      frame_speed: 2857.0,
    };

    original.write::<SpawnByteOrder>(&mut writer)?;

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

    let read: ParticleEffectFrame = ParticleEffectFrame::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read, original);

    Ok(())
  }

  #[test]
  fn test_import_export() -> DatabaseResult<()> {
    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "import_export.ini");
    let mut file: File = overwrite_file(config_path)?;
    let mut ltx: Ltx = Ltx::new();

    let original: ParticleEffectFrame = ParticleEffectFrame {
      texture_size: (1.5, 2.5),
      reserved: (5.3, 6.1),
      frame_dimension_x: 5,
      frame_count: 24,
      frame_speed: 61.3,
    };

    original.export("data", &mut ltx)?;
    ltx.write_to(&mut file)?;

    let read: ParticleEffectFrame =
      ParticleEffectFrame::import("data", &open_ini_config(config_path)?)?;

    assert_eq!(read, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> DatabaseResult<()> {
    let original: ParticleEffectFrame = ParticleEffectFrame {
      texture_size: (74.0, 236.5),
      reserved: (263.5, 5369.5),
      frame_dimension_x: 7352,
      frame_count: 44,
      frame_speed: 1.5,
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
      serde_json::from_str::<ParticleEffectFrame>(&serialized).unwrap()
    );

    Ok(())
  }
}
