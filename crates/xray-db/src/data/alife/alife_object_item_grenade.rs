use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use byteorder::ByteOrder;
use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeObjectItemGrenade {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInheritedReader<AlifeObjectItemGrenade> for AlifeObjectItemGrenade {
  /// Read alife item object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectItemGrenade> {
    Ok(AlifeObjectItemGrenade {
      base: AlifeObjectItem::read::<T>(reader)?,
    })
  }

  /// Import alife object data from ini config file section.
  fn import(props: &Properties) -> io::Result<AlifeObjectItemGrenade> {
    Ok(AlifeObjectItemGrenade {
      base: AlifeObjectItem::import(props)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectItemGrenade {
  /// Write item data into the writer.
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
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_item::AlifeObjectItem;
  use crate::data::alife::alife_object_item_grenade::AlifeObjectItemGrenade;
  use crate::test::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_relative_test_sample_file_path(file!(), "alife_object_item_grenade.chunk");

    let object: AlifeObjectItemGrenade = AlifeObjectItemGrenade {
      base: AlifeObjectItem {
        base: AlifeObjectDynamicVisual {
          base: AlifeObjectAbstract {
            game_vertex_id: 1023,
            distance: 83.3,
            direct_control: 624,
            level_vertex_id: 26263,
            flags: 123,
            custom_data: String::from("custom_data"),
            story_id: 43,
            spawn_story_id: 111,
          },
          visual_name: String::from("abdef"),
          visual_flags: 77,
        },
        condition: 0.64,
        upgrades_count: 0,
      },
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 53);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 53);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 53 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectItemGrenade =
      AlifeObjectItemGrenade::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
