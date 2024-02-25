use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectItemAmmo {
  pub base: AlifeObjectItem,
  pub ammo_left: u16,
}

impl AlifeObjectInheritedReader<AlifeObjectItemAmmo> for AlifeObjectItemAmmo {
  /// Read alife item object data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectItemAmmo> {
    let base: AlifeObjectItem = AlifeObjectItem::read_from_chunk::<T>(chunk)?;

    let ammo_left: u16 = chunk.read_u16::<SpawnByteOrder>()?;

    Ok(AlifeObjectItemAmmo { base, ammo_left })
  }

  /// Write item data into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write::<T>(writer)?;

    writer.write_u16::<T>(self.ammo_left)?;

    Ok(())
  }
}

impl AlifeObjectGeneric for AlifeObjectItemAmmo {}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_item::AlifeObjectItem;
  use crate::data::alife::alife_object_item_ammo::AlifeObjectItemAmmo;
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
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_object_item_ammo.chunk"));

    let object: AlifeObjectItemAmmo = AlifeObjectItemAmmo {
      base: AlifeObjectItem {
        base: AlifeObjectVisual {
          base: AlifeObjectAbstract {
            game_vertex_id: 1003,
            distance: 65.25,
            direct_control: 412242,
            level_vertex_id: 61853223,
            flags: 40,
            custom_data: String::from("custom_data"),
            story_id: 512,
            spawn_story_id: 33,
          },
          visual_name: String::from("cdef"),
          visual_flags: 33,
        },
        condition: 0.6,
        upgrades_count: 0,
      },
      ammo_left: 12,
    };

    object.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 54);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 54);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 54 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectItemAmmo =
      AlifeObjectItemAmmo::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}