use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeGraphPoint {
  pub connection_point_name: String,
  pub connection_level_name: String,
  // todo: Use U32Bytes?
  // todo: Use U32Bytes?
  // todo: Use U32Bytes?
  pub location0: u8,
  pub location1: u8,
  pub location2: u8,
  pub location3: u8,
}

impl ChunkReadWrite for AlifeGraphPoint {
  /// Read graph point data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      connection_point_name: reader.read_w1251_string()?,
      connection_level_name: reader.read_w1251_string()?,
      location0: reader.read_u8()?,
      location1: reader.read_u8()?,
      location2: reader.read_u8()?,
      location3: reader.read_u8()?,
    })
  }

  /// Write graph point data into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_w1251_string(&self.connection_point_name)?;
    writer.write_w1251_string(&self.connection_level_name)?;
    writer.write_u8(self.location0)?;
    writer.write_u8(self.location1)?;
    writer.write_u8(self.location2)?;
    writer.write_u8(self.location3)?;

    Ok(())
  }
}

impl LtxImportExport for AlifeGraphPoint {
  /// Import graph data from ltx file section.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "ALife object '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      connection_point_name: read_ltx_field("graph_point.connection_point_name", section)?,
      connection_level_name: read_ltx_field("graph_point.connection_level_name", section)?,
      location0: read_ltx_field("graph_point.location0", section)?,
      location1: read_ltx_field("graph_point.location1", section)?,
      location2: read_ltx_field("graph_point.location2", section)?,
      location3: read_ltx_field("graph_point.location3", section)?,
    })
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    ltx
      .with_section(section_name)
      .set(
        "graph_point.connection_point_name",
        &self.connection_point_name,
      )
      .set(
        "graph_point.connection_level_name",
        &self.connection_level_name,
      )
      .set("graph_point.location0", self.location0.to_string())
      .set("graph_point.location1", self.location1.to_string())
      .set("graph_point.location2", self.location2.to_string())
      .set("graph_point.location3", self.location3.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::inherited::alife_graph_point::AlifeGraphPoint;
  use crate::export::LtxImportExport;
  use serde_json::json;
  use std::fs::File;
  use std::io::{Seek, SeekFrom, Write};
  use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_ltx::Ltx;
  use xray_test_utils::file::read_file_as_string;
  use xray_test_utils::utils::{
    get_absolute_test_resource_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_test_relative_resource_as_file,
  };
  use xray_test_utils::FileSlice;

  #[test]
  fn test_read_write() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: AlifeGraphPoint = AlifeGraphPoint {
      connection_point_name: String::from("point-name"),
      connection_level_name: String::from("level-name"),
      location0: 0,
      location1: 1,
      location2: 2,
      location3: 3,
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 26);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 26);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 26 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeGraphPoint::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> XRayResult {
    let ltx_filename: String = get_relative_test_sample_file_path(file!(), "import_export.ltx");
    let mut ltx: Ltx = Ltx::new();

    let original: AlifeGraphPoint = AlifeGraphPoint {
      connection_point_name: String::from("point-name"),
      connection_level_name: String::from("level-name"),
      location0: 0,
      location1: 10,
      location2: 20,
      location3: 30,
    };

    original.export("data", &mut ltx)?;

    ltx.write_to(&mut overwrite_test_relative_resource_as_file(
      &ltx_filename,
    )?)?;

    let source: Ltx = Ltx::read_from_path(get_absolute_test_resource_path(&ltx_filename))?;

    assert_eq!(AlifeGraphPoint::import("data", &source)?, original);

    Ok(())
  }

  #[test]
  fn test_serialize_deserialize() -> XRayResult {
    let original: AlifeGraphPoint = AlifeGraphPoint {
      connection_point_name: String::from("point-name"),
      connection_level_name: String::from("level-name"),
      location0: 0,
      location1: 100,
      location2: 200,
      location3: 255,
    };

    let mut file: File = overwrite_test_relative_resource_as_file(
      &get_relative_test_sample_file_path(file!(), "serialize_deserialize.json"),
    )?;

    file.write_all(json!(original).to_string().as_bytes())?;
    file.seek(SeekFrom::Start(0))?;

    let serialized: String = read_file_as_string(&mut file)?;

    assert_eq!(serialized.to_string(), serialized);

    assert_eq!(
      serde_json::from_str::<AlifeGraphPoint>(&serialized).unwrap(),
      original
    );

    Ok(())
  }
}
