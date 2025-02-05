use crate::data::generic::vector_3d::Vector3d;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::Write;
use xray_chunk::{
  assert_chunk_read, ChunkIterator, ChunkReadWrite, ChunkReadWriteList, ChunkReader, ChunkWriter,
};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};
use xray_utils::assert_equal;

/// `CPatrolPoint::load_raw`, `CPatrolPoint::load` in xray codebase.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatrolPoint {
  pub name: String,
  pub position: Vector3d<f32>,
  pub flags: u32,
  pub level_vertex_id: u32,
  pub game_vertex_id: u16,
}

impl PatrolPoint {
  pub const INDEX_CHUNK_ID: u32 = 0;
  pub const DATA_CHUNK_ID: u32 = 1;
}

impl ChunkReadWriteList for PatrolPoint {
  /// Read points from the chunk reader.
  fn read_list<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Vec<Self>> {
    let mut points: Vec<Self> = Vec::new();

    for (index, mut point_reader) in ChunkIterator::from_start(reader).enumerate() {
      let mut index_reader: ChunkReader = point_reader.read_child_by_index(Self::INDEX_CHUNK_ID)?;
      let mut data_reader: ChunkReader = point_reader.read_child_by_index(Self::DATA_CHUNK_ID)?;

      assert_equal(
        index,
        index_reader.read_u32::<T>()? as usize,
        "Expect correct patrol point index",
      )?;

      points.push(Self::read::<T>(&mut data_reader)?);

      assert_chunk_read(&index_reader, "Patrol point index chunk should be read")?;
      assert_chunk_read(&point_reader, "Patrol point data chunk should be read")?;
    }

    assert_chunk_read(reader, "Patrol points chunk should be read")?;

    Ok(points)
  }

  /// Write list of patrol points into chunk writer.
  fn write_list<T: ByteOrder>(writer: &mut ChunkWriter, list: &[Self]) -> XRayResult {
    for (index, point) in list.iter().enumerate() {
      let mut point_chunk_writer: ChunkWriter = ChunkWriter::new();

      let mut point_index_writer: ChunkWriter = ChunkWriter::new();
      let mut point_writer: ChunkWriter = ChunkWriter::new();

      point_index_writer.write_u32::<T>(index as u32)?;
      point.write::<T>(&mut point_writer)?;

      point_chunk_writer
        .write_all(&point_index_writer.flush_chunk_into_buffer::<T>(Self::INDEX_CHUNK_ID)?)?;
      point_chunk_writer
        .write_all(&point_writer.flush_chunk_into_buffer::<T>(Self::DATA_CHUNK_ID)?)?;

      writer.write_all(&point_chunk_writer.flush_chunk_into_buffer::<T>(index as u32)?)?;
    }

    Ok(())
  }
}

impl ChunkReadWrite for PatrolPoint {
  /// Read patrol point data from the chunk reader.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let point: Self = Self {
      name: reader.read_w1251_string()?,
      position: reader.read_xr::<T, _>()?,
      flags: reader.read_u32::<T>()?,
      level_vertex_id: reader.read_u32::<T>()?,
      game_vertex_id: reader.read_u16::<T>()?,
    };

    assert_chunk_read(reader, "Chunk data should be read for patrol point")?;

    Ok(point)
  }

  /// Write patrol point data into chunk writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_w1251_string(&self.name)?;
    writer.write_xr::<T, _>(&self.position)?;
    writer.write_u32::<T>(self.flags)?;
    writer.write_u32::<T>(self.level_vertex_id)?;
    writer.write_u16::<T>(self.game_vertex_id)?;

    Ok(())
  }
}

