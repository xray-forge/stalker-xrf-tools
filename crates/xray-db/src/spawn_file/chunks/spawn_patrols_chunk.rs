use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::patrol::patrol::Patrol;
use crate::export::file::{create_export_file, open_ltx_config};
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::Write;
use std::path::Path;
use xray_ltx::Ltx;

/// `CPatrolPathStorage::load` in xray engine.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnPatrolsChunk {
  pub patrols: Vec<Patrol>,
}

impl SpawnPatrolsChunk {
  pub const CHUNK_ID: u32 = 3;

  /// Read patrols list from the chunk.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let mut meta_reader: ChunkReader = reader.read_child_by_index(0)?;
    let mut data_reader: ChunkReader = reader.read_child_by_index(1)?;

    let count: u32 = meta_reader.read_u32::<T>()?;
    let patrols: Vec<Patrol> = Patrol::read_list::<T>(&mut data_reader, count)?;

    assert_eq!(count, patrols.len() as u32);

    assert!(meta_reader.is_ended());
    assert!(data_reader.is_ended());
    assert!(reader.is_ended());

    log::info!("Parsed patrols, bytes {:?}", reader.read_bytes_len());

    Ok(Self { patrols })
  }

  /// Write patrols data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    let mut meta_writer: ChunkWriter = ChunkWriter::new();
    let mut data_writer: ChunkWriter = ChunkWriter::new();

    meta_writer.write_u32::<T>(self.patrols.len() as u32)?;
    Patrol::write_list::<T>(&self.patrols, &mut data_writer)?;

    writer.write_all(meta_writer.flush_chunk_into_buffer::<T>(0)?.as_slice())?;
    writer.write_all(data_writer.flush_chunk_into_buffer::<T>(1)?.as_slice())?;

    log::info!("Written patrols chunk, {:?} bytes", writer.bytes_written());

    Ok(())
  }

  /// Import patrols data from provided path.
  pub fn import(path: &Path) -> DatabaseResult<Self> {
    let patrols_ltx: Ltx = open_ltx_config(&path.join("patrols.ltx"))?;
    let patrol_points_ltx: Ltx = open_ltx_config(&path.join("patrol_points.ltx"))?;
    let patrol_links_ltx: Ltx = open_ltx_config(&path.join("patrol_links.ltx"))?;

    let mut patrols: Vec<Patrol> = Vec::new();

    for section in patrols_ltx.sections() {
      patrols.push(Patrol::import(
        section,
        &patrols_ltx,
        &patrol_points_ltx,
        &patrol_links_ltx,
      )?);
    }

    log::info!("Imported patrols chunk");

    Ok(Self { patrols })
  }

  /// Export patrols data into provided path.
  pub fn export(&self, path: &Path) -> DatabaseResult {
    let mut patrols_ltx: Ltx = Ltx::new();
    let mut patrol_points_ltx: Ltx = Ltx::new();
    let mut patrol_links_ltx: Ltx = Ltx::new();

    for patrol in &self.patrols {
      patrol.export(
        &patrol.name,
        &mut patrols_ltx,
        &mut patrol_points_ltx,
        &mut patrol_links_ltx,
      )?;
    }

    patrols_ltx.write_to(&mut create_export_file(&path.join("patrols.ltx"))?)?;
    patrol_points_ltx.write_to(&mut create_export_file(&path.join("patrol_points.ltx"))?)?;
    patrol_links_ltx.write_to(&mut create_export_file(&path.join("patrol_links.ltx"))?)?;

    log::info!("Exported patrols chunk");

    Ok(())
  }
}

impl fmt::Debug for SpawnPatrolsChunk {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "PatrolsChunk {{ patrols: Vector[{}] }}",
      self.patrols.len(),
    )
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
  use crate::spawn_file::chunks::spawn_patrols_chunk::SpawnPatrolsChunk;
  use crate::types::{DatabaseResult, SpawnByteOrder};
  use fileslice::FileSlice;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult {
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: SpawnPatrolsChunk = SpawnPatrolsChunk {
      patrols: vec![
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
              position: Vector3d::new(4.5, -5.3, 3.0),
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
      ],
    };

    let mut writer: ChunkWriter = ChunkWriter::new();

    original.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 450);

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 450);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 450 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read: SpawnPatrolsChunk = SpawnPatrolsChunk::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read, original);

    Ok(())
  }
}
