use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ltx_field;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeGraphPoint {
  pub connection_point_name: String,
  pub connection_level_name: String,
  // todo: Use U32Bytes?
  pub location0: u8,
  pub location1: u8,
  pub location2: u8,
  pub location3: u8,
}

impl AlifeObjectReader for AlifeGraphPoint {
  /// Read graph point data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      connection_point_name: reader.read_null_terminated_win_string()?,
      connection_level_name: reader.read_null_terminated_win_string()?,
      location0: reader.read_u8()?,
      location1: reader.read_u8()?,
      location2: reader.read_u8()?,
      location3: reader.read_u8()?,
    })
  }

  /// Import graph data from ltx file section.
  fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "ALife object '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
      connection_point_name: read_ltx_field("connection_point_name", section)?,
      connection_level_name: read_ltx_field("connection_point_name", section)?,
      location0: read_ltx_field("location0", section)?,
      location1: read_ltx_field("location1", section)?,
      location2: read_ltx_field("location2", section)?,
      location3: read_ltx_field("location3", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeGraphPoint {
  /// Write graph point data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_null_terminated_win_string(&self.connection_point_name)?;
    writer.write_null_terminated_win_string(&self.connection_level_name)?;
    writer.write_u8(self.location0)?;
    writer.write_u8(self.location1)?;
    writer.write_u8(self.location2)?;
    writer.write_u8(self.location3)?;

    Ok(())
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    ltx
      .with_section(section_name)
      .set("connection_point_name", &self.connection_point_name)
      .set("connection_level_name", &self.connection_level_name)
      .set("location0", self.location0.to_string())
      .set("location1", self.location1.to_string())
      .set("location2", self.location2.to_string())
      .set("location3", self.location3.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::alife_graph_point::AlifeGraphPoint;
  use crate::data::meta::alife_object_generic::AlifeObjectWriter;
  use crate::data::meta::alife_object_reader::AlifeObjectReader;
  use crate::types::DatabaseResult;
  use fileslice::FileSlice;
  use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult {
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

    original.write(&mut writer)?;

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
}
