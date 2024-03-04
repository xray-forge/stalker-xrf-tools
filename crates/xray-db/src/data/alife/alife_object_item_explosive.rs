use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io;
use xray_ltx::{Ltx, Properties};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeObjectItemExplosive {
  #[serde(rename = "base")]
  pub base: AlifeObjectItem,
}

impl AlifeObjectInheritedReader<AlifeObjectItemExplosive> for AlifeObjectItemExplosive {
  /// Read alife item object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectItemExplosive> {
    Ok(AlifeObjectItemExplosive {
      base: AlifeObjectItem::read::<T>(reader)?,
    })
  }

  fn import(props: &Properties) -> io::Result<AlifeObjectItemExplosive> {
    Ok(AlifeObjectItemExplosive {
      base: AlifeObjectItem::import(props)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectItemExplosive {
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
  use crate::data::alife::alife_object_item_explosive::AlifeObjectItemExplosive;
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
      get_relative_test_sample_file_path(file!(), "alife_object_item_explosive.chunk");

    let object: AlifeObjectItemExplosive = AlifeObjectItemExplosive {
      base: AlifeObjectItem {
        base: AlifeObjectDynamicVisual {
          base: AlifeObjectAbstract {
            game_vertex_id: 1012,
            distance: 31.2,
            direct_control: 482,
            level_vertex_id: 55113322,
            flags: 44,
            custom_data: String::from("custom_data"),
            story_id: 514,
            spawn_story_id: 34,
          },
          visual_name: String::from("ab"),
          visual_flags: 65,
        },
        condition: 1.0,
        upgrades_count: 0,
      },
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 50);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 50);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 50 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectItemExplosive =
      AlifeObjectItemExplosive::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