impl LtxImportExport for PatrolPoint {
  /// Import patrol point data from ltx config.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "Patrol point section '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      name: read_ltx_field("name", section)?,
      position: read_ltx_field("position", section)?,
      flags: read_ltx_field("flags", section)?,
      level_vertex_id: read_ltx_field("level_vertex_id", section)?,
      game_vertex_id: read_ltx_field("game_vertex_id", section)?,
    })
  }

  /// Export patrol point data into ltx.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set("name", &self.name)
      .set("flags", self.flags.to_string())
      .set("position", self.position.to_string())
      .set("level_vertex_id", self.level_vertex_id.to_string())
      .set("game_vertex_id", self.game_vertex_id.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::generic::vector_3d::Vector3d;
  use crate::data::patrols::patrol_point::PatrolPoint;
  use crate::export::LtxImportExport;
  use serde_json::to_string_pretty;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use std::path::Path;
  use xray_chunk::{ChunkReadWrite, ChunkReadWriteList, ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_ltx::Ltx;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_sample_file_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_file, overwrite_test_relative_resource_as_file,
  };
  use xray_test_utils::FileSlice;

  #[test]
  fn test_read_write_list() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write_list.chunk");

    let original: Vec<PatrolPoint> = vec![
      PatrolPoint {
        name: String::from("wp00|a=probe_stand"),
        position: Vector3d::new(-39.898224, 24.269588, 324.09656),
        flags: 1,
        level_vertex_id: 866599,
        game_vertex_id: 60,
      },
      PatrolPoint {
        name: String::from("wp01|a=probe_stand"),
        position: Vector3d::new(-32.18162, 24.257412, 315.80127),
        flags: 2,
        level_vertex_id: 882385,
        game_vertex_id: 60,
      },
      PatrolPoint {
        name: String::from("wp01|a=probe_way"),
        position: Vector3d::new(-22.432571, 24.097664, 335.9503),
        flags: 8,
        level_vertex_id: 901200,
        game_vertex_id: 237,
      },
      PatrolPoint {
        name: String::from("wp02|a=probe_stand"),
        position: Vector3d::new(-36.36119, 24.754417, 344.12573),
        flags: 4,
        level_vertex_id: 873987,
        game_vertex_id: 237,
      },
    ];

    PatrolPoint::write_list::<XRayByteOrder>(&mut writer, &original)?;

    assert_eq!(writer.bytes_written(), 274);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 274);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 274 + 8);
    assert_eq!(
      PatrolPoint::read_list::<XRayByteOrder>(
        &mut ChunkReader::from_slice(file)?.read_child_by_index(0)?
      )?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_read_write() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: PatrolPoint = PatrolPoint {
      name: String::from("wp01|a=probe_way"),
      position: Vector3d::new(-22.432571, 24.097664, 335.9503),
      flags: 8,
      level_vertex_id: 901200,
      game_vertex_id: 237,
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 39);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 39);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 39 + 8);
    assert_eq!(
      PatrolPoint::read::<XRayByteOrder>(
        &mut ChunkReader::from_slice(file)?.read_child_by_index(0)?
      )?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "import_export.ltx");
    let mut file: File = overwrite_file(config_path)?;
    let mut ltx: Ltx = Ltx::new();

    let original: PatrolPoint = PatrolPoint {
      name: String::from("patrol-point-exported"),
      position: Vector3d::new(3.5, -2.3, 6.0),
      flags: 73,
      level_vertex_id: 26543,
      game_vertex_id: 364,
    };

    original.export("data", &mut ltx)?;
    ltx.write_to(&mut file)?;

    assert_eq!(
      PatrolPoint::import("data", &Ltx::read_from_path(config_path)?)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> XRayResult {
    let original: PatrolPoint = PatrolPoint {
      name: String::from("patrol-point-serialized"),
      position: Vector3d::new(5.5, -2.3, 6.0),
      flags: 53,
      level_vertex_id: 2351,
      game_vertex_id: 321,
    };

    let mut file: File = overwrite_file(get_absolute_test_sample_file_path(
      file!(),
      "serialize_deserialize.json",
    ))?;

    file.write_all(to_string_pretty(&original)?.as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);
    assert_eq!(original, serde_json::from_str::<PatrolPoint>(&serialized)?);

    Ok(())
  }
}
