use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_shape::AlifeObjectShape;
use crate::export::file_import::read_ini_field;
use byteorder::ByteOrder;
use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeObjectClimable {
  pub base: AlifeObjectShape,
  pub game_material: String,
}

impl AlifeObjectInheritedReader<AlifeObjectClimable> for AlifeObjectClimable {
  /// Read climable object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectClimable> {
    Ok(AlifeObjectClimable {
      base: AlifeObjectShape::read::<T>(reader)?,
      game_material: reader.read_null_terminated_win_string()?,
    })
  }

  /// Import climable object data from ini config section.
  fn import(props: &Properties) -> io::Result<AlifeObjectClimable> {
    Ok(AlifeObjectClimable {
      base: AlifeObjectShape::import(props)?,
      game_material: read_ini_field("game_material", props)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectClimable {
  /// Write climable object data into the chunk.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_null_terminated_win_string(&self.game_material)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ini) {
    self.base.export(section, ini);

    ini
      .with_section(Some(section))
      .set("game_material", &self.game_material);
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_climable::AlifeObjectClimable;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_shape::AlifeObjectShape;
  use crate::data::shape::Shape;
  use crate::data::vector_3d::Vector3d;
  use crate::test::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_relative_test_sample_file_path(file!(), "alife_object_climable.chunk");

    let object: AlifeObjectClimable = AlifeObjectClimable {
      base: AlifeObjectShape {
        base: AlifeObjectAbstract {
          game_vertex_id: 4223,
          distance: 723.23,
          direct_control: 0,
          level_vertex_id: 0,
          flags: 0,
          custom_data: String::from("custom-data"),
          story_id: 0,
          spawn_story_id: 0,
        },
        shape: vec![
          Shape::Sphere((Vector3d::new(54.5, 0.5, 11.5), 1.0)),
          Shape::Box((
            Vector3d::new(51.5, 2.5, 73.1),
            Vector3d::new(55.1, 3.2, 2.3),
            Vector3d::new(51.0, 3.0, 6.4),
            Vector3d::new(59.2, 3.3, 3.0),
          )),
        ],
      },
      game_material: String::from("dest-material"),
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 119);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 119);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 119 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectClimable =
      AlifeObjectClimable::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
