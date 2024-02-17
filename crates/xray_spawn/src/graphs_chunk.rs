use crate::chunk::chunk::Chunk;
use crate::data::level::Level;
use crate::data::vertex::Vertex;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;
use std::fmt;

pub struct GraphsChunk {
  pub index: u32,
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
  pub fn from_chunk(mut chunk: Chunk) -> Option<GraphsChunk> {
    log::info!(
      "Parsing level graphs, {:?} -> {:?}",
      chunk.start_pos(),
      chunk.end_pos()
    );

    let version: u8 = chunk.read_u8().unwrap();
    let vertex_count: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();
    let edge_count: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let point_count: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let guid: u128 = chunk.read_u128::<SpawnByteOrder>().unwrap();
    let level_count: u8 = chunk.read_u8().unwrap();

    let mut levels: Vec<Level> = Vec::new();
    let mut vertices: Vec<Vertex> = Vec::new();

    for _ in 0..level_count {
      levels.push(Level::from_chunk(&mut chunk))
    }

    for _ in 0..vertex_count {
      vertices.push(Vertex::from_chunk(&mut chunk));
    }

    log::info!(
      "Parsed graphs v{version}, {:?} processed, {:?} left",
      chunk.read_bytes_len(),
      chunk.read_bytes_remain()
    );

    assert_eq!(levels.len(), level_count as usize);
    assert_eq!(vertices.len(), vertex_count as usize);
    // todo: assert_eq!(file.cursor_pos(), file.end_pos());

    return Some(GraphsChunk {
      index: chunk.index,
      version,
      levels,
      vertices,
      vertex_count,
      edge_count,
      point_count,
      guid,
      size: 4096,
    });
  }
}

impl fmt::Debug for GraphsChunk {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "GraphsChunk {{ index: {}, version: {}, vertex_count: {}, edge_count: {}, point_count: {},\
       guid: {}, size: {}, levels: Vector[{}], vertices: Vector[{}] }}",
      self.index,
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
