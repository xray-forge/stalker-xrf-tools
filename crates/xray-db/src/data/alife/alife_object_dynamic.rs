use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use byteorder::ByteOrder;
use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeObjectDynamic {
  pub base: AlifeObjectAbstract,
}

impl AlifeObjectInheritedReader<AlifeObjectDynamic> for AlifeObjectDynamic {
  /// Read dynamic object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectDynamic> {
    Ok(AlifeObjectDynamic {
      base: AlifeObjectAbstract::read::<T>(reader)?,
    })
  }

  /// Import dynamic object data from ini config section.
  fn import(props: &Properties) -> io::Result<AlifeObjectDynamic> {
    Ok(AlifeObjectDynamic {
      base: AlifeObjectAbstract::import(props)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectDynamic {
  /// Write dynamic object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ini) {
    self.base.export(section, ini);
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic::AlifeObjectDynamic;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::test::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_relative_test_sample_file_path(file!(), "alife_object_dynamic.chunk");

    let object: AlifeObjectDynamic = AlifeObjectDynamic {
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

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 38);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 38);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 38 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectDynamic = AlifeObjectDynamic::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
