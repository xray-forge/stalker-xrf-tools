use crate::chunk::reader::ChunkReader;
use crate::chunk::utils::read_till_end_binary_chunk;
use crate::chunk::writer::ChunkWriter;
use crate::constants::META_TYPE_FIELD;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ini_field;
use crate::export::string::{bytes_from_base64, bytes_to_base64};
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io::Write;
use xray_ltx::{Ltx, Section};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParticleEffectEditorData {
  pub value: Vec<u8>,
}

impl ParticleEffectEditorData {
  pub const META_TYPE: &'static str = "editor_data";

  /// Read particle effect editor data data from chunk redder.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let particle_description: Self = Self {
      value: read_till_end_binary_chunk(reader)?,
    };

    assert!(
      reader.is_ended(),
      "Expect particle editor data chunk to be ended"
    );

    Ok(particle_description)
  }

  /// Write particle effect description data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    writer.write_all(&self.value)?;

    Ok(())
  }

  /// Import optional particle effect collision data from provided path.
  pub fn import_optional(section_name: &str, ini: &Ltx) -> DatabaseResult<Option<Self>> {
    if ini.has_section(section_name) {
      Self::import(section_name, ini).map(Some)
    } else {
      Ok(None)
    }
  }

  /// Import particle effect description data from provided path.
  pub fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "Particle effect editor data section '{section_name}' should be defined in ltx file ({})",
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

    Ok(Self {
      value: bytes_from_base64(&read_ini_field::<String>("value", section)?)?,
    })
  }

  /// Export particle effect collision data into provided path.
  pub fn export_optional(
    data: Option<&Self>,
    section_name: &str,
    ini: &mut Ltx,
  ) -> DatabaseResult<()> {
    if let Some(data) = data {
      data.export(section_name, ini)
    } else {
      Ok(())
    }
  }

  /// Export particle effect editor data into provided path.
  pub fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    ini
      .with_section(section)
      .set(META_TYPE_FIELD, Self::META_TYPE)
      .set("value", bytes_to_base64(&self.value));

    Ok(())
  }
}
