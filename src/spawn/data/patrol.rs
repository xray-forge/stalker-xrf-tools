use crate::spawn::chunk::{Chunk, ChunkSliceIterator};
use crate::spawn::chunk_utils::{read_f32_vector, read_null_terminated_string};
use crate::spawn::types::Vector3d;
use byteorder::{LittleEndian, ReadBytesExt};
use fileslice::FileSlice;
use std::io::Read;

#[derive(Debug)]
pub struct Patrol {
  pub name: String,
  pub points: Vec<PatrolPoint>,
  pub links: Vec<PatrolLink>,
}

impl Patrol {
  pub fn read(file: &mut FileSlice) -> Patrol {
    let name: String = Self::read_name(file);

    let (mut data_slice, _) = Chunk::open_by_index(file, 1).unwrap();

    let points_count: u32 = Self::read_points_count(&mut data_slice);
    let points: Vec<PatrolPoint> = Self::read_points(&mut data_slice);
    let links: Vec<PatrolLink> = Self::read_links(&mut data_slice);

    assert_eq!(points_count, points.len() as u32);

    Patrol {
      name,
      points,
      links,
    }
  }

  fn read_name(file: &mut FileSlice) -> String {
    let (mut name_slice, name_chunk) = Chunk::open_by_index(file, 0).unwrap();

    let mut patrol_name: String = String::new();
    name_slice.read_to_string(&mut patrol_name).unwrap();

    assert_eq!(patrol_name.len(), name_chunk.size as usize);

    patrol_name
  }

  fn read_points_count(file: &mut FileSlice) -> u32 {
    let (mut points_count_slice, points_count_chunk) = Chunk::open_by_index(file, 0).unwrap();

    assert_eq!(points_count_chunk.size, 4);

    points_count_slice.read_u32::<LittleEndian>().unwrap()
  }

  fn read_points(file: &mut FileSlice) -> Vec<PatrolPoint> {
    let mut points: Vec<PatrolPoint> = Vec::new();
    let mut index: u32 = 0;

    let (mut points_slice, _) = Chunk::open_by_index(file, 1).unwrap();

    for (mut point_slice, _) in ChunkSliceIterator::new(&mut points_slice) {
      let (mut point_index_slice, point_index_chunk) =
        Chunk::open_by_index(&mut point_slice, 0).unwrap();

      assert_eq!(point_index_chunk.size, 4);
      assert_eq!(index, point_index_slice.read_u32::<LittleEndian>().unwrap());

      let (mut point_data_slice, _) = Chunk::open_by_index(&mut point_slice, 1).unwrap();

      points.push(PatrolPoint::from_file(&mut point_data_slice));

      index += 1;
    }

    points
  }

  fn read_links(file: &mut FileSlice) -> Vec<PatrolLink> {
    let mut links: Vec<PatrolLink> = Vec::new();

    let (mut links_slice, links_chunk) = Chunk::open_by_index(file, 2).unwrap();

    if links_chunk.size > 0 {
      let mut index: u32 = 0;
      let from: u32 = links_slice.read_u32::<LittleEndian>().unwrap();
      let count: u32 = links_slice.read_u32::<LittleEndian>().unwrap();

      let mut link: PatrolLink = PatrolLink::new(from);

      for _ in 0..count {
        let to: u32 = links_slice.read_u32::<LittleEndian>().unwrap();
        let weight: f32 = links_slice.read_f32::<LittleEndian>().unwrap();

        link.links.push((to, weight));
        index += 1;
      }

      assert_eq!(index, count);

      links.push(link);
    }

    assert_eq!(file.cursor_pos(), file.end_pos());

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
  pub fn from_file(file: &mut FileSlice) -> PatrolPoint {
    let name: String = read_null_terminated_string(file);
    let position: Vector3d = read_f32_vector::<LittleEndian>(file);
    let flags: u32 = file.read_u32::<LittleEndian>().unwrap();
    let level_vertex_id: u32 = file.read_u32::<LittleEndian>().unwrap();
    let game_vertex_id: u16 = file.read_u16::<LittleEndian>().unwrap();

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
