use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::meta::alife_object_generic::AlifeObjectGeneric;
use crate::data::meta::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::export::file_import::read_ini_field;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectMotion {
  pub motion_name: String,
}

impl AlifeObjectInheritedReader<AlifeObjectMotion> for AlifeObjectMotion {
  /// Read motion object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      motion_name: reader.read_null_terminated_win_string()?,
    })
  }

  /// Import motion object data from ini config section.
  fn import(section: &Section) -> DatabaseResult<Self> {
    Ok(Self {
      motion_name: read_ini_field("motion_name", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectMotion {
  /// Write motion object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    writer.write_null_terminated_win_string(&self.motion_name)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) {
    ini
      .with_section(section)
      .set("motion_name", &self.motion_name);
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_motion::AlifeObjectMotion;
  use crate::data::meta::alife_object_generic::AlifeObjectGeneric;
  use crate::data::meta::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::types::{DatabaseResult, SpawnByteOrder};
  use fileslice::FileSlice;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: AlifeObjectMotion = AlifeObjectMotion {
      motion_name: String::from("motion-name"),
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 12);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 12);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 12 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectMotion::read::<SpawnByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
