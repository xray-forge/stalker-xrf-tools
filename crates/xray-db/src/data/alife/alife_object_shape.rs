use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::shape::Shape;
use crate::types::SpawnByteOrder;
use byteorder::ByteOrder;
use ini::Ini;
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectShape {
  pub base: AlifeObjectAbstract,
  pub shape: Vec<Shape>,
}

impl AlifeObjectInheritedReader<AlifeObjectShape> for AlifeObjectShape {
  /// Read shape object data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectShape> {
    let base: AlifeObjectAbstract = AlifeObjectAbstract::read_from_chunk::<T>(chunk)?;

    let shape: Vec<Shape> = chunk.read_shape_description::<SpawnByteOrder>()?;

    Ok(AlifeObjectShape { base, shape })
  }
}

impl AlifeObjectGeneric for AlifeObjectShape {
  type Order = SpawnByteOrder;

  /// Write shape object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_shape_description::<Self::Order>(&self.shape)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ini) {
    self.base.export(section, ini);

    Shape::export_shapes(&self.shape, section, ini);
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_shape::AlifeObjectShape;
  use crate::data::shape::Shape;
  use crate::data::vector_3d::Vector3d;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_test_chunk_file_sub_dir(file!(), "alife_object_shape.chunk");

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
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 105);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 105 + 8);

    let mut chunk: Chunk = Chunk::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectShape =
      AlifeObjectShape::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
