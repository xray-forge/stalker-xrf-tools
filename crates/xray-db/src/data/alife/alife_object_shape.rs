use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::shape::Shape;
use crate::types::SpawnByteOrder;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io;
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectShape {
  pub base: AlifeObjectAbstract,
  pub shape: Vec<Shape>,
}

impl AlifeObjectInheritedReader<AlifeObjectShape> for AlifeObjectShape {
  /// Read shape object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectShape> {
    Ok(AlifeObjectShape {
      base: AlifeObjectAbstract::read::<T>(reader)?,
      shape: reader.read_shapes::<SpawnByteOrder>()?,
    })
  }

  /// Import alife shape object data from ini config.
  fn import(section: &Section) -> io::Result<AlifeObjectShape> {
    Ok(AlifeObjectShape {
      base: AlifeObjectAbstract::import(section)?,
      shape: Shape::import_list(section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectShape {
  /// Write shape object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_shapes_list::<SpawnByteOrder>(&self.shape)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) {
    self.base.export(section, ini);

    Shape::export_list(&self.shape, section, ini);
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_shape::AlifeObjectShape;
  use crate::data::shape::Shape;
  use crate::data::vector_3d::Vector3d;
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
    let filename: String = get_relative_test_sample_file_path(file!(), "alife_object_shape.chunk");

    let object: AlifeObjectShape = AlifeObjectShape {
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

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 105);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 105);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 105 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectShape = AlifeObjectShape::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
