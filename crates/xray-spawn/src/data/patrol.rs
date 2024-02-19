use crate::chunk::chunk::Chunk;
use crate::chunk::iterator::ChunkIterator;
use crate::types::Vector3d;
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
    let mut index: u32 = 0;

    for mut point_chunk in ChunkIterator::new(&mut points_chunk) {
      let mut point_index_chunk: Chunk = point_chunk.read_child_by_index(0).unwrap();

      assert_eq!(point_index_chunk.size, 4);
      assert_eq!(index, point_index_chunk.read_u32::<T>().unwrap());

      let mut point_data_chunk: Chunk = point_chunk.read_child_by_index(1).unwrap();

      points.push(PatrolPoint::from_chunk::<T>(&mut point_data_chunk));

      index += 1;
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

#[derive(Debug)]
pub struct PatrolPoint {
  pub name: String,
  pub position: (f32, f32, f32),
  pub flags: u32,
  pub level_vertex_id: u32,
  pub game_vertex_id: u16,
}

impl PatrolPoint {
  pub fn from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> PatrolPoint {
    let name: String = chunk.read_null_terminated_string().unwrap();
    let position: Vector3d = chunk.read_f32_3d_vector::<T>().unwrap();
    let flags: u32 = chunk.read_u32::<T>().unwrap();
    let level_vertex_id: u32 = chunk.read_u32::<T>().unwrap();
    let game_vertex_id: u16 = chunk.read_u16::<T>().unwrap();

    PatrolPoint {
      name,
      position,
      flags,
      level_vertex_id,
      game_vertex_id,
    }
  }
}

#[derive(Debug)]
pub struct PatrolLink {
  pub index: u32,
  pub links: Vec<(u32, f32)>,
}

impl PatrolLink {
  pub fn new(index: u32) -> PatrolLink {
    PatrolLink {
      index,
      links: Vec::new(),
    }
  }
}
