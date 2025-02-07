use crate::constants::META_TYPE_FIELD;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};
use xray_utils::assert_equal;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticleEffectSprite {
  pub shader_name: String,
  pub texture_name: String,
}

impl ParticleEffectSprite {
  pub const META_TYPE: &'static str = "particle_effect_sprite";
}

impl ChunkReadWrite for ParticleEffectSprite {
  /// Read effect sprite data from chunk redder.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let particle_sprite: Self = Self {
      shader_name: reader.read_w1251_string()?,
      texture_name: reader.read_w1251_string()?,
    };

    reader.assert_read("Expect particle effect sprite chunk to be ended")?;

    Ok(particle_sprite)
  }

  /// Write sprite data into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_w1251_string(&self.shader_name)?;
    writer.write_w1251_string(&self.texture_name)?;

    Ok(())
  }
}

impl LtxImportExport for ParticleEffectSprite {
  /// Import particle effect sprite data from provided path.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle sprite section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    let meta_type: String = read_ltx_field(META_TYPE_FIELD, section)?;

    assert_equal(
      meta_type.as_str(),
      Self::META_TYPE,
      "Expected corrected meta type field for particle effect sprite importing",
    )?;

    Ok(Self {
      shader_name: read_ltx_field("shader_name", section)?,
      texture_name: read_ltx_field("texture_name", section)?,
    })
  }

  /// Export particle effect sprite data into provided path.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set(META_TYPE_FIELD, Self::META_TYPE)
      .set("shader_name", &self.shader_name)
      .set("texture_name", &self.texture_name);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::particles::particle_effect_sprite::ParticleEffectSprite;
  use crate::export::LtxImportExport;
  use serde_json::to_string_pretty;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use std::path::Path;
  use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_ltx::Ltx;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_sample_file_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_file, overwrite_test_relative_resource_as_file,
  };
  use xray_test_utils::FileSlice;

  #[test]
  fn test_read_write() -> XRayResult {
    let filename: String = String::from("read_write.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let original: ParticleEffectSprite = ParticleEffectSprite {
      shader_name: String::from("shader_name"),
      texture_name: String::from("texture_name"),
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 25);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
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
      ParticleEffectSprite::read::<XRayByteOrder>(&mut reader)?;

    assert_eq!(read_sprite, original);

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "import_export.ltx");
    let mut file: File = overwrite_file(config_path)?;
    let mut ltx: Ltx = Ltx::new();

    let original: ParticleEffectSprite = ParticleEffectSprite {
      shader_name: String::from("shader-name-test"),
      texture_name: String::from("texture-name-test"),
    };

    original.export("data", &mut ltx)?;
    ltx.write_to(&mut file)?;

    assert_eq!(
      ParticleEffectSprite::import("data", &Ltx::read_from_path(config_path)?)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> XRayResult {
    let original: ParticleEffectSprite = ParticleEffectSprite {
      shader_name: String::from("shader_name"),
      texture_name: String::from("texture_name"),
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
      serde_json::from_str::<ParticleEffectSprite>(&serialized)?
    );

    Ok(())
  }
}
