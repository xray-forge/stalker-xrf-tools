use crate::data::alife::inherited::alife_object_item::AlifeObjectItem;
use crate::export::LtxImportExport;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;
use xray_ltx::Ltx;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectItemDetector {
  pub base: AlifeObjectItem,
}

impl ChunkReadWrite for AlifeObjectItemDetector {
  /// Read ALife item object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      base: reader.read_xr::<T, _>()?,
    })
  }

  /// Write item data into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_xr::<T, _>(&self.base)?;

    Ok(())
  }
}

impl LtxImportExport for AlifeObjectItemDetector {
  /// Import ALife item object data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    Ok(Self {
      base: AlifeObjectItem::import(section_name, ltx)?,
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
  use crate::data::alife::inherited::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::inherited::alife_object_item::AlifeObjectItem;
  use crate::data::alife::inherited::alife_object_item_detector::AlifeObjectItemDetector;
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

    let original: AlifeObjectItemDetector = AlifeObjectItemDetector {
      base: AlifeObjectItem {
        base: AlifeObjectDynamicVisual {
          base: AlifeObjectAbstract {
            game_vertex_id: 1005,
            distance: 34.12,
            direct_control: 6627,
            level_vertex_id: 55313,
            flags: 45,
            custom_data: String::from("custom_data"),
            story_id: 475,
            spawn_story_id: 128,
          },
          visual_name: String::from("efg"),
          visual_flags: 33,
        },
        condition: 0.4,
        upgrades_count: 0,
      },
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 51);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 51);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 51 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectItemDetector::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let ltx_filename: String = get_relative_test_sample_file_path(file!(), "import_export.ltx");
    let mut ltx: Ltx = Ltx::new();

    let original: AlifeObjectItemDetector = AlifeObjectItemDetector {
      base: AlifeObjectItem {
        base: AlifeObjectDynamicVisual {
          base: AlifeObjectAbstract {
            game_vertex_id: 5,
            distance: 6.25,
            direct_control: 55,
            level_vertex_id: 56,
            flags: 31,
            custom_data: String::from("custom_data"),
            story_id: 51,
            spawn_story_id: 62,
          },
          visual_name: String::from("cdef"),
          visual_flags: 61,
        },
        condition: 0.23,
        upgrades_count: 3,
      },
    };

    original.export("data", &mut ltx)?;

    ltx.write_to(&mut overwrite_test_relative_resource_as_file(
      &ltx_filename,
    )?)?;

    let source: Ltx = Ltx::read_from_path(get_absolute_test_resource_path(&ltx_filename))?;

    assert_eq!(AlifeObjectItemDetector::import("data", &source)?, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> XRayResult {
    let original: AlifeObjectItemDetector = AlifeObjectItemDetector {
      base: AlifeObjectItem {
        base: AlifeObjectDynamicVisual {
          base: AlifeObjectAbstract {
            game_vertex_id: 11,
            distance: 22.25,
            direct_control: 33,
            level_vertex_id: 44,
            flags: 55,
            custom_data: String::from("custom_data"),
            story_id: 66,
            spawn_story_id: 77,
          },
          visual_name: String::from("cdef"),
          visual_flags: 5,
        },
        condition: 0.15,
        upgrades_count: 2,
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
      serde_json::from_str::<AlifeObjectItemDetector>(&serialized)?,
      original
    );

    Ok(())
  }
}
