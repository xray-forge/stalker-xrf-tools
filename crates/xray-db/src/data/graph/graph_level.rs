use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::vector_3d::Vector3d;
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use std::io;
use uuid::Uuid;

/// `GameGraph::SLevel::load` in xray codebase.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphLevel {
  pub name: String,
  pub offset: Vector3d<f32>,
  pub id: u8,
  pub section: String,
  pub guid: Uuid,
}

impl GraphLevel {
  /// Read graph level data from the chunk.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<GraphLevel> {
    Ok(GraphLevel {
      name: reader.read_null_terminated_win_string()?,
      offset: reader.read_f32_3d_vector::<T>()?,
      id: reader.read_u8()?,
      section: reader.read_null_terminated_win_string()?,
      guid: Uuid::from_u128(reader.read_u128::<T>()?),
    })
  }

  /// Write graph level data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_null_terminated_win_string(&self.name)?;
    writer.write_f32_3d_vector::<T>(&self.offset)?;
    writer.write_u8(self.id)?;
    writer.write_null_terminated_win_string(&self.section)?;
    writer.write_u128::<T>(self.guid.as_u128())?;

    Ok(())
  }

  /// Import patrols data from provided path.
  pub fn import(section: &str, config: &Ini) -> io::Result<GraphLevel> {
    let props: &Properties = config
      .section(Some(section))
      .unwrap_or_else(|| panic!("Graph section {section} should be defined in ltx file"));

    Ok(GraphLevel {
      name: read_ini_field("name", props)?,
      offset: read_ini_field("offset", props)?,
      id: read_ini_field("id", props)?,
      section: read_ini_field("section", props)?,
      guid: read_ini_field("guid", props)?,
    })
  }

  /// Export graph level data into ini.
  pub fn export(&self, section: &str, config: &mut Ini) {
    config
      .with_section(Some(section))
      .set("name", &self.name)
      .set("section", &self.section)
      .set("offset", self.offset.to_string())
      .set("id", self.id.to_string())
      .set("guid", self.guid.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::graph::graph_level::GraphLevel;
  use crate::data::vector_3d::Vector3d;
  use crate::test::utils::{
    get_test_sample_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;
  use uuid::uuid;

  #[test]
  fn test_read_write_simple_graph_level_point() -> io::Result<()> {
    let filename: String = String::from("graph_level.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let level: GraphLevel = GraphLevel {
      id: 255,
      name: String::from("test-level"),
      section: String::from("test-level-section"),
      guid: uuid!("89e55023-10b1-426f-9247-bb680e5fe0b8"),
      offset: Vector3d::new(0.5, 5.55, -1.5),
    };

    level.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 59);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&get_test_sample_file_sub_dir(file!(), &filename))?,
      0,
    )?;

    assert_eq!(bytes_written, 59);

    let file: FileSlice =
      open_test_resource_as_slice(&get_test_sample_file_sub_dir(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 59 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_level: GraphLevel = GraphLevel::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_level, level);

    Ok(())
  }
}
