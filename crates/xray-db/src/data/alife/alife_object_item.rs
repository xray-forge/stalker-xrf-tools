use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectItem {
  pub base: AlifeObjectVisual,
  pub condition: f32,
  pub upgrades_count: u32,
}

impl AlifeObjectInheritedReader<AlifeObjectItem> for AlifeObjectItem {
  /// Read alife item object data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectItem> {
    let base: AlifeObjectVisual = AlifeObjectVisual::read_from_chunk::<T>(chunk)?;

    let condition: f32 = chunk.read_f32::<SpawnByteOrder>()?;
    let upgrades_count: u32 = chunk.read_u32::<SpawnByteOrder>()?;

    assert_eq!(upgrades_count, 0, "Unexpected upgraded item provided.");

    Ok(AlifeObjectItem {
      base,
      condition,
      upgrades_count,
    })
  }

  /// Write item data into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write::<T>(writer)?;

    writer.write_f32::<T>(self.condition)?;
    writer.write_u32::<T>(self.upgrades_count)?;

    Ok(())
  }
}

impl AlifeObjectGeneric for AlifeObjectItem {}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_item::AlifeObjectItem;
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
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_object_item.chunk"));

    let object: AlifeObjectItem = AlifeObjectItem {
      base: AlifeObjectVisual {
        base: AlifeObjectAbstract {
          game_vertex_id: 1002,
          distance: 65.25,
          direct_control: 41262,
          level_vertex_id: 618923,
          flags: 32,
          custom_data: String::from("custom_data"),
          story_id: 500,
          spawn_story_id: 35,
        },
        visual_name: String::from("abcd"),
        visual_flags: 33,
      },
      condition: 0.5,
      upgrades_count: 0,
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
    let read_object: AlifeObjectItem =
      AlifeObjectItem::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
