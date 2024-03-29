use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io;
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectItemHelmet {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInheritedReader<AlifeObjectItemHelmet> for AlifeObjectItemHelmet {
  /// Read alife item object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectItemHelmet> {
    Ok(AlifeObjectItemHelmet {
      base: AlifeObjectItem::read::<T>(reader)?,
    })
  }

  /// Import alife item object data from ini config section.
  fn import(section: &Section) -> io::Result<AlifeObjectItemHelmet> {
    Ok(AlifeObjectItemHelmet {
      base: AlifeObjectItem::import(section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectItemHelmet {
  /// Write item data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) {
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
  use crate::data::alife::alife_object_item_helmet::AlifeObjectItemHelmet;
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
      get_relative_test_sample_file_path(file!(), "alife_object_item_helmet.chunk");

    let object: AlifeObjectItemHelmet = AlifeObjectItemHelmet {
      base: AlifeObjectItem {
        base: AlifeObjectDynamicVisual {
          base: AlifeObjectAbstract {
            game_vertex_id: 1027,
            distance: 11.2,
            direct_control: 664,
            level_vertex_id: 823422,
            flags: 46,
            custom_data: String::from("custom_data"),
            story_id: 515,
            spawn_story_id: 33,
          },
          visual_name: String::from("abcd"),
          visual_flags: 82,
        },
        condition: 0.32,
        upgrades_count: 0,
      },
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 52);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 52);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 52 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectItemHelmet =
      AlifeObjectItemHelmet::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
