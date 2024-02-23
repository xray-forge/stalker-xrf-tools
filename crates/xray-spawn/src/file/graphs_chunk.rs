use crate::chunk::chunk::Chunk;
use crate::chunk::iterator::ChunkSizePackedIterator;
use crate::data::graph::graph_cross_table::GraphCrossTable;
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
  pub cross_tables: Vec<GraphCrossTable>,
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
    let mut cross_tables: Vec<GraphCrossTable> = Vec::new();

    for _ in 0..header.level_count {
      levels.push(GraphLevel::read_from_chunk::<T>(&mut chunk)?)
    }

    for _ in 0..header.vertex_count {
      vertices.push(GraphVertex::read_from_chunk::<T>(&mut chunk)?);
    }

    for _ in 0..header.edges_count {
      edges.push(GraphEdge::read_from_chunk::<T>(&mut chunk)?);
    }

    for _ in 0..header.point_count {
      points.push(GraphLevelPoint::read_from_chunk::<T>(&mut chunk)?);
    }

    for mut cross_table_chunk in ChunkSizePackedIterator::new(&mut chunk) {
      cross_tables.push(GraphCrossTable::read_from_chunk::<T>(
        &mut cross_table_chunk,
      )?);

      assert!(
        cross_table_chunk.is_ended(),
        "Expect cross table chunk to be ended."
      );
    }

    log::info!(
      "Parsed graphs ver {:?}, {:?} processed, {:?} left",
      header.version,
      chunk.read_bytes_len(),
      chunk.read_bytes_remain()
    );

    assert_eq!(levels.len(), header.level_count as usize);
    assert_eq!(vertices.len(), header.vertex_count as usize);
    assert_eq!(edges.len(), header.edges_count as usize);
    assert_eq!(points.len(), header.point_count as usize);
    assert!(chunk.is_ended(), "Expect graphs chunk to be ended.");

    Ok(GraphsChunk {
      header,
      levels,
      vertices,
      edges,
      points,
      cross_tables,
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
