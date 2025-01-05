use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::constants::META_TYPE_FIELD;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ltx_field;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

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

  /// Read particle effect description data from chunk redder.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let particle_description: Self = Self {
      creator: reader.read_null_terminated_win_string()?,
      editor: reader.read_null_terminated_win_string()?,
      created_time: reader.read_u32::<T>()?,
      edit_time: reader.read_u32::<T>()?,
    };

    assert!(
      reader.is_ended(),
      "Expect particle description chunk to be ended"
    );

    Ok(particle_description)
  }

  /// Write particle effect description data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_null_terminated_win_string(&self.creator)?;
    writer.write_null_terminated_win_string(&self.editor)?;
    writer.write_u32::<T>(self.created_time)?;
    writer.write_u32::<T>(self.edit_time)?;

    Ok(())
  }

  /// Import optional particle effect collision data from provided path.
  pub fn import_optional(section_name: &str, ltx: &Ltx) -> DatabaseResult<Option<Self>> {
    if ltx.has_section(section_name) {
      Self::import(section_name, ltx).map(Some)
    } else {
      Ok(None)
    }
  }

  /// Import particle effect description data from provided path.
  pub fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "Particle effect description section '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    let meta_type: String = read_ltx_field(META_TYPE_FIELD, section)?;

    assert_eq!(
      meta_type,
      Self::META_TYPE,
      "Expected corrected meta type field for '{}' importing",
      Self::META_TYPE
    );

    Ok(Self {
      creator: read_ltx_field("creator", section)?,
      editor: read_ltx_field("editor", section)?,
      created_time: read_ltx_field("created_time", section)?,
      edit_time: read_ltx_field("edit_time", section)?,
    })
  }

  /// Export particle effect collision data into provided path.
  pub fn export_optional(data: Option<&Self>, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    if let Some(data) = data {
      data.export(section_name, ltx)
    } else {
      Ok(())
    }
  }

  /// Export particle effect description data into provided path.
  pub fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
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
