use crate::chunk::iterator::ChunkIterator;
use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::patrol::patrol_link::PatrolLink;
use crate::data::patrol::patrol_point::PatrolPoint;
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::Write;

/// Patrols list is represented by list of samples containing patrol chunk.
/// 0...N, where N is chunk.
///
/// `CPatrolPathStorage::load`, `CPatrolPath::load_raw` in xray codebase.
///
/// Patrol chunk has the following structure:
/// 0 - metadata
///   - name
/// 1 - data
///   0 - points count
///   1 - patrol points
///   2 - patrol points links
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Patrol {
  #[serde(rename = "name")]
  pub name: String,
  #[serde(rename = "points")]
  pub points: Vec<PatrolPoint>,
  #[serde(rename = "links")]
  pub links: Vec<PatrolLink>,
}

impl Patrol {
  /// Read chunk as list of patrol samples.
  pub fn read_list<T: ByteOrder>(reader: &mut ChunkReader, count: u32) -> io::Result<Vec<Patrol>> {
    let mut read_patrols_count: u32 = 0;
    let mut patrols: Vec<Patrol> = Vec::new();

    for mut patrol_reader in ChunkIterator::new(reader) {
      patrols.push(Patrol::read::<T>(&mut patrol_reader)?);
      read_patrols_count += 1;
    }

    assert_eq!(read_patrols_count, count);
    assert!(
      reader.is_ended(),
      "Chunk data should be read for patrols list"
    );

    Ok(patrols)
  }

  /// Read chunk as patrol.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<Patrol> {
    let mut meta_reader: ChunkReader = reader.read_child_by_index(0)?;
    let mut data_reader: ChunkReader = reader.read_child_by_index(1)?;

    let mut point_count_reader: ChunkReader = data_reader.read_child_by_index(0)?;
    let mut points_reader: ChunkReader = data_reader.read_child_by_index(1)?;
    let mut links_reader: ChunkReader = data_reader.read_child_by_index(2)?;

    let name: String = meta_reader.read_null_terminated_win_string()?;

    assert_eq!(name.len() + 1, meta_reader.size as usize); // Count null termination char.

    let points_count: u32 = point_count_reader.read_u32::<T>()?;
    let points: Vec<PatrolPoint> = PatrolPoint::read_list::<T>(&mut points_reader)?;
    let links: Vec<PatrolLink> = PatrolLink::read_list::<T>(&mut links_reader)?;

    assert_eq!(points_count, points.len() as u32);
    assert!(reader.is_ended(), "Expect patrol chunk to be ended");

