use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectVisual {
  pub visual_name: String,
  pub visual_flags: u8,
}

impl AlifeObjectReader for AlifeObjectVisual {
  /// Read visual object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      visual_name: reader.read_w1251_string()?,
      visual_flags: reader.read_u8()?,
    })
  }

  /// Import visual object data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "ALife object '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      visual_name: read_ltx_field("visual_name", section)?,
      visual_flags: read_ltx_field("visual_flags", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeObjectVisual {
  /// Write visual alife object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_w1251_string(&self.visual_name)?;
    writer.write_u8(self.visual_flags)?;

    Ok(())
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("visual_name", &self.visual_name)
      .set("visual_flags", self.visual_flags.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::alife_object_visual::AlifeObjectVisual;
  use crate::data::meta::alife_object_generic::AlifeObjectWriter;
  use crate::data::meta::alife_object_reader::AlifeObjectReader;
  use std::fs::File;
  use std::path::Path;
  use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_ltx::Ltx;
  use xray_test_utils::utils::{
    get_absolute_test_sample_file_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_file, overwrite_test_relative_resource_as_file,
  };
  use xray_test_utils::FileSlice;

  #[test]
  fn test_read_write() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: AlifeObjectVisual = AlifeObjectVisual {
      visual_name: String::from("visual-name"),
      visual_flags: 33,
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 13);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 13);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 13 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectVisual::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "import_export.ltx");
    let mut file: File = overwrite_file(config_path)?;
    let mut ltx: Ltx = Ltx::new();

    let first: AlifeObjectVisual = AlifeObjectVisual {
      visual_name: String::from("visual-name-example"),
      visual_flags: 33,
    };

    let second: AlifeObjectVisual = AlifeObjectVisual {
      visual_name: String::from(""),
      visual_flags: 33,
    };

    first.export("first", &mut ltx)?;
    second.export("second", &mut ltx)?;

    ltx.write_to(&mut file)?;

    let source: Ltx = Ltx::read_from_path(config_path)?;

    assert_eq!(AlifeObjectVisual::import("first", &source)?, first);
    assert_eq!(AlifeObjectVisual::import("second", &source)?, second);

    Ok(())
  }
}
