use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
use crate::data::alife::alife_object_visual::AlifeObjectVisual;
use byteorder::{ByteOrder, ReadBytesExt};
use std::io;

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

  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    todo!("Implement write operation");
  }
}

impl AlifeObjectGeneric for AlifeObjectHangingLamp {}
