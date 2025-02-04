use crate::data::alife::inherited::alife_object_abstract::AlifeObjectAbstract;
use crate::export::LtxImportExport;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;
use xray_ltx::Ltx;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectDynamic {
  pub base: AlifeObjectAbstract,
}

impl ChunkReadWrite for AlifeObjectDynamic {
  /// Read dynamic object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      base: reader.read_xr::<T, _>()?,
    })
  }

  /// Write dynamic object data into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_xr::<T, _>(&self.base)?;

    Ok(())
  }
}

impl LtxImportExport for AlifeObjectDynamic {
  /// Import dynamic object data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    Ok(Self {
      base: AlifeObjectAbstract::import(section_name, ltx)?,
    })
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    self.base.export(section_name, ltx)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::inherited::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::inherited::alife_object_dynamic::AlifeObjectDynamic;
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

    let original: AlifeObjectDynamic = AlifeObjectDynamic {
      base: AlifeObjectAbstract {
        game_vertex_id: 1001,
        distance: 65.25,
        direct_control: 412421,
        level_vertex_id: 66231,
        flags: 33,
        custom_data: String::from("custom_data"),
        story_id: 400,
        spawn_story_id: 32,
      },
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 38);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 38);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 38 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectDynamic = AlifeObjectDynamic::read::<XRayByteOrder>(&mut reader)?;

    assert_eq!(read_object, original);

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let ltx_filename: String = get_relative_test_sample_file_path(file!(), "import_export.ltx");
    let mut ltx: Ltx = Ltx::new();

    let original: AlifeObjectDynamic = AlifeObjectDynamic {
      base: AlifeObjectAbstract {
        game_vertex_id: 5,
        distance: 5.25,
        direct_control: 52,
        level_vertex_id: 636,
        flags: 25,
        custom_data: String::from("custom_data"),
        story_id: 26,
        spawn_story_id: 36,
      },
    };

    original.export("data", &mut ltx)?;

    ltx.write_to(&mut overwrite_test_relative_resource_as_file(
      &ltx_filename,
    )?)?;

    let source: Ltx = Ltx::read_from_path(get_absolute_test_resource_path(&ltx_filename))?;

    assert_eq!(AlifeObjectDynamic::import("data", &source)?, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> XRayResult {
    let original: AlifeObjectDynamic = AlifeObjectDynamic {
      base: AlifeObjectAbstract {
        game_vertex_id: 73,
        distance: 1.25,
        direct_control: 73435,
        level_vertex_id: 3456,
        flags: 3,
        custom_data: String::from("custom_data"),
        story_id: 54,
        spawn_story_id: 64,
      },
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(to_string_pretty(&original)?.as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);

    assert_eq!(
      serde_json::from_str::<AlifeObjectDynamic>(&serialized)?,
      original
    );

    Ok(())
  }
}
