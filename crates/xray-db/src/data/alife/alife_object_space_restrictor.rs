use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::shape::Shape;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::Ini;
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectSpaceRestrictor {
  pub base: AlifeObjectAbstract,
  pub shape: Vec<Shape>,
  pub restrictor_type: u8,
}

impl AlifeObjectInheritedReader<AlifeObjectSpaceRestrictor> for AlifeObjectSpaceRestrictor {
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectSpaceRestrictor> {
    let base: AlifeObjectAbstract = AlifeObjectAbstract::read_from_chunk::<T>(chunk)?;

    let shape: Vec<Shape> = chunk.read_shape_description::<SpawnByteOrder>()?;
    let restrictor_type: u8 = chunk.read_u8()?;

    Ok(AlifeObjectSpaceRestrictor {
      base,
      shape,
      restrictor_type,
    })
  }
}

impl AlifeObjectGeneric for AlifeObjectSpaceRestrictor {
  type Order = SpawnByteOrder;

  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_shape_description::<Self::Order>(&self.shape)?;
    writer.write_u8(self.restrictor_type)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &String, ini: &mut Ini) {
    self.base.export(section, ini);

    ini
      .with_section(Some(section))
      .set("restrictor_type", self.restrictor_type.to_string());

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
  use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
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
    let filename: String =
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_object_abstract.chunk"));

    let object: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor {
      base: AlifeObjectAbstract {
        game_vertex_id: 1001,
        distance: 65.25,
        direct_control: 412421,
        level_vertex_id: 66231,
        flags: 33,
        custom_data: String::from("custom_data"),
        story_id: 400,
        spawn_story_id: 25,
      },
      shape: vec![
        Shape::Sphere((Vector3d::new(0.5, 0.5, 0.5), 1.0)),
        Shape::Box((
          Vector3d::new(1.1, 1.1, 3.1),
          Vector3d::new(1.1, 2.2, 3.3),
          Vector3d::new(4.0, 5.0, 6.4),
          Vector3d::new(9.2, 8.3, 7.0),
        )),
      ],
      restrictor_type: 2,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 106);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 106);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 106 + 8);

    let mut chunk: Chunk = Chunk::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectSpaceRestrictor =
      AlifeObjectSpaceRestrictor::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
