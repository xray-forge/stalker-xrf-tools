use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;

use crate::data::vector_3d::Vector3d;
use crate::export::file_export::export_vector_to_string;
use crate::export::file_import::{read_ini_field, read_ini_u32_bytes_field};
use crate::types::U32Bytes;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct GraphVertex {
  pub level_point: Vector3d<f32>,
  pub game_point: Vector3d<f32>,
  pub level_id: u8,
  pub level_vertex_id: u32,
  pub vertex_type: U32Bytes,
  pub edge_offset: u32,
  pub level_point_offset: u32,
  pub edge_count: u8,
  pub level_point_count: u8,
}

impl GraphVertex {
  /// Read graph vertex data from the chunk.
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<GraphVertex> {
    Ok(GraphVertex {
      level_point: chunk.read_f32_3d_vector::<T>()?,
      game_point: chunk.read_f32_3d_vector::<T>()?,
      level_id: chunk.read_u8()?,
      level_vertex_id: chunk.read_u24::<T>()?,
      vertex_type: chunk.read_u32_bytes()?,
      edge_offset: chunk.read_u32::<T>()?,
      level_point_offset: chunk.read_u32::<T>()?,
      edge_count: chunk.read_u8()?,
      level_point_count: chunk.read_u8()?,
    })
  }

  /// Write graph vertex data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_f32_3d_vector::<T>(&self.level_point)?;
    writer.write_f32_3d_vector::<T>(&self.game_point)?;
    writer.write_u8(self.level_id)?;
    writer.write_u24::<T>(self.level_vertex_id)?;
    writer.write_u32_bytes(&self.vertex_type)?;
    writer.write_u32::<T>(self.edge_offset)?;
    writer.write_u32::<T>(self.level_point_offset)?;
    writer.write_u8(self.edge_count)?;
    writer.write_u8(self.level_point_count)?;

    Ok(())
  }

  /// Import graph vertex from ini file.
  pub fn import(section: &str, config: &Ini) -> io::Result<GraphVertex> {
    let props: &Properties = config.section(Some(section)).unwrap_or_else(|| {
      panic!("Graph section '{section}' should be defined in graph vertex ltx file")
    });

    Ok(GraphVertex {
      level_point: read_ini_field("level_point", props)?,
      game_point: read_ini_field("game_point", props)?,
      level_id: read_ini_field("level_id", props)?,
      level_vertex_id: read_ini_field("level_vertex_id", props)?,
      vertex_type: read_ini_u32_bytes_field("vertex_type", props)?,
      edge_offset: read_ini_field("edge_offset", props)?,
      level_point_offset: read_ini_field("level_point_offset", props)?,
      edge_count: read_ini_field("edge_count", props)?,
      level_point_count: read_ini_field("level_point_count", props)?,
    })
  }

  /// Export graph vertex data into ini.
  pub fn export(&self, section: &String, ini: &mut Ini) {
    ini
      .with_section(Some(section))
      .set("level_point", self.level_point.to_string())
      .set("game_point", self.game_point.to_string())
      .set("level_id", self.level_id.to_string())
      .set("level_vertex_id", self.level_vertex_id.to_string())
      .set("edge_offset", self.edge_offset.to_string())
      .set("level_point_offset", self.level_point_offset.to_string())
      .set("edge_count", self.edge_count.to_string())
      .set("level_point_count", self.level_point_count.to_string())
      .set(
        "vertex_type",
        export_vector_to_string(&vec![
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
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::graph::graph_vertex::GraphVertex;
  use crate::data::vector_3d::Vector3d;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

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
      edge_offset: 540,
      level_point_offset: 4000,
      edge_count: 252,
      level_point_count: 253,
    };

    vertex.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 42);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&get_test_chunk_file_sub_dir(file!(), &filename))?,
      0,
    )?;

    assert_eq!(bytes_written, 42);

    let file: FileSlice =
      open_test_resource_as_slice(&get_test_chunk_file_sub_dir(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 42 + 8);

    let mut chunk: Chunk = Chunk::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_vertex: GraphVertex = GraphVertex::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_vertex, vertex);

    Ok(())
  }
}
