use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item::AlifeObjectItem;
use byteorder::ByteOrder;
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectItemHelmet {
  pub base: AlifeObjectItem,
}

impl AlifeObjectInheritedReader<AlifeObjectItemHelmet> for AlifeObjectItemHelmet {
  /// Read alife item object data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectItemHelmet> {
    let base: AlifeObjectItem = AlifeObjectItem::read_from_chunk::<T>(chunk)?;

    Ok(AlifeObjectItemHelmet { base })
  }

  /// Write item data into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write::<T>(writer)?;

    Ok(())
  }
}

impl AlifeObjectGeneric for AlifeObjectItemHelmet {}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_item::AlifeObjectItem;
  use crate::data::alife::alife_object_item_helmet::AlifeObjectItemHelmet;
  use crate::data::alife::alife_object_visual::AlifeObjectVisual;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_object_item_helmet.chunk"));

    let object: AlifeObjectItemHelmet = AlifeObjectItemHelmet {
      base: AlifeObjectItem {
        base: AlifeObjectVisual {
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

    object.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 52);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 52);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 52 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectItemHelmet =
      AlifeObjectItemHelmet::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}