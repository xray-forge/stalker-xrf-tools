use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, SpawnByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectHangingLamp {
  pub base: AlifeObjectDynamicVisual,
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

impl AlifeObjectReader<AlifeObjectHangingLamp> for AlifeObjectHangingLamp {
  /// Read hanging lamp data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      base: AlifeObjectDynamicVisual::read::<T>(reader)?,
      skeleton: AlifeObjectSkeleton::read::<T>(reader)?,
      main_color: reader.read_u32::<T>()?,
      main_brightness: reader.read_f32::<T>()?,
      color_animator: reader.read_null_terminated_win_string()?,
      main_range: reader.read_f32::<T>()?,
      light_flags: reader.read_u16::<T>()?,
      startup_animation: reader.read_null_terminated_win_string()?,
      fixed_bones: reader.read_null_terminated_win_string()?,
      health: reader.read_f32::<T>()?,
      virtual_size: reader.read_f32::<T>()?,
      ambient_radius: reader.read_f32::<T>()?,
      ambient_power: reader.read_f32::<T>()?,
      ambient_texture: reader.read_null_terminated_win_string()?,
      light_texture: reader.read_null_terminated_win_string()?,
      light_bone: reader.read_null_terminated_win_string()?,
      spot_cone_angle: reader.read_f32::<T>()?,
      glow_texture: reader.read_null_terminated_win_string()?,
      glow_radius: reader.read_f32::<T>()?,
      light_ambient_bone: reader.read_null_terminated_win_string()?,
      volumetric_quality: reader.read_f32::<T>()?,
      volumetric_intensity: reader.read_f32::<T>()?,
      volumetric_distance: reader.read_f32::<T>()?,
    })
  }

  /// Import alife hanging lamp object data from ini config section.
  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "ALife object '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectDynamicVisual::import(section_name, ini)?,
      skeleton: AlifeObjectSkeleton::import(section_name, ini)?,
      main_color: read_ini_field("main_color", section)?,
      main_brightness: read_ini_field("main_brightness", section)?,
      color_animator: read_ini_field("color_animator", section)?,
      main_range: read_ini_field("main_range", section)?,
      light_flags: read_ini_field("light_flags", section)?,
      startup_animation: read_ini_field("startup_animation", section)?,
      fixed_bones: read_ini_field("fixed_bones", section)?,
      health: read_ini_field("health", section)?,
      virtual_size: read_ini_field("virtual_size", section)?,
      ambient_radius: read_ini_field("ambient_radius", section)?,
      ambient_power: read_ini_field("ambient_power", section)?,
      ambient_texture: read_ini_field("ambient_texture", section)?,
      light_texture: read_ini_field("light_texture", section)?,
      light_bone: read_ini_field("light_bone", section)?,
      spot_cone_angle: read_ini_field("spot_cone_angle", section)?,
      glow_texture: read_ini_field("glow_texture", section)?,
      glow_radius: read_ini_field("glow_radius", section)?,
      light_ambient_bone: read_ini_field("light_ambient_bone", section)?,
      volumetric_quality: read_ini_field("volumetric_quality", section)?,
      volumetric_intensity: read_ini_field("volumetric_intensity", section)?,
      volumetric_distance: read_ini_field("volumetric_distance", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeObjectHangingLamp {
  /// Write skeleton data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    self.base.write(writer)?;
    self.skeleton.write(writer)?;

    writer.write_u32::<SpawnByteOrder>(self.main_color)?;
    writer.write_f32::<SpawnByteOrder>(self.main_brightness)?;
    writer.write_null_terminated_win_string(&self.color_animator)?;
    writer.write_f32::<SpawnByteOrder>(self.main_range)?;
    writer.write_u16::<SpawnByteOrder>(self.light_flags)?;
    writer.write_null_terminated_win_string(&self.startup_animation)?;
    writer.write_null_terminated_win_string(&self.fixed_bones)?;
    writer.write_f32::<SpawnByteOrder>(self.health)?;

    writer.write_f32::<SpawnByteOrder>(self.virtual_size)?;
    writer.write_f32::<SpawnByteOrder>(self.ambient_radius)?;
    writer.write_f32::<SpawnByteOrder>(self.ambient_power)?;
    writer.write_null_terminated_win_string(&self.ambient_texture)?;
    writer.write_null_terminated_win_string(&self.light_texture)?;
    writer.write_null_terminated_win_string(&self.light_bone)?;
    writer.write_f32::<SpawnByteOrder>(self.spot_cone_angle)?;
    writer.write_null_terminated_win_string(&self.glow_texture)?;
    writer.write_f32::<SpawnByteOrder>(self.glow_radius)?;

    writer.write_null_terminated_win_string(&self.light_ambient_bone)?;
    writer.write_f32::<SpawnByteOrder>(self.volumetric_quality)?;
    writer.write_f32::<SpawnByteOrder>(self.volumetric_intensity)?;
    writer.write_f32::<SpawnByteOrder>(self.volumetric_distance)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    self.base.export(section, ini)?;
    self.skeleton.export(section, ini)?;

    ini
      .with_section(section)
      .set("main_color", self.main_color.to_string())
      .set("main_brightness", self.main_brightness.to_string())
      .set("color_animator", &self.color_animator)
      .set("main_range", self.main_range.to_string())
      .set("light_flags", self.light_flags.to_string())
      .set("startup_animation", &self.startup_animation)
      .set("fixed_bones", &self.fixed_bones)
      .set("health", self.health.to_string())
      .set("virtual_size", self.virtual_size.to_string())
      .set("ambient_radius", self.ambient_radius.to_string())
      .set("ambient_power", self.ambient_power.to_string())
      .set("ambient_texture", &self.ambient_texture)
      .set("light_texture", &self.light_texture)
      .set("light_bone", &self.light_bone)
      .set("spot_cone_angle", self.spot_cone_angle.to_string())
      .set("glow_texture", &self.glow_texture)
      .set("glow_radius", self.glow_radius.to_string())
      .set("light_ambient_bone", &self.light_ambient_bone)
      .set("volumetric_quality", self.volumetric_quality.to_string())
      .set(
        "volumetric_intensity",
        self.volumetric_intensity.to_string(),
      )
      .set("volumetric_distance", self.volumetric_distance.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_hanging_lamp::AlifeObjectHangingLamp;
  use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
  use crate::data::meta::alife_object_generic::AlifeObjectWriter;
  use crate::data::meta::alife_object_reader::AlifeObjectReader;
  use crate::types::{DatabaseResult, SpawnByteOrder};
  use fileslice::FileSlice;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: AlifeObjectHangingLamp = AlifeObjectHangingLamp {
      base: AlifeObjectDynamicVisual {
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

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 234);

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 234);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 234 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectHangingLamp::read::<SpawnByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
