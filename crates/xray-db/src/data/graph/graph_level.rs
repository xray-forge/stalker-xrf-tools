use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::vector_3d::Vector3d;
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use uuid::Uuid;
use xray_ltx::{Ini, Properties};

/// `GameGraph::SLevel::load` in xray codebase.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphLevel {
  #[serde(rename = "name")]
  pub name: String,
  #[serde(rename = "offset")]
  pub offset: Vector3d<f32>,
  #[serde(rename = "id")]
  pub id: u8,
  #[serde(rename = "section")]
  pub section: String,
  #[serde(rename = "guid")]
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
  use crate::export::file::{export_ini_to_file, open_ini_config};
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
  use xray_ltx::Ini;

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
      &mut overwrite_test_relative_resource_as_file(&get_relative_test_sample_file_path(
        file!(),
        &filename,
      ))?,
      0,
    )?;

    assert_eq!(bytes_written, 59);

    let file: FileSlice =
      open_test_resource_as_slice(&get_relative_test_sample_file_path(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 59 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_level: GraphLevel = GraphLevel::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_level, level);

    Ok(())
  }

  #[test]
  fn test_import_export_object() -> io::Result<()> {
    let level: GraphLevel = GraphLevel {
      id: 78,
      name: String::from("test-level-exported"),
      section: String::from("test-level-section"),
      guid: uuid!("89e55023-10b1-426f-9247-bb680e5fe0a5"),
      offset: Vector3d::new(0.25, 5.55, -1.5),
    };

    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "graph_level.ini");
    let mut file: File =
      overwrite_test_relative_resource_as_file(config_path.to_str().expect("Valid path"))?;
    let mut ini: Ini = Ini::new();

    level.export("graph_level", &mut ini);
    export_ini_to_file(&ini, &mut file)?;

    let read_level: GraphLevel = GraphLevel::import("graph_level", &open_ini_config(config_path)?)?;

    assert_eq!(read_level, level);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize_object() -> io::Result<()> {
    let level: GraphLevel = GraphLevel {
      id: 243,
      name: String::from("test-level-example"),
      section: String::from("test-level-section"),
      guid: uuid!("89e55023-10b1-426f-9247-bb680e5fe0b8"),
      offset: Vector3d::new(0.25, 5.55, -1.5),
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialized.json"),
    )?;

    file.write_all(json!(level).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(level, serde_json::from_str::<GraphLevel>(&serialized)?);

    Ok(())
  }
}
