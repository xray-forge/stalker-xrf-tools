use crate::chunk::iterator::ChunkIterator;
use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::vector_3d::Vector3d;
use crate::export::file_import::read_ini_field;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::Write;
use xray_ltx::{Ltx, Section};

/// `CPatrolPoint::load_raw`, `CPatrolPoint::load` in xray codebase.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatrolPoint {
  pub name: String,
  pub position: Vector3d<f32>,
  pub flags: u32,
  pub level_vertex_id: u32,
  pub game_vertex_id: u16,
}

impl PatrolPoint {
  /// Read points from the chunk reader.
  pub fn read_list<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Vec<Self>> {
    let mut points: Vec<Self> = Vec::new();

    for (index, mut point_reader) in ChunkIterator::new(reader).enumerate() {
      let mut index_reader: ChunkReader = point_reader.read_child_by_index(0)?;
      let mut points_reader: ChunkReader = point_reader.read_child_by_index(1)?;

      assert_eq!(index, index_reader.read_u32::<T>()? as usize);

      points.push(Self::read::<T>(&mut points_reader)?);

      assert!(index_reader.is_ended());
      assert!(point_reader.is_ended());
    }

    assert!(
      reader.is_ended(),
      "Chunk data should be read for patrol points list"
    );

    Ok(points)
  }

  /// Read patrol point data from the chunk reader.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let point: Self = Self {
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
  pub fn write_list<T: ByteOrder>(points: &[Self], writer: &mut ChunkWriter) -> DatabaseResult<()> {
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
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    writer.write_null_terminated_win_string(&self.name)?;
    writer.write_f32_3d_vector::<T>(&self.position)?;
    writer.write_u32::<T>(self.flags)?;
    writer.write_u32::<T>(self.level_vertex_id)?;
    writer.write_u16::<T>(self.game_vertex_id)?;

    Ok(())
  }

  /// Import patrol point data from ini config.
  pub fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini.section(section_name).unwrap_or_else(|| {
      panic!("Patrol point section {section_name} should be defined in ltx file")
    });

    Ok(Self {
      name: read_ini_field("name", section)?,
      position: read_ini_field("position", section)?,
      flags: read_ini_field("flags", section)?,
      level_vertex_id: read_ini_field("level_vertex_id", section)?,
      game_vertex_id: read_ini_field("game_vertex_id", section)?,
    })
  }

  /// Export patrol point data into ini.
  pub fn export(&self, section: &str, config: &mut Ltx) -> DatabaseResult<()> {
    config
      .with_section(section)
      .set("name", &self.name)
      .set("flags", self.flags.to_string())
      .set("position", self.position.to_string())
      .set("level_vertex_id", self.level_vertex_id.to_string())
      .set("game_vertex_id", self.game_vertex_id.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::patrol::patrol_point::PatrolPoint;
  use crate::data::vector_3d::Vector3d;
  use crate::export::file::open_ini_config;
  use crate::types::{DatabaseResult, SpawnByteOrder};
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use std::path::Path;
  use xray_ltx::Ltx;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_sample_file_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_file, overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: PatrolPoint = PatrolPoint {
      name: String::from("patrol-point-name"),
      position: Vector3d::new(1.5, -2.3, 1.0),
      flags: 33,
      level_vertex_id: 4500,
      game_vertex_id: 555,
    };

    original.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 40);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 40);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 40 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read: PatrolPoint = PatrolPoint::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read, original);

    Ok(())
  }

  #[test]
  fn test_read_write_list() -> DatabaseResult<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write_list.chunk");

    let original: Vec<PatrolPoint> = vec![
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

    PatrolPoint::write_list::<SpawnByteOrder>(&original, &mut writer)?;

    assert_eq!(writer.bytes_written(), 140);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 140);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 140 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read: Vec<PatrolPoint> = PatrolPoint::read_list::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(original, read);

    Ok(())
  }

  #[test]
  fn test_import_export() -> DatabaseResult<()> {
    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "import_export.ini");
    let mut file: File = overwrite_file(config_path)?;
    let mut ltx: Ltx = Ltx::new();

    let original: PatrolPoint = PatrolPoint {
      name: String::from("patrol-point-exported"),
      position: Vector3d::new(3.5, -2.3, 6.0),
      flags: 73,
      level_vertex_id: 26543,
      game_vertex_id: 364,
    };

    original.export("data", &mut ltx)?;
    ltx.write_to(&mut file)?;

    let read: PatrolPoint = PatrolPoint::import("data", &open_ini_config(config_path)?)?;

    assert_eq!(read, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> DatabaseResult<()> {
    let original: PatrolPoint = PatrolPoint {
      name: String::from("patrol-point-serialized"),
      position: Vector3d::new(5.5, -2.3, 6.0),
      flags: 53,
      level_vertex_id: 2351,
      game_vertex_id: 321,
    };

    let mut file: File = overwrite_file(&get_absolute_test_sample_file_path(
      file!(),
      "serialize_deserialize.json",
    ))?;

    file.write_all(json!(original).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      original,
      serde_json::from_str::<PatrolPoint>(&serialized).unwrap()
    );

    Ok(())
  }
}
