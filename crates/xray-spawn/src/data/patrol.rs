use crate::chunk::chunk::Chunk;
use crate::chunk::iterator::ChunkIterator;
use crate::data::patrol_link::PatrolLink;
use crate::data::patrol_point::PatrolPoint;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

#[derive(Debug)]
pub struct Patrol {
  pub name: String,
  pub points: Vec<PatrolPoint>,
  pub links: Vec<PatrolLink>,
}

impl Patrol {
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
    assert_eq!(
      chunk.read_bytes_remain(),
      0,
      "Chunk data should be read for patrols list."
    );

    Ok(patrols)
  }

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
    assert_eq!(
      chunk.read_bytes_remain(),
      0,
      "Chunk data should be read for patrol."
    );

    Ok(Patrol {
      name,
      points,
      links,
    })
  }
}
