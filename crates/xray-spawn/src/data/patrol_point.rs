use crate::chunk::chunk::Chunk;
use crate::chunk::iterator::ChunkIterator;
use crate::chunk::writer::ChunkWriter;
use crate::types::Vector3d;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Debug)]
pub struct PatrolPoint {
  pub name: String,
  pub position: Vector3d,
  pub flags: u32,
  pub level_vertex_id: u32,
  pub game_vertex_id: u16,
}

impl PatrolPoint {
  /// Read points from chunk file.
  pub fn read_list_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<Vec<PatrolPoint>> {
    let mut points: Vec<PatrolPoint> = Vec::new();

    for (index, mut point_chunk) in ChunkIterator::new(chunk).enumerate() {
      let mut point_index_chunk: Chunk = point_chunk.read_child_by_index(0).unwrap();
      let mut point_points_chunk: Chunk = point_chunk.read_child_by_index(1).unwrap();

      assert_eq!(index, point_index_chunk.read_u32::<T>()? as usize);
      assert_eq!(point_index_chunk.read_bytes_remain(), 0);
      assert_eq!(point_chunk.read_bytes_remain(), 0);

      points.push(PatrolPoint::read_from_chunk::<T>(&mut point_points_chunk)?);
    }

    assert_eq!(
      chunk.read_bytes_remain(),
      0,
      "Chunk data should be read for patrol points list."
    );

    Ok(points)
  }

  /// Read patrol point data from chunk.
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<PatrolPoint> {
    let name: String = chunk.read_null_terminated_string().unwrap();
    let position: Vector3d = chunk.read_f32_3d_vector::<T>().unwrap();
    let flags: u32 = chunk.read_u32::<T>().unwrap();
    let level_vertex_id: u32 = chunk.read_u32::<T>().unwrap();
    let game_vertex_id: u16 = chunk.read_u16::<T>().unwrap();

    assert_eq!(
      chunk.read_bytes_remain(),
      0,
      "Chunk data should be read for patrol point."
    );

    Ok(PatrolPoint {
      name,
      position,
      flags,
      level_vertex_id,
      game_vertex_id,
    })
  }

  /// Write patrol point data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_null_terminated_string(&self.name)?;
    writer.write_f32_3d_vector::<T>(&self.position)?;
    writer.write_u32::<T>(self.flags)?;
    writer.write_u32::<T>(self.level_vertex_id)?;
    writer.write_u16::<T>(self.game_vertex_id)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::patrol_point::PatrolPoint;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;

  #[test]
  fn test_read_write_simple_patrol_point() {
    let mut writer: ChunkWriter = ChunkWriter::new();

    PatrolPoint {
      name: String::from("patrol-point-name"),
      position: (1.5, -2.3, 1.0),
      flags: 33,
      level_vertex_id: 4500,
      game_vertex_id: 555,
    }
    .write::<SpawnByteOrder>(&mut writer)
    .unwrap();

    assert_eq!(writer.bytes_written(), 40);

    let bytes_written: usize = writer
      .flush_chunk::<SpawnByteOrder>(
        &mut overwrite_test_resource_as_file(get_test_chunk_file_sub_dir(
          file!(),
          String::from("patrol_point_simple.chunk"),
        ))
        .unwrap(),
        0,
      )
      .unwrap();

    assert_eq!(bytes_written, 40);

    let file: FileSlice = open_test_resource_as_slice(get_test_chunk_file_sub_dir(
      file!(),
      String::from("patrol_point_simple.chunk"),
    ))
    .unwrap();

    assert_eq!(file.bytes_remaining(), 40 + 8);

    let mut chunk: Chunk = Chunk::from_file(file).unwrap().read_child_by_index(0)?;

    let point: PatrolPoint = PatrolPoint::read_from_chunk::<SpawnByteOrder>(&mut chunk).unwrap();

    assert_eq!(point.name, String::from("patrol-point-name"));
    assert_eq!(point.position.0, 1.5);
    assert_eq!(point.position.1, -2.3);
    assert_eq!(point.position.2, 1.0);
    assert_eq!(point.flags, 33);
    assert_eq!(point.level_vertex_id, 4500);
    assert_eq!(point.game_vertex_id, 555);
  }
}
