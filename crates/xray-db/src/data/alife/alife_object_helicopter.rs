use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_motion::AlifeObjectMotion;
use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
use crate::types::SpawnByteOrder;
use byteorder::ByteOrder;
use ini::Ini;
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectHelicopter {
  pub base: AlifeObjectDynamicVisual,
  pub skeleton: AlifeObjectSkeleton,
  pub motion: AlifeObjectMotion,
  pub startup_animation: String,
  pub engine_sound: String,
}

impl AlifeObjectInheritedReader<AlifeObjectHelicopter> for AlifeObjectHelicopter {
  /// Read helicopter data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectHelicopter> {
    let base: AlifeObjectDynamicVisual = AlifeObjectDynamicVisual::read_from_chunk::<T>(chunk)?;
    let motion: AlifeObjectMotion = AlifeObjectMotion::read_from_chunk::<T>(chunk)?;
    let skeleton: AlifeObjectSkeleton = AlifeObjectSkeleton::read_from_chunk::<T>(chunk)?;

    let startup_animation: String = chunk.read_null_terminated_win_string()?;
    let engine_sound: String = chunk.read_null_terminated_win_string()?;

    Ok(AlifeObjectHelicopter {
      base,
      skeleton,
      motion,
      startup_animation,
      engine_sound,
    })
  }
}

impl AlifeObjectGeneric for AlifeObjectHelicopter {
  type Order = SpawnByteOrder;

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
  fn export(&self, section: &String, ini: &mut Ini) {
    self.base.export(section, ini);
    self.motion.export(section, ini);
    self.skeleton.export(section, ini);

    ini
      .with_section(Some(section))
      .set("max_power", &self.startup_animation)
      .set("owner_id", &self.engine_sound);
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_helicopter::AlifeObjectHelicopter;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_motion::AlifeObjectMotion;
  use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
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
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_object_helicopter.chunk"));

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
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 111);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 111 + 8);

    let mut chunk: Chunk = Chunk::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectHelicopter =
      AlifeObjectHelicopter::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
