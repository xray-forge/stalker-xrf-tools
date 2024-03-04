use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_motion::AlifeObjectMotion;
use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
use crate::export::file_import::read_ini_field;
use byteorder::ByteOrder;
use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeObjectHelicopter {
  pub base: AlifeObjectDynamicVisual,
  pub motion: AlifeObjectMotion,
  pub skeleton: AlifeObjectSkeleton,
  pub startup_animation: String,
  pub engine_sound: String,
}

impl AlifeObjectInheritedReader<AlifeObjectHelicopter> for AlifeObjectHelicopter {
  /// Read helicopter data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectHelicopter> {
    Ok(AlifeObjectHelicopter {
      base: AlifeObjectDynamicVisual::read::<T>(reader)?,
      motion: AlifeObjectMotion::read::<T>(reader)?,
      skeleton: AlifeObjectSkeleton::read::<T>(reader)?,
      startup_animation: reader.read_null_terminated_win_string()?,
      engine_sound: reader.read_null_terminated_win_string()?,
    })
  }

  /// Import helicopter object data from ini config section.
  fn import(props: &Properties) -> io::Result<AlifeObjectHelicopter> {
    Ok(AlifeObjectHelicopter {
      base: AlifeObjectDynamicVisual::import(props)?,
      skeleton: AlifeObjectSkeleton::import(props)?,
      motion: AlifeObjectMotion::import(props)?,
      startup_animation: read_ini_field("startup_animation", props)?,
      engine_sound: read_ini_field("engine_sound", props)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectHelicopter {
  /// Write helicopter data into the chunk.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;
    self.motion.write(writer)?;
    self.skeleton.write(writer)?;

    writer.write_null_terminated_win_string(&self.startup_animation)?;
    writer.write_null_terminated_win_string(&self.engine_sound)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ini) {
    self.base.export(section, ini);
    self.motion.export(section, ini);
    self.skeleton.export(section, ini);

    ini
      .with_section(Some(section))
      .set("startup_animation", &self.startup_animation)
      .set("engine_sound", &self.engine_sound);
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_helicopter::AlifeObjectHelicopter;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_motion::AlifeObjectMotion;
  use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
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
      get_relative_test_sample_file_path(file!(), "alife_object_helicopter.chunk");

    let object: AlifeObjectHelicopter = AlifeObjectHelicopter {
      base: AlifeObjectDynamicVisual {
        base: AlifeObjectAbstract {
          game_vertex_id: 6432,
          distance: 243.53,
          direct_control: 25364,
          level_vertex_id: 3541,
          flags: 43,
          custom_data: String::from("custom-data"),
          story_id: 64353,
          spawn_story_id: 2533,
        },
        visual_name: String::from("visual-name"),
        visual_flags: 43,
      },
      skeleton: AlifeObjectSkeleton {
        name: String::from("skeleton-name"),
        flags: 0,
        source_id: 235,
      },
      motion: AlifeObjectMotion {
        motion_name: String::from("motion-name"),
      },
      startup_animation: String::from("startup-animation"),
      engine_sound: String::from("engine-sound"),
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 111);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 111);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 111 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectHelicopter =
      AlifeObjectHelicopter::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
