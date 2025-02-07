use crate::constants::META_TYPE_FIELD;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};
use xray_utils::assert_equal;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParticleDescription {
  pub creator: String,
  pub editor: String,
  pub created_time: u32,
  pub edit_time: u32,
}

impl ParticleDescription {
  pub const META_TYPE: &'static str = "particle_description";
}

impl ChunkReadWrite for ParticleDescription {
  /// Read particle effect description data from chunk redder.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let particle_description: Self = Self {
      creator: reader.read_w1251_string()?,
      editor: reader.read_w1251_string()?,
      created_time: reader.read_u32::<T>()?,
      edit_time: reader.read_u32::<T>()?,
    };

    reader.assert_read("Expect particle description chunk to be ended")?;

    Ok(particle_description)
  }

  /// Write particle effect description data into chunk writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_w1251_string(&self.creator)?;
    writer.write_w1251_string(&self.editor)?;
    writer.write_u32::<T>(self.created_time)?;
    writer.write_u32::<T>(self.edit_time)?;

    Ok(())
  }
}

impl LtxImportExport for ParticleDescription {
  /// Import particle effect description data from provided path.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle effect description section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    let meta_type: String = read_ltx_field(META_TYPE_FIELD, section)?;

    assert_equal(
      meta_type.as_str(),
      Self::META_TYPE,
      "Expected corrected meta type field for particle effect description importing",
    )?;

    Ok(Self {
      creator: read_ltx_field("creator", section)?,
      editor: read_ltx_field("editor", section)?,
      created_time: read_ltx_field("created_time", section)?,
      edit_time: read_ltx_field("edit_time", section)?,
    })
  }

  /// Export particle effect description data into provided path.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set(META_TYPE_FIELD, Self::META_TYPE)
      .set("creator", &self.creator)
      .set("editor", &self.editor)
      .set("created_time", self.created_time.to_string())
      .set("edit_time", self.edit_time.to_string());

    Ok(())
  }
}
