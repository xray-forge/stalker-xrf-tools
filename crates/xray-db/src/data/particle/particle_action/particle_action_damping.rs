use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::meta::particle_action_reader::ParticleActionReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::data::vector_3d::Vector3d;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, ParticlesByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionDamping {
  pub damping: Vector3d,
  pub v_low_sqr: f32,
  pub v_high_sqr: f32,
}

impl ParticleActionReader for ParticleActionDamping {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      damping: reader.read_f32_3d_vector::<T>()?,
      v_low_sqr: reader.read_f32::<T>()?,
      v_high_sqr: reader.read_f32::<T>()?,
    })
  }

  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "Particle action section '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
      damping: read_ini_field("damping", section)?,
      v_low_sqr: read_ini_field("v_low_sqr", section)?,
      v_high_sqr: read_ini_field("v_high_sqr", section)?,
    })
  }
}

#[typetag::serde]
impl ParticleActionWriter for ParticleActionDamping {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_f32_3d_vector::<ParticlesByteOrder>(&self.damping)?;
    writer.write_f32::<ParticlesByteOrder>(self.v_low_sqr)?;
    writer.write_f32::<ParticlesByteOrder>(self.v_high_sqr)?;

    Ok(())
  }

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult {
    ini
      .with_section(section)
      .set("damping", self.damping.to_string())
      .set("v_low_sqr", self.v_low_sqr.to_string())
      .set("v_high_sqr", self.v_high_sqr.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::meta::particle_action_reader::ParticleActionReader;
  use crate::data::meta::particle_action_writer::ParticleActionWriter;
  use crate::data::particle::particle_action::particle_action_damping::ParticleActionDamping;
  use crate::data::vector_3d::Vector3d;
  use crate::export::file::open_ini_config;
  use crate::types::{DatabaseResult, SpawnByteOrder};
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use xray_ltx::Ltx;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_resource_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: ParticleActionDamping = ParticleActionDamping {
      damping: Vector3d {
        x: 1.5,
        y: 2.5,
        z: 3.5,
      },
      v_low_sqr: 62.4,
      v_high_sqr: 55.3,
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 20);

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 20);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 20 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      ParticleActionDamping::read::<SpawnByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> DatabaseResult {
    let ltx_filename: String = get_relative_test_sample_file_path(file!(), "import_export.ini");
    let mut ltx: Ltx = Ltx::new();

    let original: ParticleActionDamping = ParticleActionDamping {
      damping: Vector3d {
        x: 10.5,
        y: 20.5,
        z: 30.5,
      },
      v_low_sqr: 25.4,
      v_high_sqr: 50.3,
    };

    original.export("data", &mut ltx)?;

    ltx.write_to(&mut overwrite_test_relative_resource_as_file(
      &ltx_filename,
    )?)?;

    let source: Ltx = open_ini_config(&get_absolute_test_resource_path(&ltx_filename))?;

    assert_eq!(ParticleActionDamping::import("data", &source)?, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> DatabaseResult {
    let original: ParticleActionDamping = ParticleActionDamping {
      damping: Vector3d {
        x: -1.5,
        y: -2.5,
        z: -3.5,
      },
      v_low_sqr: 150.25,
      v_high_sqr: 40.0,
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
      serde_json::from_str::<ParticleActionDamping>(&serialized).unwrap()
    );

    Ok(())
  }
}
