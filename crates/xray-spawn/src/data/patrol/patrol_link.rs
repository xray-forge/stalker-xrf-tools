use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Debug, PartialEq)]
pub struct PatrolLink {
  pub index: u32,
  pub links: Vec<(u32, f32)>,
}

impl PatrolLink {
  /// Read links from chunk file.
  pub fn read_list_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<Vec<PatrolLink>> {
    let mut links: Vec<PatrolLink> = Vec::new();

    while chunk.has_data() {
      links.push(PatrolLink::read_from_chunk::<T>(chunk)?);
    }

    if chunk.read_bytes_remain() > 0 {
      log::warn!("Data to read remains in patrol link")
    }

    assert!(
      chunk.is_ended(),
      "Chunk data should be read for patrol links."
    );

    Ok(links)
  }

  /// Read patrol link from chunk.
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<PatrolLink> {
    let index: u32 = chunk.read_u32::<T>()?;
    let count: u32 = chunk.read_u32::<T>()?;

    let mut vertices: Vec<(u32, f32)> = Vec::new();

    for _ in 0..count {
      let to: u32 = chunk.read_u32::<T>()?;
      let weight: f32 = chunk.read_f32::<T>()?;

      vertices.push((to, weight));
    }

    assert_eq!(vertices.len(), count as usize);

    Ok(PatrolLink {
      index,
      links: vertices,
    })
  }

  /// Write list patrol links into chunk writer.
  pub fn write_list<T: ByteOrder>(
    links: &Vec<PatrolLink>,
    writer: &mut ChunkWriter,
  ) -> io::Result<()> {
    for link in links {
      link.write::<T>(writer)?;
    }

    Ok(())
  }

  /// Write patrol link data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_u32::<T>(self.index)?;
    writer.write_u32::<T>(self.links.len() as u32)?;

    for (to, weight) in &self.links {
      writer.write_u32::<T>(*to)?;
      writer.write_f32::<T>(*weight)?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::patrol::patrol_link::PatrolLink;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_simple_patrol_link() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();

    let link: PatrolLink = PatrolLink {
      index: 1000,
      links: vec![(10, 1.5), (11, 2.5), (12, 3.5)],
    };

    link.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 32);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(get_test_chunk_file_sub_dir(
        file!(),
        String::from("patrol_vertex_simple.chunk"),
      ))?,
      0,
    )?;

    assert_eq!(bytes_written, 32);

    let file: FileSlice = open_test_resource_as_slice(get_test_chunk_file_sub_dir(
      file!(),
      String::from("patrol_vertex_simple.chunk"),
    ))?;

    assert_eq!(file.bytes_remaining(), 32 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_link: PatrolLink = PatrolLink::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_link, link);

    Ok(())
  }

  #[test]
  fn test_read_write_list_of_patrol_links() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();

    let links: Vec<PatrolLink> = vec![
      PatrolLink {
        index: 1000,
        links: vec![(10, 1.5), (11, 2.5), (12, 3.5)],
      },
      PatrolLink {
        index: 1001,
        links: vec![(20, 1.5)],
      },
    ];

    PatrolLink::write_list::<SpawnByteOrder>(&links, &mut writer)?;

    assert_eq!(writer.bytes_written(), 48);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(get_test_chunk_file_sub_dir(
        file!(),
        String::from("patrol_vertex_list.chunk"),
      ))?,
      0,
    )?;

    assert_eq!(bytes_written, 48);

    let file: FileSlice = open_test_resource_as_slice(get_test_chunk_file_sub_dir(
      file!(),
      String::from("patrol_vertex_list.chunk"),
    ))?;

    assert_eq!(file.bytes_remaining(), 48 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let from_file: Vec<PatrolLink> =
      PatrolLink::read_list_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(from_file, links);

    Ok(())
  }
}
