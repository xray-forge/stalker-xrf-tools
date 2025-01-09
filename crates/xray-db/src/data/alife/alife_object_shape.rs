use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::generic::shape::Shape;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::error::database_parse_error::DatabaseParseError;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectShape {
  pub base: AlifeObjectAbstract,
  pub shape: Vec<Shape>,
}

impl AlifeObjectReader for AlifeObjectShape {
  /// Read shape object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      base: AlifeObjectAbstract::read::<T>(reader)?,
      shape: Shape::read_list::<T>(reader)?,
    })
  }

  /// Import alife shape object data from ltx config.
  fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "ALife object '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;
    Ok(Self {
      base: AlifeObjectAbstract::import(section_name, ltx)?,
      shape: Shape::import_list(section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeObjectShape {
  /// Write shape object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    self.base.write(writer)?;

    Shape::write_list::<XRayByteOrder>(&self.shape, writer)?;

    Ok(())
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    self.base.export(section_name, ltx)?;

    Shape::export_list(&self.shape, section_name, ltx);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_shape::AlifeObjectShape;
  use crate::data::generic::shape::Shape;
  use crate::data::generic::vector_3d::Vector3d;
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

    let original: AlifeObjectShape = AlifeObjectShape {
      base: AlifeObjectAbstract {
        game_vertex_id: 623,
        distance: 253.55,
        direct_control: 312,
        level_vertex_id: 12534,
        flags: 53,
        custom_data: String::from("custom_data"),
        story_id: 6513,
        spawn_story_id: 527841,
      },
      shape: vec![
        Shape::Sphere((Vector3d::new(5.5, 0.5, 11.5), 1.0)),
        Shape::Box((
          Vector3d::new(5.5, 12.5, 73.1),
          Vector3d::new(5.1, 13.2, 2.3),
          Vector3d::new(1.0, 12.0, 6.4),
          Vector3d::new(9.2, 13.3, 3.0),
        )),
      ],
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 105);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 105);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 105 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectShape::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
