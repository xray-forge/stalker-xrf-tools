use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use uuid::Uuid;
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphHeader {
  #[serde(rename = "version")]
  pub version: u8,
  #[serde(rename = "verticesCount")]
  pub vertices_count: u16,
  #[serde(rename = "edgesCount")]
  pub edges_count: u32,
  #[serde(rename = "pointsCount")]
  pub points_count: u32,
  #[serde(rename = "guid")]
  pub guid: Uuid,
  #[serde(rename = "levelsCount")]
  pub levels_count: u8,
}

impl GraphHeader {
  /// Read header data from the chunk.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<GraphHeader> {
    Ok(GraphHeader {
      version: reader.read_u8()?,
      vertices_count: reader.read_u16::<T>()?,
      edges_count: reader.read_u32::<T>()?,
      points_count: reader.read_u32::<T>()?,
      guid: Uuid::from_u128(reader.read_u128::<T>()?),
      levels_count: reader.read_u8()?,
    })
  }

  /// Write graph edge data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_u8(self.version)?;
    writer.write_u16::<T>(self.vertices_count)?;
    writer.write_u32::<T>(self.edges_count)?;
    writer.write_u32::<T>(self.points_count)?;
    writer.write_u128::<T>(self.guid.as_u128())?;
    writer.write_u8(self.levels_count)?;

    Ok(())
  }

  /// Import graph header from ini file.
  pub fn import(config: &Ltx) -> io::Result<GraphHeader> {
    let props: &Section = config
      .section("header")
      .unwrap_or_else(|| panic!("Graph section 'header' should be defined in ltx file"));

    Ok(GraphHeader {
      version: read_ini_field("version", props)?,
      vertices_count: read_ini_field("vertex_count", props)?,
      edges_count: read_ini_field("edges_count", props)?,
      points_count: read_ini_field("point_count", props)?,
      levels_count: read_ini_field("level_count", props)?,
      guid: read_ini_field("guid", props)?,
    })
  }

  /// Export graph header data into level ini.
  pub fn export(&self, ini: &mut Ltx) {
    ini
      .with_section("header")
      .set("version", self.version.to_string())
      .set("vertex_count", self.vertices_count.to_string())
      .set("edges_count", self.edges_count.to_string())
      .set("point_count", self.points_count.to_string())
      .set("level_count", self.levels_count.to_string())
      .set("guid", self.guid.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::graph::graph_header::GraphHeader;
  use crate::export::file::open_ini_config;
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
  use uuid::uuid;
  use xray_ltx::Ltx;

  #[test]
  fn test_read_write_simple_graph_level_point() -> io::Result<()> {
    let filename: String = String::from("graph_header.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let header: GraphHeader = GraphHeader {
      version: 16,
      vertices_count: 4000,
      edges_count: 230_250,
      points_count: 600_500,
      guid: uuid!("78e55023-10b1-426f-9247-bb680e5fe0b7"),
      levels_count: 5,
    };

    header.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 28);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&get_relative_test_sample_file_path(
        file!(),
        &filename,
      ))?,
      0,
    )?;

    assert_eq!(bytes_written, 28);

    let file: FileSlice =
      open_test_resource_as_slice(&get_relative_test_sample_file_path(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 28 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_header: GraphHeader = GraphHeader::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_header, header);

    Ok(())
  }

  #[test]
  fn test_import_export_object() -> io::Result<()> {
    let header: GraphHeader = GraphHeader {
      version: 16,
      vertices_count: 6434,
      edges_count: 456,
      points_count: 5635,
      levels_count: 25,
      guid: uuid!("23e55044-10b1-426f-9247-bb680e5fe0c8"),
    };

    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "graph_header.ini");
    let mut file: File =
      overwrite_test_relative_resource_as_file(config_path.to_str().expect("Valid path"))?;
    let mut ltx: Ltx = Ltx::new();

    header.export(&mut ltx);
    ltx.write_to(&mut file)?;

    let read_header: GraphHeader = GraphHeader::import(&open_ini_config(config_path)?)?;

    assert_eq!(read_header, header);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize_object() -> io::Result<()> {
    let header: GraphHeader = GraphHeader {
      version: 12,
      vertices_count: 2341,
      edges_count: 12513,
      points_count: 43231,
      levels_count: 31,
      guid: uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"),
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialized.json"),
    )?;

    file.write_all(json!(header).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(header, serde_json::from_str::<GraphHeader>(&serialized)?);

    Ok(())
  }
}
