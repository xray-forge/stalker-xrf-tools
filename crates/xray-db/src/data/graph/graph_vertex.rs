use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;

use crate::data::vector_3d::Vector3d;
use crate::export::file_export::export_vector_to_string;
use crate::export::file_import::{read_ini_field, read_ini_u32_bytes_field};
use crate::types::U32Bytes;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphVertex {
  #[serde(rename = "levelPoint")]
  pub level_point: Vector3d<f32>,
  #[serde(rename = "gamePoint")]
  pub game_point: Vector3d<f32>,
  #[serde(rename = "levelId")]
  pub level_id: u8,
  #[serde(rename = "levelVertexId")]
  pub level_vertex_id: u32,
  #[serde(rename = "vertexType")]
  pub vertex_type: U32Bytes,
  #[serde(rename = "edgesOffset")]
  pub edges_offset: u32,
  #[serde(rename = "levelPointsOffset")]
  pub level_points_offset: u32,
  #[serde(rename = "edgesCount")]
  pub edges_count: u8,
  #[serde(rename = "levelPointsCount")]
  pub level_points_count: u8,
}

impl GraphVertex {
  /// Read graph vertex data from the chunk.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<GraphVertex> {
    Ok(GraphVertex {
      level_point: reader.read_f32_3d_vector::<T>()?,
      game_point: reader.read_f32_3d_vector::<T>()?,
      level_id: reader.read_u8()?,
      level_vertex_id: reader.read_u24::<T>()?,
      vertex_type: reader.read_u32_bytes()?,
      edges_offset: reader.read_u32::<T>()?,
      level_points_offset: reader.read_u32::<T>()?,
      edges_count: reader.read_u8()?,
      level_points_count: reader.read_u8()?,
    })
  }

  /// Write graph vertex data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_f32_3d_vector::<T>(&self.level_point)?;
    writer.write_f32_3d_vector::<T>(&self.game_point)?;
    writer.write_u8(self.level_id)?;
    writer.write_u24::<T>(self.level_vertex_id)?;
    writer.write_u32_bytes(&self.vertex_type)?;
    writer.write_u32::<T>(self.edges_offset)?;
    writer.write_u32::<T>(self.level_points_offset)?;
    writer.write_u8(self.edges_count)?;
    writer.write_u8(self.level_points_count)?;

    Ok(())
  }

  /// Import graph vertex from ini file.
  pub fn import(section_name: &str, config: &Ltx) -> io::Result<GraphVertex> {
    let section: &Section = config.section(section_name).unwrap_or_else(|| {
      panic!("Graph section '{section_name}' should be defined in graph vertex ltx file")
    });

    Ok(GraphVertex {
      level_point: read_ini_field("level_point", section)?,
      game_point: read_ini_field("game_point", section)?,
      level_id: read_ini_field("level_id", section)?,
      level_vertex_id: read_ini_field("level_vertex_id", section)?,
      vertex_type: read_ini_u32_bytes_field("vertex_type", section)?,
      edges_offset: read_ini_field("edge_offset", section)?,
      level_points_offset: read_ini_field("level_point_offset", section)?,
      edges_count: read_ini_field("edge_count", section)?,
      level_points_count: read_ini_field("level_point_count", section)?,
    })
  }

  /// Export graph vertex data into ini.
  pub fn export(&self, section: &str, ini: &mut Ltx) {
    ini
      .with_section(section)
      .set("level_point", self.level_point.to_string())
      .set("game_point", self.game_point.to_string())
      .set("level_id", self.level_id.to_string())
      .set("level_vertex_id", self.level_vertex_id.to_string())
      .set("edge_offset", self.edges_offset.to_string())
      .set("level_point_offset", self.level_points_offset.to_string())
      .set("edge_count", self.edges_count.to_string())
      .set("level_point_count", self.level_points_count.to_string())
      .set(
        "vertex_type",
        export_vector_to_string(&[
          self.vertex_type.0,
          self.vertex_type.1,
          self.vertex_type.2,
          self.vertex_type.3,
        ]),
      );
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::graph::graph_vertex::GraphVertex;
  use crate::data::vector_3d::Vector3d;
  use crate::export::file::open_ini_config;
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io;
  use std::io::{Seek, SeekFrom, Write};
  use std::path::Path;
  use xray_ltx::Ltx;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_sample_file_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write_simple_graph_level_point() -> io::Result<()> {
    let filename: String = String::from("graph_vertex.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let vertex: GraphVertex = GraphVertex {
      level_point: Vector3d::new(10.5, 11.6, 12.3),
      game_point: Vector3d::new(0.5, -4.0, 1000.0),
      level_id: 255,
      level_vertex_id: 4000,
      vertex_type: (1, 2, 3, 4),
      edges_offset: 540,
      level_points_offset: 4000,
      edges_count: 252,
      level_points_count: 253,
    };

    vertex.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 42);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&get_relative_test_sample_file_path(
        file!(),
        &filename,
      ))?,
      0,
    )?;

    assert_eq!(bytes_written, 42);

    let file: FileSlice =
      open_test_resource_as_slice(&get_relative_test_sample_file_path(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 42 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_vertex: GraphVertex = GraphVertex::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_vertex, vertex);

    Ok(())
  }

  #[test]
  fn test_import_export_object() -> io::Result<()> {
    let vertex: GraphVertex = GraphVertex {
      level_point: Vector3d::new(32.5, 523.6, 342.3),
      game_point: Vector3d::new(0.23, -4.0, 123.0),
      level_id: 53,
      level_vertex_id: 5462,
      vertex_type: (1, 2, 3, 4),
      edges_offset: 5643,
      level_points_offset: 2134,
      edges_count: 14,
      level_points_count: 63,
    };

    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "graph_vertex.ini");
    let mut file: File =
      overwrite_test_relative_resource_as_file(config_path.to_str().expect("Valid path"))?;
    let mut ltx: Ltx = Ltx::new();

    vertex.export("graph_vertex", &mut ltx);
    ltx.write_to(&mut file)?;

    let read_vertex: GraphVertex =
      GraphVertex::import("graph_vertex", &open_ini_config(config_path)?)?;

    assert_eq!(read_vertex, vertex);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize_object() -> io::Result<()> {
    let vertex: GraphVertex = GraphVertex {
      level_point: Vector3d::new(25.5, 15.6, 43.3),
      game_point: Vector3d::new(0.44, -4.0, 1000.0),
      level_id: 213,
      level_vertex_id: 5234,
      vertex_type: (1, 2, 3, 4),
      edges_offset: 3242,
      level_points_offset: 6345,
      edges_count: 211,
      level_points_count: 234,
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialized.json"),
    )?;

    file.write_all(json!(vertex).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(vertex, serde_json::from_str::<GraphVertex>(&serialized)?);

    Ok(())
  }
}
