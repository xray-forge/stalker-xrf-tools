use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
use byteorder::ByteOrder;
use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeSmartZone {
  pub base: AlifeObjectSpaceRestrictor,
}

impl AlifeObjectInheritedReader<AlifeSmartZone> for AlifeSmartZone {
  /// Read generic alife smart zone object from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeSmartZone> {
    Ok(AlifeSmartZone {
      base: AlifeObjectSpaceRestrictor::read::<T>(reader)?,
    })
  }

  /// Import generic alife smart zone object from ini config section.
  fn import(props: &Properties) -> io::Result<AlifeSmartZone> {
    Ok(AlifeSmartZone {
      base: AlifeObjectSpaceRestrictor::import(props)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeSmartZone {
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ini) {
    self.base.export(section, ini);
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::alife::alife_smart_zone::AlifeSmartZone;
  use crate::data::shape::Shape;
  use crate::data::vector_3d::Vector3d;
  use crate::test::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "alife_smart_zone.chunk");

    let object: AlifeSmartZone = AlifeSmartZone {
      base: AlifeObjectSpaceRestrictor {
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
          Shape::Sphere((Vector3d::new(3.5, 0.5, 11.5), 1.0)),
          Shape::Box((
            Vector3d::new(1.5, 1.1, 73.1),
            Vector3d::new(5.1, 2.2, 3.3),
            Vector3d::new(4.0, 3.0, 6.4),
            Vector3d::new(9.2, 8.3, 3.0),
          )),
        ],
        restrictor_type: 3,
      },
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

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeSmartZone = AlifeSmartZone::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
