use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatrolLink {
  #[serde(rename = "index")]
  pub index: u32,
  #[serde(rename = "links")]
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
  pub fn import(section_name: &str, config: &Ltx) -> io::Result<PatrolLink> {
    let section: &Section = config.section(section_name).unwrap_or_else(|| {
      panic!("Patrol point link '{section_name}' should be defined in ltx file")
    });

    let index: u32 = read_ini_field("index", section)?;
    let count: usize = read_ini_field("count", section)?;

    let mut links: Vec<(u32, f32)> = Vec::new();

    for link in 0..count {
      links.push((
        read_ini_field(&format!("from.{link}"), section)?,
        read_ini_field(&format!("weight.{link}"), section)?,
      ))
    }

    assert_eq!(links.len(), count);

    Ok(PatrolLink { index, links })
  }

  /// Export patrol link data into ini.
  pub fn export(&self, section: &str, ini: &mut Ltx) {
    ini
      .with_section(section)
      .set("index", self.index.to_string())
      .set("count", self.links.len().to_string());

    for (index, (from, weight)) in self.links.iter().enumerate() {
      ini
        .with_section(section)
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
    open_test_resource_as_slice, overwrite_file, overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write_simple_patrol_link() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "patrol_link_simple.chunk");

    let link: PatrolLink = PatrolLink {
      index: 1000,
      links: vec![(10, 1.5), (11, 2.5), (12, 3.5)],
    };

    link.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 32);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
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
    let filename: String = get_relative_test_sample_file_path(file!(), "patrol_link_list.chunk");

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
      &mut overwrite_test_relative_resource_as_file(&filename)?,
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

  #[test]
  fn test_import_export_object() -> io::Result<()> {
    let link: PatrolLink = PatrolLink {
      index: 1000,
      links: vec![(10, 1.5), (11, 2.5), (12, 3.5)],
    };

    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "patrol_link.ini");
    let mut file: File = overwrite_file(&config_path)?;
    let mut ltx: Ltx = Ltx::new();

    link.export("patrol_link", &mut ltx);
    ltx.write_to(&mut file)?;

    let read_point: PatrolLink = PatrolLink::import("patrol_link", &open_ini_config(config_path)?)?;

    assert_eq!(read_point, link);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize_object() -> io::Result<()> {
    let link: PatrolLink = PatrolLink {
      index: 1000,
      links: vec![(10, 1.5), (11, 2.5), (12, 3.5)],
    };

    let mut file: File = overwrite_file(&get_absolute_test_sample_file_path(
      file!(),
      "serialized.json",
    ))?;

    file.write_all(json!(link).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(link, serde_json::from_str::<PatrolLink>(&serialized)?);

    Ok(())
  }
}
