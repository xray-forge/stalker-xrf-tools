use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::vector_3d::Vector3d;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct GraphLevelPoint {
  pub position: Vector3d,
  pub level_vertex_id: u32,
  pub distance: f32,
}

impl GraphLevelPoint {
  /// Read level point from chunk.
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<GraphLevelPoint> {
    let position: Vector3d = chunk.read_f32_3d_vector::<T>()?;
    let level_vertex_id: u32 = chunk.read_u32::<T>()?;
    let distance: f32 = chunk.read_f32::<T>()?;

    Ok(GraphLevelPoint {
      position,
      level_vertex_id,
      distance,
    })
  }

  /// Write level point data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_f32_3d_vector::<T>(&self.position)?;
    writer.write_u32::<T>(self.level_vertex_id)?;
    writer.write_f32::<T>(self.distance)?;

    Ok(())
  }

  /// Import graph level point from ini file.
  pub fn import(section: &str, config: &Ini) -> io::Result<GraphLevelPoint> {
    let props: &Properties = config.section(Some(section)).expect(
      format!("Graph section '{section}' should be defined in level point ltx file.").as_str(),
    );

    Ok(GraphLevelPoint {
      position: props
        .get("position")
        .expect("'position' to be in graph config")
        .parse::<Vector3d>()
        .expect("'position' to be valid Vector3d"),
      level_vertex_id: props
        .get("level_vertex_id")
        .expect("'level_vertex_id' to be in graph config")
        .parse::<u32>()
        .expect("'level_vertex_id' to be valid u32"),
      distance: props
        .get("distance")
        .expect("'distance' to be in graph config")
        .parse::<f32>()
        .expect("'distance' to be valid f32"),
    })
  }

  /// Export graph level point data into ini.
  pub fn export(&self, section: &String, ini: &mut Ini) {
    ini
      .with_section(Some(section))
      .set("position", self.position.to_string())
      .set("level_vertex_id", self.level_vertex_id.to_string())
      .set("distance", self.distance.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::graph::graph_level_point::GraphLevelPoint;
  use crate::data::vector_3d::Vector3d;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_simple_graph_level_point() -> io::Result<()> {
    let filename: String = String::from("graph_level_point_simple.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let point: GraphLevelPoint = GraphLevelPoint {
      position: Vector3d::new(10.5, 11.6, 12.7),
      distance: 400.50,
      level_vertex_id: 8000,
    };

    point.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 20);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&get_test_chunk_file_sub_dir(file!(), &filename))?,
      0,
    )?;

    assert_eq!(bytes_written, 20);

    let file: FileSlice =
      open_test_resource_as_slice(&get_test_chunk_file_sub_dir(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 20 + 8);

    let mut chunk: Chunk = Chunk::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_point: GraphLevelPoint =
      GraphLevelPoint::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_point, point);

    Ok(())
  }
}
