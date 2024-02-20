use crate::chunk::chunk::Chunk;
use crate::data::level::Level;
use crate::data::vertex::Vertex;
use byteorder::{ByteOrder, ReadBytesExt};
use std::fmt;

pub struct GraphsChunk {
  pub chunk: Chunk,
  pub version: u8,
  pub vertex_count: u16,
  pub edge_count: u32,
  pub point_count: u32,
  pub guid: u128,
  pub size: u32,
  pub levels: Vec<Level>,
  pub vertices: Vec<Vertex>,
}

impl GraphsChunk {
  /// Read patrols chunk by position descriptor.
  pub fn read_from_chunk<T: ByteOrder>(mut chunk: Chunk) -> Option<GraphsChunk> {
    log::info!(
      "Parsing level graphs, {:?} -> {:?}",
      chunk.start_pos(),
      chunk.end_pos()
    );

    let version: u8 = chunk.read_u8().unwrap();
    let vertex_count: u16 = chunk.read_u16::<T>().unwrap();
    let edge_count: u32 = chunk.read_u32::<T>().unwrap();
    let point_count: u32 = chunk.read_u32::<T>().unwrap();
    let guid: u128 = chunk.read_u128::<T>().unwrap();
    let level_count: u8 = chunk.read_u8().unwrap();

    let mut levels: Vec<Level> = Vec::new();
    let mut vertices: Vec<Vertex> = Vec::new();

    for _ in 0..level_count {
      levels.push(Level::read_from_chunk::<T>(&mut chunk))
    }

    for _ in 0..vertex_count {
      vertices.push(Vertex::read_from_chunk::<T>(&mut chunk));
    }

    log::info!(
      "Parsed graphs v{version}, {:?} processed, {:?} left",
      chunk.read_bytes_len(),
      chunk.read_bytes_remain()
    );

    assert_eq!(levels.len(), level_count as usize);
    assert_eq!(vertices.len(), vertex_count as usize);
    // todo: assert_eq!(file.cursor_pos(), file.end_pos());

    Some(GraphsChunk {
      chunk,
      version,
      levels,
      vertices,
      vertex_count,
      edge_count,
      point_count,
      guid,
      size: 4096,
    })
  }
}

impl fmt::Debug for GraphsChunk {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "GraphsChunk {{ chunk: {:?}, version: {}, vertex_count: {}, edge_count: {}, point_count: {},\
       guid: {}, size: {}, levels: Vector[{}], vertices: Vector[{}] }}",
      self.chunk,
      self.version,
      self.vertex_count,
      self.edge_count,
      self.point_count,
      self.guid,
      self.size,
      self.levels.len(),
      self.vertices.len(),
    )
  }
}
