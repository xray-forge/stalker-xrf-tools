use crate::chunk::chunk::Chunk;
use crate::chunk::iterator::ChunkIterator;
use crate::chunk::writer::ChunkWriter;
use crate::data::vector_3d::Vector3d;
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use std::io;
use std::io::Write;

/// `CPatrolPoint::load_raw`, `CPatrolPoint::load` in xray codebase.
#[derive(Clone, Debug, PartialEq)]
pub struct PatrolPoint {
  pub name: String,
  pub position: Vector3d<f32>,
  pub flags: u32,
  pub level_vertex_id: u32,
  pub game_vertex_id: u16,
}

impl PatrolPoint {
  /// Read points from chunk file.
  pub fn read_list_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<Vec<PatrolPoint>> {
    let mut points: Vec<PatrolPoint> = Vec::new();

    for (index, mut point_chunk) in ChunkIterator::new(chunk).enumerate() {
      let mut index_chunk: Chunk = point_chunk.read_child_by_index(0)?;
      let mut points_chunk: Chunk = point_chunk.read_child_by_index(1)?;

      assert_eq!(index, index_chunk.read_u32::<T>()? as usize);

      points.push(PatrolPoint::read_from_chunk::<T>(&mut points_chunk)?);

      assert!(index_chunk.is_ended());
      assert!(point_chunk.is_ended());
    }

    assert!(
      chunk.is_ended(),
      "Chunk data should be read for patrol points list"
    );

    Ok(points)
  }

  /// Read patrol point data from chunk.
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<PatrolPoint> {
    let point: PatrolPoint = PatrolPoint {
      name: chunk.read_null_terminated_win_string()?,
      position: chunk.read_f32_3d_vector::<T>()?,
      flags: chunk.read_u32::<T>()?,
      level_vertex_id: chunk.read_u32::<T>()?,
      game_vertex_id: chunk.read_u16::<T>()?,
    };

    assert!(
      chunk.is_ended(),
      "Chunk data should be read for patrol point"
    );

    Ok(point)
  }

  /// Write list of patrol points into chunk writer.
  pub fn write_list<T: ByteOrder>(
    points: &Vec<PatrolPoint>,
    writer: &mut ChunkWriter,
  ) -> io::Result<()> {
    for (index, point) in points.iter().enumerate() {
      let mut point_chunk_writer: ChunkWriter = ChunkWriter::new();

      let mut point_index_writer: ChunkWriter = ChunkWriter::new();
      let mut point_writer: ChunkWriter = ChunkWriter::new();

      point_index_writer.write_u32::<T>(index as u32)?;
      point.write::<T>(&mut point_writer)?;

      point_chunk_writer.write_all(&point_index_writer.flush_chunk_into_buffer::<T>(0)?)?;
      point_chunk_writer.write_all(&point_writer.flush_chunk_into_buffer::<T>(1)?)?;

      writer.write_all(&point_chunk_writer.flush_chunk_into_buffer::<T>(index)?)?;
    }

    Ok(())
  }

  /// Write patrol point data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_null_terminated_win_string(&self.name)?;
    writer.write_f32_3d_vector::<T>(&self.position)?;
    writer.write_u32::<T>(self.flags)?;
    writer.write_u32::<T>(self.level_vertex_id)?;
    writer.write_u16::<T>(self.game_vertex_id)?;

    Ok(())
  }

  /// Import patrol point data from ini config.
  pub fn import(section: &str, config: &Ini) -> io::Result<PatrolPoint> {
    let props: &Properties = config
      .section(Some(section))
      .unwrap_or_else(|| panic!("Patrol point section {section} should be defined in ltx file"));

    Ok(PatrolPoint {
      name: read_ini_field("name", props)?,
      position: read_ini_field("position", props)?,
      flags: read_ini_field("flags", props)?,
      level_vertex_id: read_ini_field("level_vertex_id", props)?,
      game_vertex_id: read_ini_field("game_vertex_id", props)?,
    })
  }

  /// Export patrol point data into ini.
  pub fn export(&self, section: &String, config: &mut Ini) {
    config
      .with_section(Some(section))
      .set("name", &self.name)
      .set("flags", self.flags.to_string())
      .set("position", self.position.to_string())
      .set("level_vertex_id", self.level_vertex_id.to_string())
      .set("game_vertex_id", self.game_vertex_id.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::patrol::patrol_point::PatrolPoint;
  use crate::data::vector_3d::Vector3d;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_simple_patrol_point() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_test_chunk_file_sub_dir(file!(), &String::from("patrol_point_simple.chunk"));

    let point: PatrolPoint = PatrolPoint {
      name: String::from("patrol-point-name"),
      position: Vector3d::new(1.5, -2.3, 1.0),
      flags: 33,
      level_vertex_id: 4500,
      game_vertex_id: 555,
    };

    point.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 40);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 40);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 40 + 8);

    let mut chunk: Chunk = Chunk::from_slice(file)?.read_child_by_index(0)?;

    let read_point: PatrolPoint = PatrolPoint::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_point, point);

    Ok(())
  }

  #[test]
  fn test_read_write_list_of_patrol_points() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_test_chunk_file_sub_dir(file!(), &String::from("patrol_point_list.chunk"));

    let points: Vec<PatrolPoint> = vec![
      PatrolPoint {
        name: String::from("patrol-point-name-1"),
        position: Vector3d::new(1.5, -2.3, 1.0),
        flags: 33,
        level_vertex_id: 7304,
        game_vertex_id: 55,
      },
      PatrolPoint {
        name: String::from("patrol-point-name-2"),
        position: Vector3d::new(2.25, 4.3, 1.5),
        flags: 64,
        level_vertex_id: 8415,
        game_vertex_id: 66,
      },
    ];

    PatrolPoint::write_list::<SpawnByteOrder>(&points, &mut writer)?;

    assert_eq!(writer.bytes_written(), 140);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 140);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 140 + 8);

    let mut chunk: Chunk = Chunk::from_slice(file)?.read_child_by_index(0)?;

    let read_points: Vec<PatrolPoint> =
      PatrolPoint::read_list_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(points, read_points);

    Ok(())
  }
}
