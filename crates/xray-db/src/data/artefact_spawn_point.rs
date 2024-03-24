use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::vector_3d::Vector3d;
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtefactSpawnPoint {
  pub position: Vector3d,
  pub level_vertex_id: u32,
  pub distance: f32,
}

impl ArtefactSpawnPoint {
  /// Read artefact spawn point from the chunk.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<ArtefactSpawnPoint> {
    Ok(ArtefactSpawnPoint {
      position: reader.read_f32_3d_vector::<T>()?,
      level_vertex_id: reader.read_u32::<T>()?,
      distance: reader.read_f32::<T>()?,
    })
  }

  /// Write artefact spawn point data into the writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_f32_3d_vector::<T>(&self.position)?;
    writer.write_u32::<T>(self.level_vertex_id)?;
    writer.write_f32::<T>(self.distance)?;

    Ok(())
  }

  /// Import artefact spawn point data from ini section.
  pub fn import(section: &Section) -> io::Result<ArtefactSpawnPoint> {
    Ok(ArtefactSpawnPoint {
      position: read_ini_field("position", section)?,
      level_vertex_id: read_ini_field("level_vertex_id", section)?,
      distance: read_ini_field("distance", section)?,
    })
  }

  /// Export artefact spawn point data into ini.
  pub fn export(&self, section: &str, ini: &mut Ltx) {
    ini
      .with_section(section)
      .set("distance", self.distance.to_string())
      .set("position", self.position.to_string())
      .set("level_vertex_id", self.level_vertex_id.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::artefact_spawn_point::ArtefactSpawnPoint;
  use crate::data::vector_3d::Vector3d;
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
  fn test_read_write_simple_artefact_spawn_point() -> io::Result<()> {
    let point: ArtefactSpawnPoint = ArtefactSpawnPoint {
      position: Vector3d::new(10.5, 20.3, -40.5),
      level_vertex_id: 1000,
      distance: 500.55,
    };

    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_relative_test_sample_file_path(file!(), "artefact_spawn_point_simple.chunk");

    point.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 20);

    let bytes_written: usize = writer
      .flush_chunk_into_file::<SpawnByteOrder>(
        &mut overwrite_test_relative_resource_as_file(&filename)?,
        0,
      )
      .unwrap();

    assert_eq!(bytes_written, 20);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 20 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_point: ArtefactSpawnPoint = ArtefactSpawnPoint::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_point, point);

    Ok(())
  }

  #[test]
  fn test_import_export_object() -> io::Result<()> {
    let point: ArtefactSpawnPoint = ArtefactSpawnPoint {
      position: Vector3d::new(11.5, 12.3, -10.5),
      level_vertex_id: 1001,
      distance: 6213.123,
    };

    let config_path: &Path =
      &get_absolute_test_sample_file_path(file!(), "artefact_spawn_point.ini");
    let mut file: File = overwrite_file(&config_path)?;
    let mut ltx: Ltx = Ltx::new();

    point.export("artefact_spawn_point", &mut ltx);
    ltx.write_to(&mut file)?;

    let read_point: ArtefactSpawnPoint = ArtefactSpawnPoint::import(
      open_ini_config(config_path)?
        .section("artefact_spawn_point")
        .expect("0 point section"),
    )?;

    assert_eq!(read_point, point);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize_object() -> io::Result<()> {
    let point: ArtefactSpawnPoint = ArtefactSpawnPoint {
      position: Vector3d::new(21.5, 22.3, -20.5),
      level_vertex_id: 1001,
      distance: 3452.123,
    };

    let mut file: File = overwrite_file(&get_absolute_test_sample_file_path(
      file!(),
      "serialized.json",
    ))?;

    file.write_all(json!(point).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      point,
      serde_json::from_str::<ArtefactSpawnPoint>(&serialized)?
    );

    Ok(())
  }
}
