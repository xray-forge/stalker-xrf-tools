use crate::spawn::chunk::Chunk;
use crate::spawn::data::level::Level;
use crate::spawn::data::vertex::Vertex;
use byteorder::{LittleEndian, ReadBytesExt};
use fileslice::FileSlice;
use std::fmt;

pub struct GraphsChunk {
  pub id: u32,
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
  pub fn from_chunk(file: &mut FileSlice, chunk: &Chunk) -> Option<GraphsChunk> {
    let mut file: FileSlice = chunk.in_slice(file);

    log::info!(
      "Parsing level graphs, {:?} -> {:?}",
      file.start_pos(),
      file.end_pos()
    );

    let version: u8 = file.read_u8().unwrap();
    let vertex_count: u16 = file.read_u16::<LittleEndian>().unwrap();
    let edge_count: u32 = file.read_u32::<LittleEndian>().unwrap();
    let point_count: u32 = file.read_u32::<LittleEndian>().unwrap();
    let guid: u128 = file.read_u128::<LittleEndian>().unwrap();
    let level_count: u8 = file.read_u8().unwrap();

    let mut levels: Vec<Level> = Vec::new();
    let mut vertices: Vec<Vertex> = Vec::new();

    for _ in 0..level_count {
      levels.push(Level::from_file(&mut file))
    }

    for _ in 0..vertex_count {
      vertices.push(Vertex::from_file(&mut file));
    }

    log::info!(
      "Parsed graphs v{version}, {:?} / {:?}, {:?} left",
      file.cursor_pos(),
      file.end_pos(),
      file.end_pos() - file.cursor_pos()
    );

    assert_eq!(levels.len(), level_count as usize);
    assert_eq!(vertices.len(), vertex_count as usize);
    // todo: assert_eq!(file.cursor_pos(), file.end_pos());

    return Some(GraphsChunk {
      id: chunk.id,
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
      "GraphsChunk {{ id: {}, version: {}, vertex_count: {}, edge_count: {}, point_count: {},\
       guid: {}, size: {}, levels: Vector[{}], vertices: Vector[{}] }}",
      self.id,
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
