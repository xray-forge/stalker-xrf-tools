use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::shape::Shape;
use crate::export::file_import::read_ini_field;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectSpaceRestrictor {
  pub base: AlifeObjectAbstract,
  pub shape: Vec<Shape>,
  pub restrictor_type: u8,
}

impl AlifeObjectInheritedReader<AlifeObjectSpaceRestrictor> for AlifeObjectSpaceRestrictor {
  /// Read generic space restrictor data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectSpaceRestrictor> {
    Ok(AlifeObjectSpaceRestrictor {
      base: AlifeObjectAbstract::read_from_chunk::<T>(chunk)?,
      shape: chunk.read_shape_description::<SpawnByteOrder>()?,
      restrictor_type: chunk.read_u8()?,
    })
  }

  /// Import generic space restrictor data from the chunk.
  fn import(props: &Properties) -> io::Result<AlifeObjectSpaceRestrictor> {
    Ok(AlifeObjectSpaceRestrictor {
      base: AlifeObjectAbstract::import(props)?,
      shape: Shape::import_shapes(props)?,
      restrictor_type: read_ini_field("restrictor_type", props)?,
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
  fn export(&self, section: &str, ini: &mut Ini) {
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
  use crate::export::file::{export_ini_to_file, open_ini_config};
  use crate::test::assertions::files_are_equal_by_path;
  use crate::test::utils::{
    get_test_resource_path, get_test_sample_file_sub_dir, open_test_resource_as_slice,
    overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use ini::Ini;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_test_sample_file_sub_dir(file!(), "alife_object_abstract.chunk");

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

  #[test]
  fn test_import_export_object() -> io::Result<()> {
    let first: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor {
      base: AlifeObjectAbstract {
        game_vertex_id: 2593,
        distance: 34.7,
        direct_control: 235,
        level_vertex_id: 245423,
        flags: 32,
        custom_data: String::from("test-custom-data"),
        story_id: 253423,
        spawn_story_id: 457,
      },
      shape: vec![],
      restrictor_type: 3,
    };

    let second: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor {
      base: AlifeObjectAbstract {
        game_vertex_id: 45724,
        distance: 43.0,
        direct_control: 236623,
        level_vertex_id: 2364,
        flags: 75,
        custom_data: String::new(),
        story_id: 253,
        spawn_story_id: 7546,
      },
      shape: vec![
        Shape::Sphere((Vector3d::new(54.5, 0.5, 11.5), 1.0)),
        Shape::Box((
          Vector3d::new(3.5, 2.5, 73.1),
          Vector3d::new(55.1, 1.2, 2.3),
          Vector3d::new(51.0, 7.0, 3.4),
          Vector3d::new(59.2, 3.3, 4.1),
        )),
      ],
      restrictor_type: 4,
    };

    let exported_filename: String = get_test_sample_file_sub_dir(file!(), "exported.ini");
    let mut exported: Ini = Ini::new();

    first.export("first", &mut exported);
    second.export("second", &mut exported);

    export_ini_to_file(
      &exported,
      &mut overwrite_test_resource_as_file(&exported_filename)?,
    )?;

    let source: Ini = open_ini_config(&get_test_resource_path(&exported_filename))?;

    let read_first: AlifeObjectSpaceRestrictor =
      AlifeObjectSpaceRestrictor::import(source.section(Some("first")).unwrap())?;
    let read_second: AlifeObjectSpaceRestrictor =
      AlifeObjectSpaceRestrictor::import(source.section(Some("second")).unwrap())?;

    assert_eq!(read_first, first);
    assert_eq!(read_second, second);

    let imported_filename: String = get_test_sample_file_sub_dir(file!(), "imported.ini");
    let mut imported: Ini = Ini::new();

    read_first.export("first", &mut imported);
    read_second.export("second", &mut imported);

    export_ini_to_file(
      &imported,
      &mut overwrite_test_resource_as_file(&imported_filename)?,
    )?;

    assert!(files_are_equal_by_path(
      get_test_resource_path(&exported_filename),
      get_test_resource_path(&imported_filename)
    )?);

    Ok(())
  }
}
