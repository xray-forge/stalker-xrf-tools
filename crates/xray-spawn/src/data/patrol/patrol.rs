use crate::chunk::chunk::Chunk;
use crate::chunk::iterator::ChunkIterator;
use crate::chunk::writer::ChunkWriter;
use crate::data::patrol::patrol_link::PatrolLink;
use crate::data::patrol::patrol_point::PatrolPoint;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

/// Patrols list is represented by list of chunks containing patrol chunk.
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
#[derive(Debug)]
pub struct Patrol {
  pub name: String,
  pub points: Vec<PatrolPoint>,
  pub links: Vec<PatrolLink>,
}

impl Patrol {
  /// Read chunk as list of patrol chunks.
  pub fn read_list_from_chunk<T: ByteOrder>(
    chunk: &mut Chunk,
    count: u32,
  ) -> io::Result<Vec<Patrol>> {
    let mut read_patrols_count: u32 = 0;
    let mut patrols: Vec<Patrol> = Vec::new();

    for mut patrol_chunk in ChunkIterator::new(chunk) {
      patrols.push(Patrol::read_from_chunk::<T>(&mut patrol_chunk)?);
      read_patrols_count += 1;
    }

    assert_eq!(read_patrols_count, count);
    assert!(
      chunk.is_ended(),
      "Chunk data should be read for patrols list."
    );

    Ok(patrols)
  }

  /// Read chunk as patrol.
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<Patrol> {
    let mut meta_chunk: Chunk = chunk.read_child_by_index(0)?;
    let mut data_chunk: Chunk = chunk.read_child_by_index(1)?;

    let mut point_count_chunk: Chunk = data_chunk.read_child_by_index(0)?;
    let mut points_chunk: Chunk = data_chunk.read_child_by_index(1)?;
    let mut links_chunk: Chunk = data_chunk.read_child_by_index(2)?;

    let name: String = meta_chunk.read_null_terminated_string()?;

    assert_eq!(name.len() + 1, meta_chunk.size as usize); // Count null termination char.

    let points_count: u32 = point_count_chunk.read_u32::<T>()?;
    let points: Vec<PatrolPoint> = PatrolPoint::read_list_from_chunk::<T>(&mut points_chunk)?;
    let links: Vec<PatrolLink> = PatrolLink::read_list_from_chunk::<T>(&mut links_chunk)?;

    assert_eq!(points_count, points.len() as u32);
    assert!(chunk.is_ended(), "Expect patrol chunk to be ended.");

    Ok(Patrol {
      name,
      points,
      links,
    })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    let mut meta_writer: ChunkWriter = ChunkWriter::new();
    let mut data_writer: ChunkWriter = ChunkWriter::new();

    let mut point_count_writer: ChunkWriter = ChunkWriter::new();
    let mut points_writer: ChunkWriter = ChunkWriter::new();
    let mut links_writer: ChunkWriter = ChunkWriter::new();

    meta_writer.write_null_terminated_string(&self.name)?;

    point_count_writer.write_u32::<T>(self.points.len() as u32)?;

    PatrolPoint::write_list::<T>(&self.points, &mut points_writer)?;
    PatrolLink::write_list::<T>(&self.links, &mut links_writer)?;

    todo!("Implement writer.");

    Ok(())
  }
}
