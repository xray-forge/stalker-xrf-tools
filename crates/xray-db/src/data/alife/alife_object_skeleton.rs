use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::constants::FLAG_SKELETON_SAVED_DATA;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, SpawnByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectSkeleton {
  pub name: String,
  pub flags: u8,
  pub source_id: u16,
}

impl AlifeObjectInheritedReader<AlifeObjectSkeleton> for AlifeObjectSkeleton {
  /// Read skeleton data from the chunk reader.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<AlifeObjectSkeleton> {
    let object = AlifeObjectSkeleton {
      name: reader.read_null_terminated_win_string()?,
      flags: reader.read_u8()?,
      source_id: reader.read_u16::<SpawnByteOrder>()?,
    };

    if object.flags & FLAG_SKELETON_SAVED_DATA != 0 {
      todo!("Extend skeleton parsing to include bones")
    }

    Ok(object)
  }

  /// Import skeleton data from ini config section.
  fn import(section: &Section) -> DatabaseResult<AlifeObjectSkeleton> {
    Ok(AlifeObjectSkeleton {
      name: read_ini_field("name", section)?,
      flags: read_ini_field("flags", section)?,
      source_id: read_ini_field("source_id", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectSkeleton {
  /// Write skeleton data into the chunk writer.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    writer.write_null_terminated_win_string(&self.name)?;
    writer.write_u8(self.flags)?;
    writer.write_u16::<SpawnByteOrder>(self.source_id)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) {
    ini
      .with_section(section)
      .set("name", &self.name)
      .set("flags", self.flags.to_string())
      .set("source_id", self.source_id.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
  use crate::export::file::open_ini_config;
  use crate::types::{DatabaseResult, SpawnByteOrder};
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use xray_ltx::Ltx;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_resource_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let object: AlifeObjectSkeleton = AlifeObjectSkeleton {
      name: String::from("test-name"),
      flags: 33,
      source_id: 753,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 13);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 13);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 13 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectSkeleton =
      AlifeObjectSkeleton::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }

  #[test]
  fn test_import_export() -> DatabaseResult<()> {
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

    let exported_filename: String =
      get_relative_test_sample_file_path(file!(), "import_export.ini");
    let mut exported: Ltx = Ltx::new();

    first.export("first", &mut exported);
    second.export("second", &mut exported);

    exported.write_to(&mut overwrite_test_relative_resource_as_file(
      &exported_filename,
    )?)?;

    let source: Ltx = open_ini_config(&get_absolute_test_resource_path(&exported_filename))?;

    let read_first: AlifeObjectSkeleton =
      AlifeObjectSkeleton::import(source.section("first").unwrap())?;
    let read_second: AlifeObjectSkeleton =
      AlifeObjectSkeleton::import(source.section("second").unwrap())?;

    assert_eq!(read_first, first);
    assert_eq!(read_second, second);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> DatabaseResult<()> {
    let object: AlifeObjectSkeleton = AlifeObjectSkeleton {
      name: String::from("test-name-serde"),
      flags: 45,
      source_id: 34,
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(json!(object).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      object,
      serde_json::from_str::<AlifeObjectSkeleton>(&serialized).unwrap()
    );

    Ok(())
  }
}
