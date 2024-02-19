use crate::chunk::chunk::Chunk;
use crate::chunk::iterator::ChunkIterator;
use crate::data::patrol_link::PatrolLink;
use crate::data::patrol_point::PatrolPoint;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io::Read;

#[derive(Debug)]
pub struct Patrol {
  pub name: String,
  pub points: Vec<PatrolPoint>,
  pub links: Vec<PatrolLink>,
}

impl Patrol {
  pub fn from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> Patrol {
    let name: String = Self::read_name(chunk);

    let mut data_chunk: Chunk = chunk.read_child_by_index(1).unwrap();

    let points_count: u32 = Self::read_points_count::<T>(&mut data_chunk);
    let points: Vec<PatrolPoint> = Self::read_points::<T>(&mut data_chunk);
    let links: Vec<PatrolLink> = Self::read_links::<T>(&mut data_chunk);

    assert_eq!(points_count, points.len() as u32);

    Patrol {
      name,
      points,
      links,
    }
  }

  fn read_name(chunk: &mut Chunk) -> String {
    let mut name_chunk: Chunk = chunk.read_child_by_index(0).unwrap();
    let mut patrol_name: String = String::new();

    name_chunk.read_to_string(&mut patrol_name).unwrap();

    assert_eq!(patrol_name.len(), name_chunk.size as usize);

    patrol_name
  }

  fn read_points_count<T: ByteOrder>(chunk: &mut Chunk) -> u32 {
    let mut points_count_chunk: Chunk = chunk.read_child_by_index(0).unwrap();

    assert_eq!(points_count_chunk.size, 4);

    points_count_chunk.read_u32::<T>().unwrap()
  }

  fn read_points<T: ByteOrder>(chunk: &mut Chunk) -> Vec<PatrolPoint> {
    let mut points_chunk: Chunk = chunk.read_child_by_index(1).unwrap();
    let mut points: Vec<PatrolPoint> = Vec::new();

    for (index, mut point_chunk) in ChunkIterator::new(&mut points_chunk).enumerate() {
      let mut point_index_chunk: Chunk = point_chunk.read_child_by_index(0).unwrap();

      assert_eq!(point_index_chunk.size, 4);
      assert_eq!(index, point_index_chunk.read_u32::<T>().unwrap() as usize);

      let mut point_data_chunk: Chunk = point_chunk.read_child_by_index(1).unwrap();

      points.push(PatrolPoint::from_chunk::<T>(&mut point_data_chunk));
    }

    points
  }

  fn read_links<T: ByteOrder>(chunk: &mut Chunk) -> Vec<PatrolLink> {
    let mut links_chunk: Chunk = chunk.read_child_by_index(2).unwrap();
    let mut links: Vec<PatrolLink> = Vec::new();

    if links_chunk.size > 0 {
      let mut index: u32 = 0;
      let from: u32 = links_chunk.read_u32::<T>().unwrap();
      let count: u32 = links_chunk.read_u32::<T>().unwrap();

      let mut link: PatrolLink = PatrolLink::new(from);

      for _ in 0..count {
        let to: u32 = links_chunk.read_u32::<T>().unwrap();
        let weight: f32 = links_chunk.read_f32::<T>().unwrap();

        link.links.push((to, weight));
        index += 1;
      }

      assert_eq!(index, count);

      links.push(link);
    }

    assert_eq!(chunk.read_bytes_remain(), 0);

    links
  }
}
