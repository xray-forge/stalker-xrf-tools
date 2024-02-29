use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct GraphEdge {
  pub game_vertex_id: u16,
  pub distance: f32,
}

impl GraphEdge {
  /// Read edge from chunk.
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<GraphEdge> {
    let game_vertex_id: u16 = chunk.read_u16::<T>()?;
    let distance: f32 = chunk.read_f32::<T>()?;

    Ok(GraphEdge {
      game_vertex_id,
      distance,
    })
  }

  /// Write graph edge data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_u16::<T>(self.game_vertex_id)?;
    writer.write_f32::<T>(self.distance)?;

    Ok(())
  }

  /// Import graph edge from ini file.
  pub fn import(section: &str, config: &Ini) -> io::Result<GraphEdge> {
    let props: &Properties = config.section(Some(section)).expect(
      format!("Graph section '{section}' should be defined in level point ltx file.").as_str(),
    );

    Ok(GraphEdge {
      game_vertex_id: props
        .get("game_vertex_id")
        .expect("'game_vertex_id' to be in graph config")
        .parse::<u16>()
        .expect("'game_vertex_id' to be valid u16"),
      distance: props
        .get("distance")
        .expect("'distance' to be in graph config")
        .parse::<f32>()
        .expect("'distance' to be valid f32"),
    })
  }
  /// Export graph edge data into ini.
  pub fn export(&self, section: &String, ini: &mut Ini) {
    ini
      .with_section(Some(section))
      .set("game_vertex_id", self.game_vertex_id.to_string())
      .set("distance", self.distance.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::graph::graph_edge::GraphEdge;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_simple_graph_level_point() -> io::Result<()> {
    let filename: String = String::from("graph_edge_simple.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let edge: GraphEdge = GraphEdge {
      game_vertex_id: 713,
      distance: 400.50,
    };

    edge.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 6);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&get_test_chunk_file_sub_dir(file!(), &filename))?,
      0,
    )?;

    assert_eq!(bytes_written, 6);

    let file: FileSlice =
      open_test_resource_as_slice(&get_test_chunk_file_sub_dir(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 6 + 8);

    let mut chunk: Chunk = Chunk::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_edge: GraphEdge = GraphEdge::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_edge, edge);

    Ok(())
  }
}
