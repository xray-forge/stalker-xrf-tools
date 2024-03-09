use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeGraphPoint {
  #[serde(rename = "connectionPointName")]
  pub connection_point_name: String,
  #[serde(rename = "connectionLevelName")]
  pub connection_level_name: String,
  // todo: Use U32Bytes?
  #[serde(rename = "location0")]
  pub location0: u8,
  #[serde(rename = "location1")]
  pub location1: u8,
  #[serde(rename = "location2")]
  pub location2: u8,
  #[serde(rename = "location3")]
  pub location3: u8,
}

impl AlifeObjectInheritedReader<AlifeGraphPoint> for AlifeGraphPoint {
  /// Read graph point data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeGraphPoint> {
    Ok(AlifeGraphPoint {
      connection_point_name: reader.read_null_terminated_win_string()?,
      connection_level_name: reader.read_null_terminated_win_string()?,
      location0: reader.read_u8()?,
      location1: reader.read_u8()?,
      location2: reader.read_u8()?,
      location3: reader.read_u8()?,
    })
  }

  /// Import graph data from ini file section.
  fn import(section: &Section) -> io::Result<AlifeGraphPoint> {
    Ok(AlifeGraphPoint {
      connection_point_name: read_ini_field("connection_point_name", section)?,
      connection_level_name: read_ini_field("connection_point_name", section)?,
      location0: read_ini_field("location0", section)?,
      location1: read_ini_field("location1", section)?,
      location2: read_ini_field("location2", section)?,
      location3: read_ini_field("location3", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeGraphPoint {
  /// Write graph point data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_null_terminated_win_string(&self.connection_point_name)?;
    writer.write_null_terminated_win_string(&self.connection_level_name)?;
    writer.write_u8(self.location0)?;
    writer.write_u8(self.location1)?;
    writer.write_u8(self.location2)?;
    writer.write_u8(self.location3)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) {
    ini
      .with_section(section)
      .set("connection_point_name", &self.connection_point_name)
      .set("connection_level_name", &self.connection_level_name)
      .set("location0", self.location0.to_string())
      .set("location1", self.location1.to_string())
      .set("location2", self.location2.to_string())
      .set("location3", self.location3.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_graph_point::AlifeGraphPoint;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "alife_graph_point.chunk");

    let object: AlifeGraphPoint = AlifeGraphPoint {
      connection_point_name: String::from("point-name"),
      connection_level_name: String::from("level-name"),
      location0: 0,
      location1: 1,
      location2: 2,
      location3: 3,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 26);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 26);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 26 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeGraphPoint = AlifeGraphPoint::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
