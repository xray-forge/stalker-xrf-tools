use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct GraphHeader {
  pub version: u8,
  pub vertex_count: u16,
  pub edge_count: u32,
  pub point_count: u32,
  pub guid: u128,
  pub level_count: u8,
}

impl GraphHeader {
  /// Read header data from the chunk.
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<GraphHeader> {
    let version: u8 = chunk.read_u8()?;
    let vertex_count: u16 = chunk.read_u16::<T>()?;
    let edge_count: u32 = chunk.read_u32::<T>()?;
    let point_count: u32 = chunk.read_u32::<T>()?;
    let guid: u128 = chunk.read_u128::<T>()?;
    let level_count: u8 = chunk.read_u8()?;

    Ok(GraphHeader {
      version,
      vertex_count,
      edge_count,
      point_count,
      guid,
      level_count,
    })
  }

  /// Write graph edge data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_u8(self.version)?;
    writer.write_u16::<T>(self.vertex_count)?;
    writer.write_u32::<T>(self.edge_count)?;
    writer.write_u32::<T>(self.point_count)?;
    writer.write_u128::<T>(self.guid)?;
    writer.write_u8(self.level_count)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::graph::graph_header::GraphHeader;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_simple_graph_level_point() -> io::Result<()> {
    let filename: String = String::from("graph_header.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let header: GraphHeader = GraphHeader {
      version: 16,
      vertex_count: 4000,
      edge_count: 230_250,
      point_count: 600_500,
      guid: 4321,
      level_count: 5,
    };

    header.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 28);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&get_test_chunk_file_sub_dir(file!(), &filename))?,
      0,
    )?;

    assert_eq!(bytes_written, 28);

    let file: FileSlice =
      open_test_resource_as_slice(&get_test_chunk_file_sub_dir(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 28 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_header: GraphHeader = GraphHeader::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_header, header);

    Ok(())
  }
}
