use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::constants::FLAG_SKELETON_SAVED_DATA;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::export::file_import::read_ini_field;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeObjectSkeleton {
  #[serde(rename = "name")]
  pub name: String,
  #[serde(rename = "flags")]
  pub flags: u8,
  #[serde(rename = "sourceId")]
  pub source_id: u16,
}

impl AlifeObjectInheritedReader<AlifeObjectSkeleton> for AlifeObjectSkeleton {
  /// Read skeleton data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectSkeleton> {
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
  fn import(section: &Section) -> io::Result<AlifeObjectSkeleton> {
    Ok(AlifeObjectSkeleton {
      name: read_ini_field("name", section)?,
      flags: read_ini_field("flags", section)?,
      source_id: read_ini_field("source_id", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectSkeleton {
  /// Write skeleton data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
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
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_relative_test_sample_file_path(file!(), "alife_object_skeleton.chunk");

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
}
