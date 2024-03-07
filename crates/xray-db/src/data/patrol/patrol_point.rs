use crate::chunk::iterator::ChunkIterator;
use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::vector_3d::Vector3d;
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::Write;
use xray_ltx::{Ltx, Properties};

/// `CPatrolPoint::load_raw`, `CPatrolPoint::load` in xray codebase.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatrolPoint {
  #[serde(rename = "name")]
  pub name: String,
  #[serde(rename = "position")]
  pub position: Vector3d<f32>,
  #[serde(rename = "flags")]
  pub flags: u32,
  #[serde(rename = "levelVertexId")]
  pub level_vertex_id: u32,
  #[serde(rename = "gameVertexId")]
  pub game_vertex_id: u16,
}

impl PatrolPoint {
  /// Read points from chunk file.
  pub fn read_list<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<Vec<PatrolPoint>> {
    let mut points: Vec<PatrolPoint> = Vec::new();

    for (index, mut point_reader) in ChunkIterator::new(reader).enumerate() {
      let mut index_reader: ChunkReader = point_reader.read_child_by_index(0)?;
      let mut points_reader: ChunkReader = point_reader.read_child_by_index(1)?;

      assert_eq!(index, index_reader.read_u32::<T>()? as usize);

      points.push(PatrolPoint::read::<T>(&mut points_reader)?);

      assert!(index_reader.is_ended());
      assert!(point_reader.is_ended());
    }

    assert!(
      reader.is_ended(),
      "Chunk data should be read for patrol points list"
    );

    Ok(points)
  }

  /// Read patrol point data from chunk.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<PatrolPoint> {
    let point: PatrolPoint = PatrolPoint {
      name: reader.read_null_terminated_win_string()?,
      position: reader.read_f32_3d_vector::<T>()?,
      flags: reader.read_u32::<T>()?,
      level_vertex_id: reader.read_u32::<T>()?,
      game_vertex_id: reader.read_u16::<T>()?,
    };

    assert!(
      reader.is_ended(),
      "Chunk data should be read for patrol point"
    );

    Ok(point)
  }

  /// Write list of patrol points into chunk writer.
  pub fn write_list<T: ByteOrder>(
    points: &[PatrolPoint],
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
  pub fn import(section: &str, config: &Ltx) -> io::Result<PatrolPoint> {
    let props: &Properties = config
      .section(section)
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
  pub fn export(&self, section: &str, config: &mut Ltx) {
    config
      .with_section(section)
      .set("name", &self.name)
      .set("flags", self.flags.to_string())
      .set("position", self.position.to_string())
      .set("level_vertex_id", self.level_vertex_id.to_string())
      .set("game_vertex_id", self.game_vertex_id.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::patrol::patrol_point::PatrolPoint;
  use crate::data::vector_3d::Vector3d;
  use crate::export::file::open_ini_config;
  use crate::test::file::read_file_as_string;
  use crate::test::utils::{
    get_absolute_test_sample_file_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_file, overwrite_test_relative_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io;
  use std::io::{Seek, SeekFrom, Write};
  use std::path::Path;
  use xray_ltx::Ltx;

  #[test]
  fn test_read_write_simple_patrol_point() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "patrol_point_simple.chunk");

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
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 40);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 40 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    let read_point: PatrolPoint = PatrolPoint::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_point, point);

    Ok(())
  }

  #[test]
  fn test_read_write_list_of_patrol_points() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "patrol_point_list.chunk");

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
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 140);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 140 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    let read_points: Vec<PatrolPoint> = PatrolPoint::read_list::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(points, read_points);

    Ok(())
  }

  #[test]
  fn test_import_export_object() -> io::Result<()> {
    let point: PatrolPoint = PatrolPoint {
      name: String::from("patrol-point-exported"),
      position: Vector3d::new(3.5, -2.3, 6.0),
      flags: 73,
      level_vertex_id: 26543,
      game_vertex_id: 364,
    };

    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "patrol_point.ini");
    let mut file: File = overwrite_file(&config_path)?;
    let mut ltx: Ltx = Ltx::new();

    point.export("patrol_point", &mut ltx);

    ltx.write_to(&mut file)?;

    let read_point: PatrolPoint =
      PatrolPoint::import("patrol_point", &open_ini_config(config_path)?)?;

    assert_eq!(read_point, point);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize_object() -> io::Result<()> {
    let point: PatrolPoint = PatrolPoint {
      name: String::from("patrol-point-serialized"),
      position: Vector3d::new(5.5, -2.3, 6.0),
      flags: 53,
      level_vertex_id: 2351,
      game_vertex_id: 321,
    };

    let mut file: File = overwrite_file(&get_absolute_test_sample_file_path(
      file!(),
      "serialized.json",
    ))?;

    file.write_all(json!(point).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(point, serde_json::from_str::<PatrolPoint>(&serialized)?);

    Ok(())
  }
}
