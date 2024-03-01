use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct PatrolLink {
  pub index: u32,
  pub links: Vec<(u32, f32)>,
}

impl PatrolLink {
  /// Read links from chunk file.
  pub fn read_list<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<Vec<PatrolLink>> {
    let mut links: Vec<PatrolLink> = Vec::new();

    while reader.has_data() {
      links.push(PatrolLink::read::<T>(reader)?);
    }

    if reader.read_bytes_remain() > 0 {
      log::warn!("Data to read remains in patrol link")
    }

    assert!(
      reader.is_ended(),
      "Chunk data should be read for patrol links"
    );

    Ok(links)
  }

  /// Read patrol link from chunk.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<PatrolLink> {
    let index: u32 = reader.read_u32::<T>()?;
    let count: u32 = reader.read_u32::<T>()?;

    let mut vertices: Vec<(u32, f32)> = Vec::new();

    for _ in 0..count {
      let to: u32 = reader.read_u32::<T>()?; // from->to in u16.
      let weight: f32 = reader.read_f32::<T>()?;

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

  /// Import patrol point link from ini config.
  pub fn import(section: &str, config: &Ini) -> io::Result<PatrolLink> {
    let props: &Properties = config
      .section(Some(section))
      .unwrap_or_else(|| panic!("Patrol point link '{section}' should be defined in ltx file"));

    let index: u32 = read_ini_field("index", props)?;
    let count: usize = read_ini_field("count", props)?;

    let mut links: Vec<(u32, f32)> = Vec::new();

    for link in 0..count {
      links.push((
        read_ini_field(&format!("from.{link}"), props)?,
        read_ini_field(&format!("weight.{link}"), props)?,
      ))
    }

    assert_eq!(links.len(), count);

    Ok(PatrolLink { index, links })
  }

  /// Export patrol link data into ini.
  pub fn export(&self, section: &str, ini: &mut Ini) {
    ini
      .with_section(Some(section))
      .set("index", self.index.to_string())
      .set("count", self.links.len().to_string());

    for (index, (from, weight)) in self.links.iter().enumerate() {
      ini
        .with_section(Some(section))
        .set(format!("from.{index}"), from.to_string())
        .set(format!("weight.{index}"), weight.to_string());
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::patrol::patrol_link::PatrolLink;
  use crate::test::utils::{
    get_test_sample_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_simple_patrol_link() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_test_sample_file_sub_dir(file!(), "patrol_vertex_simple.chunk");

    let link: PatrolLink = PatrolLink {
      index: 1000,
      links: vec![(10, 1.5), (11, 2.5), (12, 3.5)],
    };

    link.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 32);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 32);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 32 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_link: PatrolLink = PatrolLink::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_link, link);

    Ok(())
  }

  #[test]
  fn test_read_write_list_of_patrol_links() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_test_sample_file_sub_dir(file!(), "patrol_vertex_list.chunk");

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
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 48);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 48 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let from_file: Vec<PatrolLink> = PatrolLink::read_list::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(from_file, links);

    Ok(())
  }
}
