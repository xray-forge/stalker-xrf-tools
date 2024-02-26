use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectInventoryBox {
  pub base: AlifeObjectVisual,
  pub can_take: u8,
  pub is_closed: u8,
  pub tip: String,
}

impl AlifeObjectInheritedReader<AlifeObjectInventoryBox> for AlifeObjectInventoryBox {
  /// Read inventory object data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectInventoryBox> {
    let base: AlifeObjectVisual = AlifeObjectVisual::read_from_chunk::<T>(chunk)?;

    let can_take: u8 = chunk.read_u8()?;
    let is_closed: u8 = chunk.read_u8()?;
    let tip: String = chunk.read_null_terminated_string()?;

    Ok(AlifeObjectInventoryBox {
      base,
      can_take,
      is_closed,
      tip,
    })
  }
}

impl AlifeObjectGeneric for AlifeObjectInventoryBox {
  type Order = SpawnByteOrder;

  /// Write inventory object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_u8(self.can_take)?;
    writer.write_u8(self.is_closed)?;
    writer.write_null_terminated_string(&self.tip)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_inventory_box::AlifeObjectInventoryBox;
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
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_object_inventory_box.chunk"));

    let object: AlifeObjectInventoryBox = AlifeObjectInventoryBox {
      base: AlifeObjectVisual {
        base: AlifeObjectAbstract {
          game_vertex_id: 0,
          distance: 0.0,
          direct_control: 0,
          level_vertex_id: 0,
          flags: 0,
          custom_data: "".to_string(),
          story_id: 0,
          spawn_story_id: 0,
        },
        visual_name: "".to_string(),
        visual_flags: 0,
      },
      can_take: 0,
      is_closed: 0,
      tip: "".to_string(),
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 32);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 32);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 32 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectInventoryBox =
      AlifeObjectInventoryBox::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
