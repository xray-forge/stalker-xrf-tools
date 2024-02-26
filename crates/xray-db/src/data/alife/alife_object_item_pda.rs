use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectItemPda {
  pub base: AlifeObjectItem,
  pub owner: u16,
  pub character: String,
  pub info_portion: String,
}

impl AlifeObjectInheritedReader<AlifeObjectItemPda> for AlifeObjectItemPda {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectItemPda> {
    let base: AlifeObjectItem = AlifeObjectItem::read_from_chunk::<T>(chunk)?;

    let owner: u16 = chunk.read_u16::<SpawnByteOrder>()?;
    let character: String = chunk.read_null_terminated_string()?;
    let info_portion: String = chunk.read_null_terminated_string()?;

    Ok(AlifeObjectItemPda {
      base,
      owner,
      character,
      info_portion,
    })
  }
}

impl AlifeObjectGeneric for AlifeObjectItemPda {
  type Order = SpawnByteOrder;

  /// Write item data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_u16::<Self::Order>(self.owner)?;
    writer.write_null_terminated_string(&self.character)?;
    writer.write_null_terminated_string(&self.info_portion)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_item::AlifeObjectItem;
  use crate::data::alife::alife_object_item_pda::AlifeObjectItemPda;
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
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_object_item_pda.chunk"));

    let object: AlifeObjectItemPda = AlifeObjectItemPda {
      base: AlifeObjectItem {
        base: AlifeObjectDynamicVisual {
          base: AlifeObjectAbstract {
            game_vertex_id: 1011,
            distance: 35.2511,
            direct_control: 835,
            level_vertex_id: 3381,
            flags: 45,
            custom_data: String::from("custom_data"),
            story_id: 513,
            spawn_story_id: 35,
          },
          visual_name: String::from("zxcvb"),
          visual_flags: 43,
        },
        condition: 0.15,
        upgrades_count: 0,
      },
      owner: 0,
      character: String::from("character"),
      info_portion: String::from("info-portion"),
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 78);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 78);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 78 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectItemPda =
      AlifeObjectItemPda::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
