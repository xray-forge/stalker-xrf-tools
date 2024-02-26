use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectHangingLamp {
  pub base: AlifeObjectVisual,
  pub skeleton: AlifeObjectSkeleton,
  pub main_color: u32,
  pub main_brightness: f32,
  pub color_animator: String,
  pub main_range: f32,
  pub light_flags: u16,
  pub startup_animation: String,
  pub fixed_bones: String,
  pub health: f32,
  pub virtual_size: f32,
  pub ambient_radius: f32,
  pub ambient_power: f32,
  pub ambient_texture: String,
  pub light_texture: String,
  pub light_bone: String,
  pub spot_cone_angle: f32,
  pub glow_texture: String,
  pub glow_radius: f32,
  pub light_ambient_bone: String,
  pub volumetric_quality: f32,
  pub volumetric_intensity: f32,
  pub volumetric_distance: f32,
}

impl AlifeObjectInheritedReader<AlifeObjectHangingLamp> for AlifeObjectHangingLamp {
  /// Read hanging lamp data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectHangingLamp> {
    let base: AlifeObjectVisual = AlifeObjectVisual::read_from_chunk::<T>(chunk)?;
    let skeleton: AlifeObjectSkeleton = AlifeObjectSkeleton::read_from_chunk::<T>(chunk)?;

    let main_color: u32 = chunk.read_u32::<T>()?;
    let main_brightness: f32 = chunk.read_f32::<T>()?;
    let color_animator: String = chunk.read_null_terminated_string()?;
    let main_range: f32 = chunk.read_f32::<T>()?;
    let light_flags: u16 = chunk.read_u16::<T>()?;
    let startup_animation: String = chunk.read_null_terminated_string()?;
    let fixed_bones: String = chunk.read_null_terminated_string()?;
    let health: f32 = chunk.read_f32::<T>()?;

    let virtual_size: f32 = chunk.read_f32::<T>()?;
    let ambient_radius: f32 = chunk.read_f32::<T>()?;
    let ambient_power: f32 = chunk.read_f32::<T>()?;
    let ambient_texture: String = chunk.read_null_terminated_string()?;
    let light_texture: String = chunk.read_null_terminated_string()?;
    let light_bone: String = chunk.read_null_terminated_string()?;
    let spot_cone_angle: f32 = chunk.read_f32::<T>()?;
    let glow_texture: String = chunk.read_null_terminated_string()?;
    let glow_radius: f32 = chunk.read_f32::<T>()?;

    let light_ambient_bone: String = chunk.read_null_terminated_string()?;
    let volumetric_quality: f32 = chunk.read_f32::<T>()?;
    let volumetric_intensity: f32 = chunk.read_f32::<T>()?;
    let volumetric_distance: f32 = chunk.read_f32::<T>()?;

    Ok(AlifeObjectHangingLamp {
      base,
      skeleton,
      main_color,
      main_brightness,
      color_animator,
      main_range,
      light_flags,
      startup_animation,
      fixed_bones,
      health,
      virtual_size,
      ambient_radius,
      ambient_power,
      ambient_texture,
      light_texture,
      light_bone,
      spot_cone_angle,
      glow_texture,
      glow_radius,
      light_ambient_bone,
      volumetric_quality,
      volumetric_intensity,
      volumetric_distance,
    })
  }
}

impl AlifeObjectGeneric for AlifeObjectHangingLamp {
  type Order = SpawnByteOrder;

  /// Write skeleton data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;
    self.skeleton.write(writer)?;

    writer.write_u32::<Self::Order>(self.main_color)?;
    writer.write_f32::<Self::Order>(self.main_brightness)?;
    writer.write_null_terminated_string(&self.color_animator)?;
    writer.write_f32::<Self::Order>(self.main_range)?;
    writer.write_u16::<Self::Order>(self.light_flags)?;
    writer.write_null_terminated_string(&self.startup_animation)?;
    writer.write_null_terminated_string(&self.fixed_bones)?;
    writer.write_f32::<Self::Order>(self.health)?;

    writer.write_f32::<Self::Order>(self.virtual_size)?;
    writer.write_f32::<Self::Order>(self.ambient_radius)?;
    writer.write_f32::<Self::Order>(self.ambient_power)?;
    writer.write_null_terminated_string(&self.ambient_texture)?;
    writer.write_null_terminated_string(&self.light_texture)?;
    writer.write_null_terminated_string(&self.light_bone)?;
    writer.write_f32::<Self::Order>(self.spot_cone_angle)?;
    writer.write_null_terminated_string(&self.glow_texture)?;
    writer.write_f32::<Self::Order>(self.glow_radius)?;

    writer.write_null_terminated_string(&self.light_ambient_bone)?;
    writer.write_f32::<Self::Order>(self.volumetric_quality)?;
    writer.write_f32::<Self::Order>(self.volumetric_intensity)?;
    writer.write_f32::<Self::Order>(self.volumetric_distance)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_hanging_lamp::AlifeObjectHangingLamp;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
  use crate::data::alife::alife_object_visual::AlifeObjectVisual;
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
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_object_hanging_lamp.chunk"));

    let object: AlifeObjectHangingLamp = AlifeObjectHangingLamp {
      base: AlifeObjectVisual {
        base: AlifeObjectAbstract {
          game_vertex_id: 15,
          distance: 7634.124,
          direct_control: 253,
          level_vertex_id: 3456,
          flags: 34,
          custom_data: String::from("custom-data"),
          story_id: 6987,
          spawn_story_id: 3986,
        },
        visual_name: String::from("visual-name"),
        visual_flags: 168,
      },
      skeleton: AlifeObjectSkeleton {
        name: String::from("skeleton-name"),
        flags: 0,
        source_id: 978,
      },
      main_color: 52323,
      main_brightness: 1.0,
      color_animator: String::from("color-animator"),
      main_range: 0.5,
      light_flags: 425,
      startup_animation: String::from("setup-animation"),
      fixed_bones: String::from("fixed-bones"),
      health: 1.0,
      virtual_size: 0.7,
      ambient_radius: 24.0,
      ambient_power: 52.0,
      ambient_texture: String::from("ambient-texture"),
      light_texture: String::from("light-texture"),
      light_bone: String::from("light-bone"),
      spot_cone_angle: 5.23,
      glow_texture: String::from("glow-texture"),
      glow_radius: 15.43,
      light_ambient_bone: String::from("light-ambient-bone"),
      volumetric_quality: 1.3,
      volumetric_intensity: 2.2,
      volumetric_distance: 3.1,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 234);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 234);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 234 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectHangingLamp =
      AlifeObjectHangingLamp::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
