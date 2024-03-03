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
  pub name: String,
  pub points: Vec<PatrolPoint>,
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
    patrols_config: &mut Ini,
    patrol_points_config: &mut Ini,
    patrol_links_config: &mut Ini,
  ) -> io::Result<()> {
    patrols_config
      .with_section(Some(&self.name))
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

    Ok(())
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
  use crate::test::utils::{
    get_test_sample_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_simple_patrol_point() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_test_sample_file_sub_dir(file!(), "patrol_simple.chunk");

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
      &mut overwrite_test_resource_as_file(&filename)?,
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
    let filename: String = get_test_sample_file_sub_dir(file!(), "patrol_list.chunk");

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
      &mut overwrite_test_resource_as_file(&filename)?,
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
}
