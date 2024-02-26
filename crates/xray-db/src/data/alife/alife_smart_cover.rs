use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_smart_cover::AlifeObjectSmartCover;
use crate::data::alife::alife_smart_cover_loophole::AlifeSmartCoverLoophole;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

/// Represents script extension of base server smart cover class.
#[derive(Clone, Debug, PartialEq)]
pub struct AlifeSmartCover {
  pub base: AlifeObjectSmartCover,
  pub last_description: String,
  pub loopholes: Vec<AlifeSmartCoverLoophole>,
}

impl AlifeObjectInheritedReader<AlifeSmartCover> for AlifeSmartCover {
  /// Read smart cover data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeSmartCover> {
    let base: AlifeObjectSmartCover = AlifeObjectSmartCover::read_from_chunk::<T>(chunk)?;

    let last_description: String = chunk.read_null_terminated_win_string()?;
    let count: u8 = chunk.read_u8()?;
    let mut loopholes: Vec<AlifeSmartCoverLoophole> = Vec::new();

    for _ in 0..count {
      let name: String = chunk.read_null_terminated_win_string()?;
      let enabled: u8 = chunk.read_u8()?;

      loopholes.push(AlifeSmartCoverLoophole { name, enabled })
    }

    Ok(AlifeSmartCover {
      base,
      last_description,
      loopholes,
    })
  }
}

impl AlifeObjectGeneric for AlifeSmartCover {
  type Order = SpawnByteOrder;

  /// Write smart cover data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_null_terminated_win_string(&self.last_description)?;
    writer.write_u8(self.loopholes.len() as u8)?;

    for loophole in &self.loopholes {
      writer.write_null_terminated_win_string(&loophole.name)?;
      writer.write_u8(loophole.enabled)?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic::AlifeObjectDynamic;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_smart_cover::AlifeObjectSmartCover;
  use crate::data::shape::Shape;
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
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_smart_cover.chunk"));

    let object: AlifeObjectSmartCover = AlifeObjectSmartCover {
      base: AlifeObjectDynamic {
        base: AlifeObjectAbstract {
          game_vertex_id: 6734,
          distance: 38.287,
          direct_control: 234760,
          level_vertex_id: 29836,
          flags: 68,
          custom_data: String::from("custom-data"),
          story_id: 8723,
          spawn_story_id: 160278,
        },
      },
      shape: vec![
        Shape::Sphere(((2.5, 1.3, -4.125), 5.5)),
        Shape::Box((
          (1.1, 1.1, 6.1),
          (1.4, 2.2, 6.3),
          (4.0, 3.0, 6.4),
          (9.2, 8.3, 6.0),
        )),
      ],
      description: String::from("description"),
      hold_position_time: 34.0,
      enter_min_enemy_distance: 23.0,
      exit_min_enemy_distance: 36.0,
      is_combat_cover: 1,
      can_fire: 1,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 131);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 131);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 131 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectSmartCover =
      AlifeObjectSmartCover::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
