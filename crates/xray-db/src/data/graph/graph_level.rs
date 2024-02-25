use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::types::Vector3d;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

/// `GameGraph::SLevel::load` in xray codebase.
#[derive(Clone, Debug, PartialEq)]
pub struct GraphLevel {
  pub name: String,
  pub offset: Vector3d<f32>,
  pub id: u8,
  pub section: String,
  pub guid: u128,
}

impl GraphLevel {
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<GraphLevel> {
    let name: String = chunk.read_null_terminated_string()?;
    let offset: Vector3d = chunk.read_f32_3d_vector::<T>()?;
    let id: u8 = chunk.read_u8()?;
    let section: String = chunk.read_null_terminated_string()?;
    let guid: u128 = chunk.read_u128::<T>()?;

    Ok(GraphLevel {
      name,
      offset,
      id,
      section,
      guid,
    })
  }

  /// Write graph level data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_null_terminated_string(&self.name)?;
    writer.write_f32_3d_vector::<T>(&self.offset)?;
    writer.write_u8(self.id)?;
    writer.write_null_terminated_string(&self.section)?;
    writer.write_u128::<T>(self.guid)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::graph::graph_level::GraphLevel;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_simple_graph_level_point() -> io::Result<()> {
    let filename: String = String::from("graph_level.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let level: GraphLevel = GraphLevel {
      id: 255,
      name: String::from("test-level"),
      section: String::from("test-level-section"),
      guid: 4000060000,
      offset: (0.5, 5.55, -1.5),
    };

    level.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 59);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&get_test_chunk_file_sub_dir(file!(), &filename))?,
      0,
    )?;

    assert_eq!(bytes_written, 59);

    let file: FileSlice =
      open_test_resource_as_slice(&get_test_chunk_file_sub_dir(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 59 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_level: GraphLevel = GraphLevel::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_level, level);

    Ok(())
  }
}