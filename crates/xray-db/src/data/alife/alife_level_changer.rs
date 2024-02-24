use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
use crate::types::{SpawnByteOrder, Vector3d};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeLevelChanger {
  pub base: AlifeObjectSpaceRestrictor,
  pub dest_game_vertex_id: u16,
  pub dest_level_vertex_id: u32,
  pub dest_position: Vector3d,
  pub dest_direction: Vector3d,
  pub angle_y: f32,
  pub dest_level_name: String,
  pub dest_graph_point: String,
  pub silent_mode: u8,
  pub enabled: u8,
  pub hint: String,
  pub save_marker: u16,
}

impl AlifeObjectInheritedReader<AlifeLevelChanger> for AlifeLevelChanger {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeLevelChanger> {
    let base: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor::read_from_chunk::<T>(chunk)?;

    let dest_game_vertex_id: u16 = chunk.read_u16::<SpawnByteOrder>()?;
    let dest_level_vertex_id: u32 = chunk.read_u32::<SpawnByteOrder>()?;
    let dest_position: Vector3d = chunk.read_f32_3d_vector::<SpawnByteOrder>()?;
    let dest_direction: Vector3d = chunk.read_f32_3d_vector::<SpawnByteOrder>()?;
    let angle_y: f32 = chunk.read_f32::<SpawnByteOrder>()?;
    let dest_level_name: String = chunk.read_null_terminated_string()?;
    let dest_graph_point: String = chunk.read_null_terminated_string()?;
    let silent_mode: u8 = chunk.read_u8()?;

    let enabled: u8 = chunk.read_u8()?;
    let hint: String = chunk.read_null_terminated_string()?;
    let save_marker: u16 = chunk.read_u16::<SpawnByteOrder>()?;

    assert_eq!(
      save_marker, 26,
      "Unexpected script data provided for level changer."
    );

    Ok(AlifeLevelChanger {
      base,
      dest_game_vertex_id,
      dest_level_vertex_id,
      dest_position,
      dest_direction,
      angle_y,
      dest_level_name,
      dest_graph_point,
      silent_mode,
      enabled,
      hint,
      save_marker,
    })
  }

  /// Write object data into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write::<T>(writer)?;

    writer.write_u16::<T>(self.dest_game_vertex_id)?;
    writer.write_u32::<T>(self.dest_level_vertex_id)?;
    writer.write_f32_3d_vector::<T>(&self.dest_position)?;
    writer.write_f32_3d_vector::<T>(&self.dest_direction)?;
    writer.write_f32::<T>(self.angle_y)?;
    writer.write_null_terminated_string(&self.dest_level_name)?;
    writer.write_null_terminated_string(&self.dest_graph_point)?;
    writer.write_u8(self.silent_mode)?;

    writer.write_u8(self.enabled)?;
    writer.write_null_terminated_string(&self.hint)?;
    writer.write_u16::<T>(self.save_marker)?;

    Ok(())
  }
}

impl AlifeObjectGeneric for AlifeLevelChanger {}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_level_changer::AlifeLevelChanger;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
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
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_level_changer.chunk"));

    let object: AlifeLevelChanger = AlifeLevelChanger {
      base: AlifeObjectSpaceRestrictor {
        base: AlifeObjectAbstract {
          game_vertex_id: 12451,
          distance: 253.0,
          direct_control: 12,
          level_vertex_id: 331,
          flags: 33,
          custom_data: String::from("custom-data"),
          story_id: 4553,
          spawn_story_id: 213,
        },
        shape: vec![
          Shape::Sphere(((4.5, 0.5, 11.5), 1.0)),
          Shape::Box((
            (1.5, 2.5, 73.1),
            (5.1, 3.2, 2.3),
            (1.0, 3.0, 6.4),
            (9.2, 3.3, 3.0),
          )),
        ],
        restrictor_type: 3,
      },
      dest_game_vertex_id: 312,
      dest_level_vertex_id: 3312,
      dest_position: (4.0, 3.0, 2.0),
      dest_direction: (1.0, 2.0, 3.0),
      angle_y: 35.0,
      dest_level_name: String::from("dest-level"),
      dest_graph_point: String::from("dest-graph-point"),
      silent_mode: 1,
      enabled: 1,
      hint: String::from("hint"),
      save_marker: 26,
    };

    object.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 177);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 177);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 177 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeLevelChanger =
      AlifeLevelChanger::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
