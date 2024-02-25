use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_dynamic::AlifeObjectDynamic;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::shape::Shape;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectSmartCover {
  pub base: AlifeObjectDynamic,
  pub shape: Vec<Shape>,
  pub description: String,
  pub hold_position_time: f32,
  pub enter_min_enemy_distance: f32,
  pub exit_min_enemy_distance: f32,
  pub is_combat_cover: u8,
  pub can_fire: u8,
}

impl AlifeObjectInheritedReader<AlifeObjectSmartCover> for AlifeObjectSmartCover {
  /// Read smart cover object data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectSmartCover> {
    let base: AlifeObjectDynamic = AlifeObjectDynamic::read_from_chunk::<T>(chunk)?;

    let shape: Vec<Shape> = chunk.read_shape_description::<SpawnByteOrder>()?;
    let description: String = chunk.read_null_terminated_string()?;
    let hold_position_time: f32 = chunk.read_f32::<SpawnByteOrder>()?;
    let enter_min_enemy_distance: f32 = chunk.read_f32::<SpawnByteOrder>()?;
    let exit_min_enemy_distance: f32 = chunk.read_f32::<SpawnByteOrder>()?;
    let is_combat_cover: u8 = chunk.read_u8()?;
    let can_fire: u8 = chunk.read_u8()?;

    Ok(AlifeObjectSmartCover {
      base,
      shape,
      description,
      hold_position_time,
      enter_min_enemy_distance,
      exit_min_enemy_distance,
      is_combat_cover,
      can_fire,
    })
  }

  /// Write smart cover object data into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write::<T>(writer)?;

    writer.write_shape_description::<T>(&self.shape)?;
    writer.write_null_terminated_string(&self.description)?;
    writer.write_f32::<T>(self.hold_position_time)?;
    writer.write_f32::<T>(self.enter_min_enemy_distance)?;
    writer.write_f32::<T>(self.exit_min_enemy_distance)?;
    writer.write_u8(self.is_combat_cover)?;
    writer.write_u8(self.can_fire)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic::AlifeObjectDynamic;
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
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_object_smart_cover.chunk"));

    let object: AlifeObjectSmartCover = AlifeObjectSmartCover {
      base: AlifeObjectDynamic {
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
      },
      shape: vec![
        Shape::Sphere(((0.5, 0.3, -0.125), 2.5)),
        Shape::Box((
          (1.1, 1.1, 3.1),
          (1.4, 2.2, 3.3),
          (4.0, 3.0, 5.4),
          (9.2, 8.3, 2.0),
        )),
      ],
      description: String::from("test-description"),
      hold_position_time: 4.532,
      enter_min_enemy_distance: 32.4,
      exit_min_enemy_distance: 25.3,
      is_combat_cover: 0,
      can_fire: 1,
    };

    object.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 136);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 136);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 136 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectSmartCover =
      AlifeObjectSmartCover::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}