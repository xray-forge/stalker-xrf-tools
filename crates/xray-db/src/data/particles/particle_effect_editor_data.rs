use crate::constants::META_TYPE_FIELD;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io::Write;
use xray_chunk::{
  assert_chunk_read, read_till_end_binary_chunk, ChunkReadWrite, ChunkReader, ChunkWriter,
};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};
use xray_utils::{assert_equal, decode_bytes_from_base64, encode_bytes_to_base64};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ParticleEffectEditorData {
  pub value: Vec<u8>,
}

impl ParticleEffectEditorData {
  pub const META_TYPE: &'static str = "editor_data";
}

impl ChunkReadWrite for ParticleEffectEditorData {
  /// Read particle effect editor data data from chunk redder.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let particle_description: Self = Self {
      value: read_till_end_binary_chunk(reader)?,
    };

    assert_chunk_read(reader, "Expect particle editor data chunk to be ended")?;

    Ok(particle_description)
  }

  /// Write particle effect description data into chunk writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_all(&self.value)?;

    Ok(())
  }
}

impl LtxImportExport for ParticleEffectEditorData {
  /// Import particle effect description data from provided path.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Particle effect editor data section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    let meta_type: String = read_ltx_field(META_TYPE_FIELD, section)?;

    assert_equal(
      meta_type.as_str(),
      Self::META_TYPE,
      "Expected corrected meta type field for particle editor data importing",
    )?;

    Ok(Self {
      value: decode_bytes_from_base64(&read_ltx_field::<String>("value", section)?)?,
    })
  }

  /// Export particle effect editor data into provided path.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set(META_TYPE_FIELD, Self::META_TYPE)
      .set("value", encode_bytes_to_base64(&self.value));

    Ok(())
  }
}
