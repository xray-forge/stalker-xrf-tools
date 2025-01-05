use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::data::shape::Shape;
use crate::error::database_parse_error::DatabaseParseError;
use crate::types::{DatabaseResult, SpawnByteOrder};
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectShape {
  pub base: AlifeObjectAbstract,
  pub shape: Vec<Shape>,
}

impl AlifeObjectReader<AlifeObjectShape> for AlifeObjectShape {
  /// Read shape object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      base: AlifeObjectAbstract::read::<T>(reader)?,
      shape: reader.read_shapes::<SpawnByteOrder>()?,
    })
  }

  /// Import alife shape object data from ini config.
  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "ALife object '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;
    Ok(Self {
      base: AlifeObjectAbstract::import(section_name, ini)?,
      shape: Shape::import_list(section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeObjectShape {
  /// Write shape object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    self.base.write(writer)?;

    writer.write_shapes_list::<SpawnByteOrder>(&self.shape)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult {
    self.base.export(section, ini)?;

    Shape::export_list(&self.shape, section, ini);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_shape::AlifeObjectShape;
  use crate::data::meta::alife_object_generic::AlifeObjectWriter;
  use crate::data::meta::alife_object_reader::AlifeObjectReader;
  use crate::data::shape::Shape;
  use crate::data::vector_3d::Vector3d;
  use crate::types::{DatabaseResult, SpawnByteOrder};
  use fileslice::FileSlice;
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

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 105);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 105 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectShape::read::<SpawnByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