    Ok(Patrol {
      name,
      points,
      links,
    })
  }

  /// Write list of patrols into chunk writer.
  pub fn write_list<T: ByteOrder>(patrols: &[Patrol], writer: &mut ChunkWriter) -> io::Result<()> {
    for (index, patrol) in patrols.iter().enumerate() {
      let mut patrol_writer: ChunkWriter = ChunkWriter::new();

      patrol.write::<T>(&mut patrol_writer)?;

      writer.write_all(&patrol_writer.flush_chunk_into_buffer::<T>(index)?)?;
    }

    Ok(())
  }

  /// Write single patrol entity into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    let mut meta_writer: ChunkWriter = ChunkWriter::new();
    let mut data_writer: ChunkWriter = ChunkWriter::new();

    let mut point_count_writer: ChunkWriter = ChunkWriter::new();
    let mut points_writer: ChunkWriter = ChunkWriter::new();
    let mut links_writer: ChunkWriter = ChunkWriter::new();

    meta_writer.write_null_terminated_win_string(&self.name)?;

    point_count_writer.write_u32::<T>(self.points.len() as u32)?;

    PatrolPoint::write_list::<T>(&self.points, &mut points_writer)?;
    PatrolLink::write_list::<T>(&self.links, &mut links_writer)?;

    data_writer.write_all(&point_count_writer.flush_chunk_into_buffer::<T>(0)?)?;
    data_writer.write_all(&points_writer.flush_chunk_into_buffer::<T>(1)?)?;
    data_writer.write_all(&links_writer.flush_chunk_into_buffer::<T>(2)?)?;

    writer.write_all(&meta_writer.flush_chunk_into_buffer::<T>(0)?)?;
    writer.write_all(&data_writer.flush_chunk_into_buffer::<T>(1)?)?;

    Ok(())
  }

  /// Import patrols data from provided path.
  pub fn import(
    section: &str,
    patrols_config: &Ini,
    patrol_points_config: &Ini,
    patrol_links_config: &Ini,
  ) -> io::Result<Patrol> {
    let props: &Properties = patrols_config
      .section(Some(section))
      .unwrap_or_else(|| panic!("Patrol section {section} should be defined in ltx file"));

    let name: String = read_ini_field("name", props)?;
    let points_list: String = read_ini_field("points", props)?;
    let links_count: usize = read_ini_field("links_count", props)?;

    let mut points: Vec<PatrolPoint> = Vec::new();
    let mut links: Vec<PatrolLink> = Vec::new();

    for section in points_list.split(',').map(|it| it.trim()) {
      points.push(PatrolPoint::import(
        &format!("{}.{}", name, section),
        patrol_points_config,
      )?);
    }

    for index in 0..links_count {
      links.push(PatrolLink::import(
        &format!("{}.{}", name, index),
        patrol_links_config,
      )?);
    }

    assert_eq!(links.len(), links_count);

    Ok(Patrol {
      name,
      points,
      links,
    })
  }

  /// Export patrol data into ltx config files.
  /// Creates separate files for patrols, points and links.
  pub fn export<T: ByteOrder>(
    &self,
    section: &str,
    patrols_config: &mut Ini,
    patrol_points_config: &mut Ini,
    patrol_links_config: &mut Ini,
  ) {
    patrols_config
      .with_section(Some(section))
      .set("name", &self.name)
      .set(
        "points",
        self
          .points
          .iter()
          .map(|it| it.name.clone())
          .collect::<Vec<String>>()
          .join(","),
      )
      .set("links_count", self.links.len().to_string());

    for point in &self.points {
      point.export(
        &format!("{}.{}", self.name, point.name),
        patrol_points_config,
      );
    }

    for (index, link) in self.links.iter().enumerate() {
      link.export(&format!("{}.{}", self.name, index), patrol_links_config);
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::patrol::patrol::Patrol;
  use crate::data::patrol::patrol_link::PatrolLink;
  use crate::data::patrol::patrol_point::PatrolPoint;
  use crate::data::vector_3d::Vector3d;
  use crate::export::file::{export_ini_to_file, open_ini_config};
  use crate::test::file::read_file_as_string;
  use crate::test::utils::{
    get_absolute_test_sample_file_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_file, overwrite_test_relative_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use ini::Ini;
  use serde_json::json;
  use std::fs::File;
  use std::io;
  use std::io::{Seek, SeekFrom, Write};
  use std::path::Path;

  #[test]
  fn test_read_write_simple_patrol_point() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "patrol_simple.chunk");

    let patrol: Patrol = Patrol {
      name: String::from("patrol-name"),
      points: vec![
        PatrolPoint {
          name: String::from("patrol-point-1"),
          position: Vector3d::new(7.5, -2.3, -100.0),
          flags: 33,
          level_vertex_id: 63463634,
          game_vertex_id: 555,
        },
        PatrolPoint {
          name: String::from("patrol-point-2"),
          position: Vector3d::new(2.5, -5.3, 3.0),
          flags: 64,
          level_vertex_id: 5500,
          game_vertex_id: 666,
        },
      ],
      links: vec![PatrolLink {
        index: 0,
        links: vec![(10, 50.5), (15, 60.25)],
      }],
    };

    patrol.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 210);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 210);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 210 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_patrol: Patrol = Patrol::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_patrol, patrol);

    Ok(())
  }

  #[test]
  fn test_read_write_simple_patrols_list() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "patrol_list.chunk");

    let patrols: Vec<Patrol> = vec![
      Patrol {
        name: String::from("patrol-1"),
        points: vec![
          PatrolPoint {
            name: String::from("patrol-point-1"),
            position: Vector3d::new(1.5, -2.3, 1.0),
            flags: 33,
            level_vertex_id: 250,
            game_vertex_id: 555,
          },
          PatrolPoint {
            name: String::from("patrol-point-2"),
            position: Vector3d::new(2.5, -5.3, 3.0),
            flags: 64,
            level_vertex_id: 5500,
            game_vertex_id: 666,
          },
        ],
        links: vec![PatrolLink {
          index: 0,
          links: vec![(10, 50.5), (15, 60.25)],
        }],
      },
      Patrol {
        name: String::from("patrol-2"),
        points: vec![
          PatrolPoint {
            name: String::from("patrol-point-1"),
            position: Vector3d::new(7.5, -4.3, 3.0),
            flags: 1,
            level_vertex_id: 601,
            game_vertex_id: 541,
          },
          PatrolPoint {
            name: String::from("patrol-point-2"),
            position: Vector3d::new(2.5, -5.3, 3.0),
            flags: 0,
            level_vertex_id: 600,
            game_vertex_id: 542,
          },
        ],
        links: vec![PatrolLink {
          index: 0,
          links: vec![(10, 50.5), (15, 60.25)],
        }],
      },
    ];

    Patrol::write_list::<SpawnByteOrder>(&patrols, &mut writer)?;

    assert_eq!(writer.bytes_written(), 430);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 430);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 430 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_patrols: Vec<Patrol> = Patrol::read_list::<SpawnByteOrder>(&mut reader, 2)?;

    assert_eq!(read_patrols, patrols);

    Ok(())
  }

  #[test]
  fn test_import_export_object() -> io::Result<()> {
    let patrol: Patrol = Patrol {
      name: String::from("patrol-name-exp"),
      points: vec![
        PatrolPoint {
          name: String::from("patrol-point-1-exp"),
          position: Vector3d::new(7.5, -2.42, -4.0),
          flags: 53,
          level_vertex_id: 2533,
          game_vertex_id: 512,
        },
        PatrolPoint {
          name: String::from("patrol-point-2-exp"),
          position: Vector3d::new(4.5, -5.3, 4.5),
          flags: 12,
          level_vertex_id: 23421,
          game_vertex_id: 5233,
        },
      ],
      links: vec![PatrolLink {
        index: 0,
        links: vec![(22, 34.5), (24, 553.25)],
      }],
    };

    let patrol_config_path: &Path = &get_absolute_test_sample_file_path(file!(), "patrol.ini");
    let points_config_path: &Path =
      &get_absolute_test_sample_file_path(file!(), "patrol_points.ini");
    let links_config_path: &Path = &get_absolute_test_sample_file_path(file!(), "patrol_links.ini");

    let mut patrol_file: File = overwrite_file(&patrol_config_path)?;
    let mut points_file: File = overwrite_file(&points_config_path)?;
    let mut links_file: File = overwrite_file(&links_config_path)?;

    let mut patrol_ini: Ini = Ini::new();
    let mut links_ini: Ini = Ini::new();
    let mut points_ini: Ini = Ini::new();

    patrol.export::<SpawnByteOrder>(
      &patrol.name,
      &mut patrol_ini,
      &mut points_ini,
      &mut links_ini,
    );

    export_ini_to_file(&patrol_ini, &mut patrol_file)?;
    export_ini_to_file(&points_ini, &mut points_file)?;
    export_ini_to_file(&links_ini, &mut links_file)?;

    let read_point: Patrol = Patrol::import(
      &patrol.name,
      &open_ini_config(patrol_config_path)?,
      &open_ini_config(points_config_path)?,
      &open_ini_config(links_config_path)?,
    )?;

    assert_eq!(read_point, patrol);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize_object() -> io::Result<()> {
    let patrol: Patrol = Patrol {
      name: String::from("patrol-name-serde"),
      points: vec![
        PatrolPoint {
          name: String::from("patrol-point-1-serde"),
          position: Vector3d::new(4.5, -5.42, -3.2),
          flags: 83,
          level_vertex_id: 4657,
          game_vertex_id: 457,
        },
        PatrolPoint {
          name: String::from("patrol-point-2-serde"),
          position: Vector3d::new(6.21, -5.34, 3.23),
          flags: 53,
          level_vertex_id: 6345,
          game_vertex_id: 15211,
        },
      ],
      links: vec![PatrolLink {
        index: 0,
        links: vec![(32, 34.5), (24, 53.25)],
      }],
    };

    let mut file: File = overwrite_file(&get_absolute_test_sample_file_path(
      file!(),
      "serialized.json",
    ))?;

    file.write_all(json!(patrol).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(patrol, serde_json::from_str::<Patrol>(&serialized)?);

    Ok(())
  }
}
