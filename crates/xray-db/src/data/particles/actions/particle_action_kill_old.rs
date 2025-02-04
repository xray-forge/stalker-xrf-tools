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
pub struct ParticleActionKillOld {
  pub action_flags: u32,
  pub action_type: ParticleActionType,
  pub age_limit: f32,
  pub kill_less_than: u32,
}

impl ChunkReadWrite for ParticleActionKillOld {
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      action_flags: reader.read_u32::<T>()?,
      action_type: reader.read_xr::<T, _>()?,
      age_limit: reader.read_f32::<T>()?,
      kill_less_than: reader.read_u32::<T>()?,
    })
  }

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_u32::<T>(self.action_flags)?;
    writer.write_xr::<T, _>(&self.action_type)?;
    writer.write_f32::<T>(self.age_limit)?;
    writer.write_u32::<T>(self.kill_less_than)?;

    Ok(())
  }
}

impl LtxImportExport for ParticleActionKillOld {
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
      age_limit: read_ltx_field("age_limit", section)?,
      kill_less_than: read_ltx_field("kill_less_than", section)?,
    })
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("action_flags", self.action_flags.to_string())
      .set("action_type", self.action_type.to_string())
      .set("age_limit", self.age_limit.to_string())
      .set("kill_less_than", self.kill_less_than.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::particles::actions::particle_action_kill_old::ParticleActionKillOld;
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

    let original: ParticleActionKillOld = ParticleActionKillOld {
      action_flags: 1,
      action_type: ParticleActionType::KillOld,
      age_limit: 25.3,
      kill_less_than: 12,
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
      ParticleActionKillOld::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let ltx_filename: String = get_relative_test_sample_file_path(file!(), "import_export.ltx");
    let mut ltx: Ltx = Ltx::new();

    let original: ParticleActionKillOld = ParticleActionKillOld {
      action_flags: 1,
      action_type: ParticleActionType::KillOld,
      age_limit: 25.3,
      kill_less_than: 12,
    };

    original.export("data", &mut ltx)?;

    ltx.write_to(&mut overwrite_test_relative_resource_as_file(
      &ltx_filename,
    )?)?;

    let source: Ltx = Ltx::read_from_path(get_absolute_test_resource_path(&ltx_filename))?;

    assert_eq!(ParticleActionKillOld::import("data", &source)?, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> XRayResult {
    let original: ParticleActionKillOld = ParticleActionKillOld {
      action_flags: 1,
      action_type: ParticleActionType::KillOld,
      age_limit: 25.3,
      kill_less_than: 12,
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
      serde_json::from_str::<ParticleActionKillOld>(&serialized)?
    );

    Ok(())
  }
}
