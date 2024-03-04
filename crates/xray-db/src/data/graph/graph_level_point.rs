use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::vector_3d::Vector3d;
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphLevelPoint {
  #[serde(rename = "position")]
  pub position: Vector3d,
  #[serde(rename = "levelVertexId")]
  pub level_vertex_id: u32,
  #[serde(rename = "distance")]
  pub distance: f32,
}

impl GraphLevelPoint {
  /// Read level point from chunk.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<GraphLevelPoint> {
    Ok(GraphLevelPoint {
      position: reader.read_f32_3d_vector::<T>()?,
      level_vertex_id: reader.read_u32::<T>()?,
      distance: reader.read_f32::<T>()?,
    })
  }

  /// Write level point data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_f32_3d_vector::<T>(&self.position)?;
    writer.write_u32::<T>(self.level_vertex_id)?;
    writer.write_f32::<T>(self.distance)?;

    Ok(())
  }

  /// Import graph level point from ini file.
  pub fn import(section: &str, config: &Ini) -> io::Result<GraphLevelPoint> {
    let props: &Properties = config.section(Some(section)).unwrap_or_else(|| {
      panic!("Graph section '{section}' should be defined in level point ltx file")
    });

    Ok(GraphLevelPoint {
      position: read_ini_field("position", props)?,
      level_vertex_id: read_ini_field("level_vertex_id", props)?,
      distance: read_ini_field("distance", props)?,
    })
  }

  /// Export graph level point data into ini.
  pub fn export(&self, section: &str, ini: &mut Ini) {
    ini
      .with_section(Some(section))
      .set("position", self.position.to_string())
      .set("level_vertex_id", self.level_vertex_id.to_string())
      .set("distance", self.distance.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::graph::graph_level_point::GraphLevelPoint;
  use crate::data::vector_3d::Vector3d;
  use crate::export::file::{export_ini_to_file, open_ini_config};
  use crate::test::file::read_file_as_string;
  use crate::test::utils::{
    get_absolute_test_sample_file_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use ini::Ini;
  use serde_json::json;
  use std::fs::File;
  use std::io;
  use std::io::{Seek, SeekFrom, Write};
  use std::path::Path;

  #[test]
  fn test_read_write_simple_graph_level_point() -> io::Result<()> {
    let filename: String = String::from("graph_level_point_simple.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let point: GraphLevelPoint = GraphLevelPoint {
      position: Vector3d::new(10.5, 11.6, 12.7),
      distance: 400.50,
      level_vertex_id: 8000,
    };

    point.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 20);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_file(&get_absolute_test_sample_file_path(file!(), &filename))?,
      0,
    )?;

    assert_eq!(bytes_written, 20);

    let file: FileSlice =
      open_test_resource_as_slice(&get_relative_test_sample_file_path(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 20 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_point: GraphLevelPoint = GraphLevelPoint::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_point, point);

    Ok(())
  }

  #[test]
  fn test_import_export_object() -> io::Result<()> {
    let point: GraphLevelPoint = GraphLevelPoint {
      position: Vector3d::new(66.5, 55.6, 88.7),
      distance: 4235.50,
      level_vertex_id: 236263,
    };

    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "graph_level_point.ini");
    let mut file: File = overwrite_file(&config_path)?;
    let mut ini: Ini = Ini::new();

    point.export("graph_level_point", &mut ini);
    export_ini_to_file(&ini, &mut file)?;

    let read_point: GraphLevelPoint =
      GraphLevelPoint::import("graph_level_point", &open_ini_config(config_path)?)?;

    assert_eq!(read_point, point);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize_object() -> io::Result<()> {
    let point: GraphLevelPoint = GraphLevelPoint {
      position: Vector3d::new(11.5, 11.6, 2.7),
      distance: 321.50,
      level_vertex_id: 5213,
    };

    let mut file: File = overwrite_file(&get_absolute_test_sample_file_path(
      file!(),
      "serialized.json",
    ))?;

    file.write_all(json!(point).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(point, serde_json::from_str::<GraphLevelPoint>(&serialized)?);

    Ok(())
  }
}
