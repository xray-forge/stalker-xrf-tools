use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use crate::data::alife_object::AlifeObjectInherited;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

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

impl AlifeObjectInherited<AlifeObjectHangingLamp> for AlifeObjectHangingLamp {
  fn from_chunk(chunk: &mut Chunk) -> AlifeObjectHangingLamp {
    let base: AlifeObjectVisual = AlifeObjectVisual::from_chunk(chunk);
    let skeleton: AlifeObjectSkeleton = AlifeObjectSkeleton::from_chunk(chunk);

    let main_color: u32 = chunk.read_u32::<SpawnByteOrder>().unwrap();
    let main_brightness: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let color_animator: String = chunk.read_null_terminated_string().unwrap();
    let main_range: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let light_flags: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();
    let startup_animation: String = chunk.read_null_terminated_string().unwrap();
    let fixed_bones: String = chunk.read_null_terminated_string().unwrap();
    let health: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();

    let virtual_size: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let ambient_radius: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let ambient_power: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let ambient_texture: String = chunk.read_null_terminated_string().unwrap();
    let light_texture: String = chunk.read_null_terminated_string().unwrap();
    let light_bone: String = chunk.read_null_terminated_string().unwrap();
    let spot_cone_angle: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let glow_texture: String = chunk.read_null_terminated_string().unwrap();
    let glow_radius: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();

    let light_ambient_bone: String = chunk.read_null_terminated_string().unwrap();
    let volumetric_quality: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let volumetric_intensity: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();
    let volumetric_distance: f32 = chunk.read_f32::<SpawnByteOrder>().unwrap();

    AlifeObjectHangingLamp {
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
    }
  }
}
