use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::export::file_import::read_ini_field;
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
    Ok(GraphEdge {
      game_vertex_id: chunk.read_u16::<T>()?,
      distance: chunk.read_f32::<T>()?,
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
    let props: &Properties = config.section(Some(section)).unwrap_or_else(|| {
      panic!("Graph section '{section}' should be defined in level point ltx file")
    });

    Ok(GraphEdge {
      game_vertex_id: read_ini_field("game_vertex_id", props)?,
      distance: read_ini_field("distance", props)?,
    })
  }

  /// Export graph edge data into ini.
  pub fn export(&self, section: &str, ini: &mut Ini) {
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
