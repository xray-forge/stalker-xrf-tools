use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use xray_ltx::{Ltx, Properties};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphEdge {
  #[serde(rename = "gameVertexId")]
  pub game_vertex_id: u16,
  #[serde(rename = "distance")]
  pub distance: f32,
}

impl GraphEdge {
  /// Read edge from chunk.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<GraphEdge> {
    Ok(GraphEdge {
      game_vertex_id: reader.read_u16::<T>()?,
      distance: reader.read_f32::<T>()?,
    })
  }

  /// Write graph edge data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_u16::<T>(self.game_vertex_id)?;
    writer.write_f32::<T>(self.distance)?;

    Ok(())
  }

  /// Import graph edge from ini file.
  pub fn import(section: &str, config: &Ltx) -> io::Result<GraphEdge> {
    let props: &Properties = config.section(Some(section)).unwrap_or_else(|| {
      panic!("Graph section '{section}' should be defined in level point ltx file")
    });

    Ok(GraphEdge {
      game_vertex_id: read_ini_field("game_vertex_id", props)?,
      distance: read_ini_field("distance", props)?,
    })
  }

  /// Export graph edge data into ini.
  pub fn export(&self, section: &str, ini: &mut Ltx) {
    ini
      .with_section(Some(section))
      .set("game_vertex_id", self.game_vertex_id.to_string())
      .set("distance", self.distance.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::graph::graph_edge::GraphEdge;
  use crate::export::file::{export_ini_to_file, open_ini_config};
  use crate::test::file::read_file_as_string;
  use crate::test::utils::{
    get_absolute_test_sample_file_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_test_relative_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io;
  use std::io::{Seek, SeekFrom, Write};
  use std::path::Path;
  use xray_ltx::Ltx;

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
      &mut overwrite_test_relative_resource_as_file(&get_relative_test_sample_file_path(
        file!(),
        &filename,
      ))?,
      0,
    )?;

    assert_eq!(bytes_written, 6);

    let file: FileSlice =
      open_test_resource_as_slice(&get_relative_test_sample_file_path(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 6 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_edge: GraphEdge = GraphEdge::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_edge, edge);

    Ok(())
  }

  #[test]
  fn test_import_export_object() -> io::Result<()> {
    let edge: GraphEdge = GraphEdge {
      game_vertex_id: 352,
      distance: 2554.50,
    };

    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "graph_edge.ini");
    let mut file: File =
      overwrite_test_relative_resource_as_file(config_path.to_str().expect("Valid path"))?;
    let mut ltx: Ltx = Ltx::new();

    edge.export("graph_edge", &mut ltx);
    export_ini_to_file(&ltx, &mut file)?;

    let read_header: GraphEdge = GraphEdge::import("graph_edge", &open_ini_config(config_path)?)?;

    assert_eq!(read_header, edge);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize_object() -> io::Result<()> {
    let edge: GraphEdge = GraphEdge {
      game_vertex_id: 713,
      distance: 400.50,
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialized.json"),
    )?;

    file.write_all(json!(edge).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(edge, serde_json::from_str::<GraphEdge>(&serialized)?);

    Ok(())
  }
}
