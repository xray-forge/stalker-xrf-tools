use crate::chunk::chunk::Chunk;
use crate::chunk::iterator::ChunkSizePackedIterator;
use crate::chunk::writer::ChunkWriter;
use crate::data::graph::graph_cross_table::GraphCrossTable;
use crate::data::graph::graph_edge::GraphEdge;
use crate::data::graph::graph_header::GraphHeader;
use crate::data::graph::graph_level::GraphLevel;
use crate::data::graph::graph_level_point::GraphLevelPoint;
use crate::data::graph::graph_vertex::GraphVertex;
use crate::export::file_export::{create_export_file, export_ini_to_file};
use byteorder::{ByteOrder, WriteBytesExt};
use ini::Ini;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fmt, io};

/// `GameGraph::CHeader::load`, `GameGraph::SLevel::load`, `CGameGraph::Initialize`
#[derive(Clone, PartialEq)]
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
    let mut levels: Vec<GraphLevel> = Vec::new();
    let mut vertices: Vec<GraphVertex> = Vec::new();
    let mut edges: Vec<GraphEdge> = Vec::new();
    let mut points: Vec<GraphLevelPoint> = Vec::new();
    let mut cross_tables: Vec<GraphCrossTable> = Vec::new();

    let header: GraphHeader = GraphHeader::read_from_chunk::<T>(&mut chunk)?;

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
      "Parsed graphs ver {:?}, {:?} bytes",
      header.version,
      chunk.read_bytes_len(),
    );

    assert_eq!(levels.len(), header.level_count as usize);
    assert_eq!(vertices.len(), header.vertex_count as usize);
    assert_eq!(edges.len(), header.edges_count as usize);
    assert_eq!(points.len(), header.point_count as usize);
    assert_eq!(cross_tables.len(), header.level_count as usize);
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

  /// Write whole graphs chunk into the writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.header.write::<T>(writer)?;

    for level in &self.levels {
      level.write::<T>(writer)?;
    }

    for vertex in &self.vertices {
      vertex.write::<T>(writer)?;
    }

    for edge in &self.edges {
      edge.write::<T>(writer)?;
    }

    for point in &self.points {
      point.write::<T>(writer)?;
    }

    for table in &self.cross_tables {
      let mut table_writer: ChunkWriter = ChunkWriter::new();

      table.write::<T>(&mut table_writer)?;
      writer.write_u32::<T>(table_writer.bytes_written() as u32 + 4)?;
      writer.write_all(&table_writer.buffer)?;
    }

    log::info!("Written graphs chunk, {:?} bytes", writer.bytes_written());

    Ok(())
  }

  /// Import graphs data from provided path.
  pub fn import(_: &Path) -> io::Result<GraphsChunk> {
    Ok(GraphsChunk {
      header: GraphHeader {
        version: 0,
        vertex_count: 0,
        edges_count: 0,
        point_count: 0,
        guid: 0,
        level_count: 0,
      },
      levels: vec![],
      vertices: vec![],
      edges: vec![],
      points: vec![],
      cross_tables: vec![],
    })
  }

  /// Export graphs data into provided path.
  /// Constructs many files with contained data.
  pub fn export<T: ByteOrder>(&self, path: &Path) -> io::Result<()> {
    let mut graphs_header_config: Ini = Ini::new();

    self
      .header
      .export(&String::from("header"), &mut graphs_header_config);

    export_ini_to_file(
      &graphs_header_config,
      &mut create_export_file(&path.join("graphs_header.ltx"))?,
    )?;

    let mut graphs_level_config: Ini = Ini::new();

    for (index, level) in self.levels.iter().enumerate() {
      level.export(&index.to_string(), &mut graphs_level_config);
    }

    export_ini_to_file(
      &graphs_level_config,
      &mut create_export_file(&path.join("graphs_levels.ltx"))?,
    )?;

    let mut graphs_vertices_config: Ini = Ini::new();

    for (index, vertex) in self.vertices.iter().enumerate() {
      vertex.export(&index.to_string(), &mut graphs_vertices_config);
    }

    export_ini_to_file(
      &graphs_vertices_config,
      &mut create_export_file(&path.join("graphs_vertices.ltx"))?,
    )?;

    let mut graphs_points_config: Ini = Ini::new();

    for (index, point) in self.points.iter().enumerate() {
      point.export(&index.to_string(), &mut graphs_points_config);
    }

    export_ini_to_file(
      &graphs_points_config,
      &mut create_export_file(&path.join("graphs_points.ltx"))?,
    )?;

    let mut graphs_edges_config: Ini = Ini::new();

    for (index, edge) in self.edges.iter().enumerate() {
      edge.export(&index.to_string(), &mut graphs_edges_config);
    }

    export_ini_to_file(
      &graphs_edges_config,
      &mut create_export_file(&path.join("graphs_edges.ltx"))?,
    )?;

    // Export cross-tables as separate chunk file.
    let mut gct_file: File = create_export_file(&path.join("graphs_cross_tables.gct"))?;
    let mut cross_tables_writer: ChunkWriter = ChunkWriter::new();

    for (index, cross_table) in self.cross_tables.iter().enumerate() {
      let mut cross_table_writer: ChunkWriter = ChunkWriter::new();

      cross_table.write::<T>(&mut cross_table_writer)?;
      cross_tables_writer.write_all(&cross_table_writer.flush_chunk_into_buffer::<T>(index)?)?;
    }

    cross_tables_writer.flush_raw_into_file(&mut gct_file)?;

    log::info!("Exported graphs chunk");

    Ok(())
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

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::graph::graph_cross_table::GraphCrossTable;
  use crate::data::graph::graph_edge::GraphEdge;
  use crate::data::graph::graph_header::GraphHeader;
  use crate::data::graph::graph_level::GraphLevel;
  use crate::data::graph::graph_level_point::GraphLevelPoint;
  use crate::data::graph::graph_vertex::GraphVertex;
  use crate::data::vector_3d::Vector3d;
  use crate::file::graphs_chunk::GraphsChunk;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_empty_graphs_chunk() -> io::Result<()> {
    let filename: String = String::from("graphs_chunk_empty.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let graphs_chunk: GraphsChunk = GraphsChunk {
      header: GraphHeader {
        version: 10,
        vertex_count: 0,
        edges_count: 0,
        point_count: 0,
        guid: 4000,
        level_count: 0,
      },
      levels: vec![],
      vertices: vec![],
      edges: vec![],
      points: vec![],
      cross_tables: vec![],
    };

    graphs_chunk.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 28);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&get_test_chunk_file_sub_dir(file!(), &filename))?,
      0,
    )?;

    assert_eq!(bytes_written, 28);

    let file: FileSlice =
      open_test_resource_as_slice(&get_test_chunk_file_sub_dir(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 28 + 8);

    let chunk: Chunk = Chunk::from_file(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_graphs_chunk: GraphsChunk = GraphsChunk::read_from_chunk::<SpawnByteOrder>(chunk)?;

    assert_eq!(read_graphs_chunk, graphs_chunk);

    Ok(())
  }

  #[test]
  fn test_read_write_generic_graphs_chunk() -> io::Result<()> {
    let filename: String = String::from("graphs_chunk_generic.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let graphs_chunk: GraphsChunk = GraphsChunk {
      header: GraphHeader {
        version: 12,
        vertex_count: 2,
        edges_count: 4,
        point_count: 3,
        guid: 4500,
        level_count: 2,
      },
      levels: vec![
        GraphLevel {
          id: 25,
          name: String::from("test-level-1"),
          section: String::from("test-level-section-1"),
          guid: 4000060000,
          offset: Vector3d::new(2.5, 4.55, -6.5),
        },
        GraphLevel {
          id: 26,
          name: String::from("test-level"),
          section: String::from("test-level-section"),
          guid: 5000060000,
          offset: Vector3d::new(3.5, 5.55, -7.5),
        },
      ],
      vertices: vec![
        GraphVertex {
          level_point: Vector3d::new(12.5, 11.6, 12.3),
          game_point: Vector3d::new(1.5, -4.0, 1000.0),
          level_id: 255,
          level_vertex_id: 4000,
          vertex_type: (1, 2, 3, 4),
          edge_offset: 540,
          level_point_offset: 4000,
          edge_count: 252,
          level_point_count: 253,
        },
        GraphVertex {
          level_point: Vector3d::new(43.5, 15.6, 0.3),
          game_point: Vector3d::new(0.5, -4.0, 44.0),
          level_id: 255,
          level_vertex_id: 3000,
          vertex_type: (4, 2, 4, 4),
          edge_offset: 31,
          level_point_offset: 623,
          edge_count: 252,
          level_point_count: 23,
        },
      ],
      edges: vec![
        GraphEdge {
          game_vertex_id: 713,
          distance: 21.50,
        },
        GraphEdge {
          game_vertex_id: 714,
          distance: 8443.50,
        },
        GraphEdge {
          game_vertex_id: 715,
          distance: 4.50,
        },
        GraphEdge {
          game_vertex_id: 716,
          distance: 3.0,
        },
      ],
      points: vec![
        GraphLevelPoint {
          position: Vector3d::new(1.5, 11.6, 12.7),
          distance: 100.50,
          level_vertex_id: 8000,
        },
        GraphLevelPoint {
          position: Vector3d::new(2.5, 11.6, 12.7),
          distance: 200.50,
          level_vertex_id: 8001,
        },
        GraphLevelPoint {
          position: Vector3d::new(3.5, 11.6, 12.7),
          distance: 300.50,
          level_vertex_id: 8002,
        },
      ],
      cross_tables: vec![
        GraphCrossTable {
          version: 16,
          nodes_count: 51,
          vertex_count: 4000,
          level_guid: 1231,
          game_guid: 124,
          data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        },
        GraphCrossTable {
          version: 16,
          nodes_count: 4232,
          vertex_count: 3000,
          level_guid: 235235,
          game_guid: 423423,
          data: vec![1, 2, 3, 4, 5],
        },
      ],
    };

    graphs_chunk.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 430);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&get_test_chunk_file_sub_dir(file!(), &filename))?,
      0,
    )?;

    assert_eq!(bytes_written, 430);

    let file: FileSlice =
      open_test_resource_as_slice(&get_test_chunk_file_sub_dir(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 430 + 8);

    let chunk: Chunk = Chunk::from_file(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_graphs_chunk: GraphsChunk = GraphsChunk::read_from_chunk::<SpawnByteOrder>(chunk)?;

    assert_eq!(read_graphs_chunk, graphs_chunk);

    Ok(())
  }
}
