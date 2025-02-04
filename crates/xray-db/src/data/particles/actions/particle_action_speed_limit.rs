use crate::data::particles::particle_action_type::ParticleActionType;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleActionSpeedLimit {
  pub action_flags: u32,
  pub action_type: ParticleActionType,
  pub min_speed: f32,
  pub max_speed: f32,
}

impl ChunkReadWrite for ParticleActionSpeedLimit {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      action_flags: reader.read_u32::<T>()?,
      action_type: reader.read_xr::<T, _>()?,
      min_speed: reader.read_f32::<T>()?,
      max_speed: reader.read_f32::<T>()?,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_u32::<T>(self.action_flags)?;
    writer.write_xr::<T, _>(&self.action_type)?;
    writer.write_f32::<T>(self.min_speed)?;
    writer.write_f32::<T>(self.max_speed)?;

    Ok(())
  }
}

impl LtxImportExport for ParticleActionSpeedLimit {
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle action section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      action_flags: read_ltx_field("action_flags", section)?,
      action_type: read_ltx_field("action_type", section)?,
      min_speed: read_ltx_field("min_speed", section)?,
      max_speed: read_ltx_field("max_speed", section)?,
    })
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("action_flags", self.action_flags.to_string())
      .set("action_type", self.action_type.to_string())
      .set("min_speed", self.min_speed.to_string())
      .set("max_speed", self.max_speed.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::particles::actions::particle_action_speed_limit::ParticleActionSpeedLimit;
  use crate::data::particles::particle_action_type::ParticleActionType;
  use crate::export::LtxImportExport;
  use serde_json::to_string_pretty;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_ltx::Ltx;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_resource_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_test_relative_resource_as_file,
  };
  use xray_test_utils::FileSlice;

  #[test]
  fn test_read_write() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: ParticleActionSpeedLimit = ParticleActionSpeedLimit {
      action_flags: 1,
      action_type: ParticleActionType::SpeedLimit,
      min_speed: 1.0,
      max_speed: 1.25,
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 16);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 16);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 16 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      ParticleActionSpeedLimit::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let ltx_filename: String = get_relative_test_sample_file_path(file!(), "import_export.ltx");
    let mut ltx: Ltx = Ltx::new();

    let original: ParticleActionSpeedLimit = ParticleActionSpeedLimit {
      action_flags: 1,
      action_type: ParticleActionType::SpeedLimit,
      min_speed: 2.0,
      max_speed: 3.25,
    };

    original.export("data", &mut ltx)?;

    ltx.write_to(&mut overwrite_test_relative_resource_as_file(
      &ltx_filename,
    )?)?;

    let source: Ltx = Ltx::read_from_path(get_absolute_test_resource_path(&ltx_filename))?;

    assert_eq!(ParticleActionSpeedLimit::import("data", &source)?, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> XRayResult {
    let original: ParticleActionSpeedLimit = ParticleActionSpeedLimit {
      action_flags: 1,
      action_type: ParticleActionType::SpeedLimit,
      min_speed: 1.25,
      max_speed: 2.15,
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
      serde_json::from_str::<ParticleActionSpeedLimit>(&serialized)?
    );

    Ok(())
  }
}
