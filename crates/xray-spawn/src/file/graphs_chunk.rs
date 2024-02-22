use crate::chunk::chunk::Chunk;
use crate::data::graph::graph_edge::GraphEdge;
use crate::data::graph::graph_header::GraphHeader;
use crate::data::graph::graph_level::GraphLevel;
use crate::data::graph::graph_level_point::GraphLevelPoint;
use crate::data::graph::graph_vertex::GraphVertex;
use byteorder::ByteOrder;
use std::{fmt, io};

/// `GameGraph::CHeader::load`, `GameGraph::SLevel::load`, `CGameGraph::Initialize`
pub struct GraphsChunk {
  pub header: GraphHeader,
  pub levels: Vec<GraphLevel>,
  pub vertices: Vec<GraphVertex>,
  pub edges: Vec<GraphEdge>,
  pub points: Vec<GraphLevelPoint>,
}

impl GraphsChunk {
  /// Read graphs chunk by position descriptor.
  pub fn read_from_chunk<T: ByteOrder>(mut chunk: Chunk) -> io::Result<GraphsChunk> {
    log::info!(
      "Parsing level graphs, {:?} -> {:?}",
      chunk.start_pos(),
      chunk.end_pos()
    );

    let header: GraphHeader = GraphHeader::read_from_chunk::<T>(&mut chunk)?;

    let mut levels: Vec<GraphLevel> = Vec::new();
    let mut vertices: Vec<GraphVertex> = Vec::new();
    let mut edges: Vec<GraphEdge> = Vec::new();
    let mut points: Vec<GraphLevelPoint> = Vec::new();

    for _ in 0..header.level_count {
      levels.push(GraphLevel::read_from_chunk::<T>(&mut chunk)?)
    }

    for _ in 0..header.vertex_count {
      vertices.push(GraphVertex::read_from_chunk::<T>(&mut chunk)?);
    }

    for _ in 0..header.edge_count {
      edges.push(GraphEdge::read_from_chunk::<T>(&mut chunk)?);
    }

    for _ in 0..header.point_count {
      points.push(GraphLevelPoint::read_from_chunk::<T>(&mut chunk)?);
    }

    log::info!(
      "Parsed graphs ver {:?}, {:?} processed, {:?} left",
      header.version,
      chunk.read_bytes_len(),
      chunk.read_bytes_remain()
    );

    assert_eq!(levels.len(), header.level_count as usize);
    assert_eq!(vertices.len(), header.vertex_count as usize);
    // todo: assert_eq!(file.cursor_pos(), file.end_pos());

    Ok(GraphsChunk {
      header,
      levels,
      vertices,
      edges,
      points,
    })
  }
}

impl fmt::Debug for GraphsChunk {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "GraphsChunk {{ header: {:?}, levels: Vector[{}], vertices: Vector[{}] }}",
      self.header,
      self.levels.len(),
      self.vertices.len(),
    )
  }
}
