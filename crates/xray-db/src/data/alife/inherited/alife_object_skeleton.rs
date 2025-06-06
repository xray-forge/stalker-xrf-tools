use crate::constants::FLAG_SKELETON_SAVED_DATA;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter, XRayByteOrder};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectSkeleton {
  pub name: String,
  pub flags: u8,
  pub source_id: u16,
}

impl ChunkReadWrite for AlifeObjectSkeleton {
  /// Read skeleton data from the chunk reader.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let object = Self {
      name: reader.read_w1251_string()?,
      flags: reader.read_u8()?,
      source_id: reader.read_u16::<XRayByteOrder>()?,
    };

    if object.flags & FLAG_SKELETON_SAVED_DATA != 0 {
      todo!("Extend skeleton parsing to include bones based on flag")
    }

    Ok(object)
  }

  /// Write skeleton data into the chunk writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_w1251_string(&self.name)?;
    writer.write_u8(self.flags)?;
    writer.write_u16::<XRayByteOrder>(self.source_id)?;

    Ok(())
  }
}

impl LtxImportExport for AlifeObjectSkeleton {
  /// Import skeleton data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "ALife object '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      name: read_ltx_field("skeleton.name", section)?,
      flags: read_ltx_field("skeleton.flags", section)?,
      source_id: read_ltx_field("skeleton.source_id", section)?,
    })
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("skeleton.name", &self.name)
      .set("skeleton.flags", self.flags.to_string())
      .set("skeleton.source_id", self.source_id.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::inherited::alife_object_skeleton::AlifeObjectSkeleton;
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

    let original: AlifeObjectSkeleton = AlifeObjectSkeleton {
      name: String::from("test-name"),
      flags: 33,
      source_id: 753,
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 13);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 13);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 13 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectSkeleton = AlifeObjectSkeleton::read::<XRayByteOrder>(&mut reader)?;

    assert_eq!(read_object, original);

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let ltx_filename: String = get_relative_test_sample_file_path(file!(), "import_export.ltx");
    let mut ltx: Ltx = Ltx::new();

    let first: AlifeObjectSkeleton = AlifeObjectSkeleton {
      name: String::from("test-name-first"),
      flags: 33,
      source_id: 753,
    };

    let second: AlifeObjectSkeleton = AlifeObjectSkeleton {
      name: String::from("test-name-second"),
      flags: 54,
      source_id: 526,
    };

    first.export("first", &mut ltx)?;
    second.export("second", &mut ltx)?;

    ltx.write_to(&mut overwrite_test_relative_resource_as_file(
      &ltx_filename,
    )?)?;

    let source: Ltx = Ltx::read_from_path(get_absolute_test_resource_path(&ltx_filename))?;

    assert_eq!(AlifeObjectSkeleton::import("first", &source)?, first);
    assert_eq!(AlifeObjectSkeleton::import("second", &source)?, second);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> XRayResult {
    let original: AlifeObjectSkeleton = AlifeObjectSkeleton {
      name: String::from("test-name-serde"),
      flags: 45,
      source_id: 34,
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
      serde_json::from_str::<AlifeObjectSkeleton>(&serialized)?
    );

    Ok(())
  }
}
