use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::types::Vector3d;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::Ini;
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

  /// Export graph level point data into ini.
  pub fn export(&self, section: &String, ini: &mut Ini) {
    ini
      .with_section(Some(section))
      .set("position", self.position.0.to_string()) // todo: Write vector.
      .set("level_vertex_id", self.level_vertex_id.to_string())
      .set("distance", self.distance.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::graph::graph_level_point::GraphLevelPoint;
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
      position: (10.5, 11.6, 12.7),
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

    let mut chunk: Chunk = Chunk::from_file(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_point: GraphLevelPoint =
      GraphLevelPoint::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_point, point);

    Ok(())
  }
}
