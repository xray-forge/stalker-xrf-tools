use crate::data::generic::vector_3d::Vector3d;
use crate::export::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};
use xray_error::XRayResult;
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtefactSpawnPoint {
  pub position: Vector3d,
  pub level_vertex_id: u32,
  pub distance: f32,
}

impl ArtefactSpawnPoint {
  /// Read artefact spawn point from the chunk reader.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      position: Vector3d::read::<T>(reader)?,
      level_vertex_id: reader.read_u32::<T>()?,
      distance: reader.read_f32::<T>()?,
    })
  }

  /// Write artefact spawn point data into the chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    self.position.write::<T>(writer)?;

    writer.write_u32::<T>(self.level_vertex_id)?;
    writer.write_f32::<T>(self.distance)?;

    Ok(())
  }

  /// Import artefact spawn point data from ltx section.
  pub fn import(section: &Section) -> XRayResult<Self> {
    Ok(Self {
      position: read_ltx_field("position", section)?,
      level_vertex_id: read_ltx_field("level_vertex_id", section)?,
      distance: read_ltx_field("distance", section)?,
    })
  }

  /// Export artefact spawn point data into ltx.
  pub fn export(&self, section_name: &str, ltx: &mut Ltx) {
    ltx
      .with_section(section_name)
      .set("distance", self.distance.to_string())
      .set("position", self.position.to_string())
      .set("level_vertex_id", self.level_vertex_id.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::data::artefact_spawn::artefact_spawn_point::ArtefactSpawnPoint;
  use crate::data::generic::vector_3d::Vector3d;
  use crate::export::file::open_ltx_config;
  use fileslice::FileSlice;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use std::path::Path;
  use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_ltx::Ltx;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_sample_file_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_file, overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> XRayResult {
    let original: ArtefactSpawnPoint = ArtefactSpawnPoint {
      position: Vector3d::new(10.5, 20.3, -40.5),
      level_vertex_id: 1000,
      distance: 500.55,
    };

    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 20);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 20);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 20 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    assert_eq!(
      ArtefactSpawnPoint::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let original: ArtefactSpawnPoint = ArtefactSpawnPoint {
      position: Vector3d::new(11.5, 12.3, -10.5),
      level_vertex_id: 1001,
      distance: 6213.123,
    };

    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "import_export.ltx");
    let mut file: File = overwrite_file(config_path)?;
    let mut ltx: Ltx = Ltx::new();

    original.export("artefact_spawn_point", &mut ltx);
    ltx.write_to(&mut file)?;

    assert_eq!(
      ArtefactSpawnPoint::import(
        open_ltx_config(config_path)?
          .section("artefact_spawn_point")
          .expect("0 point section"),
      )?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> XRayResult {
    let original: ArtefactSpawnPoint = ArtefactSpawnPoint {
      position: Vector3d::new(21.5, 22.3, -20.5),
      level_vertex_id: 1001,
      distance: 3452.123,
    };

    let mut file: File = overwrite_file(&get_absolute_test_sample_file_path(
      file!(),
      "serialize_deserialize.json",
    ))?;

    file.write_all(json!(original).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(
      original,
      serde_json::from_str::<ArtefactSpawnPoint>(&serialized).unwrap()
    );

    Ok(())
  }
}
