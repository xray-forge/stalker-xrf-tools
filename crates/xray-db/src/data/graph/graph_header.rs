use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use std::io;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphHeader {
  pub version: u8,
  pub vertex_count: u16,
  pub edges_count: u32,
  pub point_count: u32,
  pub guid: Uuid,
  pub level_count: u8,
}

impl GraphHeader {
  /// Read header data from the chunk.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<GraphHeader> {
    Ok(GraphHeader {
      version: reader.read_u8()?,
      vertex_count: reader.read_u16::<T>()?,
      edges_count: reader.read_u32::<T>()?,
      point_count: reader.read_u32::<T>()?,
      guid: Uuid::from_u128(reader.read_u128::<T>()?),
      level_count: reader.read_u8()?,
    })
  }

  /// Write graph edge data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_u8(self.version)?;
    writer.write_u16::<T>(self.vertex_count)?;
    writer.write_u32::<T>(self.edges_count)?;
    writer.write_u32::<T>(self.point_count)?;
    writer.write_u128::<T>(self.guid.as_u128())?;
    writer.write_u8(self.level_count)?;

    Ok(())
  }

  /// Import graph header from ini file.
  pub fn import(config: &Ini) -> io::Result<GraphHeader> {
    let props: &Properties = config
      .section(Some("header"))
      .unwrap_or_else(|| panic!("Graph section 'header' should be defined in ltx file"));

    Ok(GraphHeader {
      version: read_ini_field("version", props)?,
      vertex_count: read_ini_field("vertex_count", props)?,
      edges_count: read_ini_field("edges_count", props)?,
      point_count: read_ini_field("point_count", props)?,
      level_count: read_ini_field("level_count", props)?,
      guid: read_ini_field("guid", props)?,
    })
  }

  /// Export graph header data into level ini.
  pub fn export(&self, section: &str, ini: &mut Ini) {
    ini
      .with_section(Some(section))
      .set("version", self.version.to_string())
      .set("vertex_count", self.vertex_count.to_string())
      .set("edges_count", self.edges_count.to_string())
      .set("point_count", self.point_count.to_string())
      .set("level_count", self.level_count.to_string())
      .set("guid", self.guid.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::graph::graph_header::GraphHeader;
  use crate::test::utils::{
    get_test_sample_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;
  use uuid::uuid;

  #[test]
  fn test_read_write_simple_graph_level_point() -> io::Result<()> {
    let filename: String = String::from("graph_header.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let header: GraphHeader = GraphHeader {
      version: 16,
      vertex_count: 4000,
      edges_count: 230_250,
      point_count: 600_500,
      guid: uuid!("78e55023-10b1-426f-9247-bb680e5fe0b7"),
      level_count: 5,
    };

    header.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 28);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&get_test_sample_file_sub_dir(file!(), &filename))?,
      0,
    )?;

    assert_eq!(bytes_written, 28);

    let file: FileSlice =
      open_test_resource_as_slice(&get_test_sample_file_sub_dir(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 28 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_header: GraphHeader = GraphHeader::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_header, header);

    Ok(())
  }
}
