use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_dynamic::AlifeObjectDynamic;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::data::shape::Shape;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ini_field;
use crate::types::{DatabaseResult, SpawnByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectSmartCover {
  pub base: AlifeObjectDynamic,
  pub shape: Vec<Shape>,
  pub description: String,
  pub hold_position_time: f32,
  pub enter_min_enemy_distance: f32,
  pub exit_min_enemy_distance: f32,
  pub is_combat_cover: u8,
  pub can_fire: u8,
}

impl AlifeObjectReader<AlifeObjectSmartCover> for AlifeObjectSmartCover {
  /// Read smart cover object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      base: AlifeObjectDynamic::read::<T>(reader)?,
      shape: reader.read_shapes::<SpawnByteOrder>()?,
      description: reader.read_null_terminated_win_string()?,
      hold_position_time: reader.read_f32::<SpawnByteOrder>()?,
      enter_min_enemy_distance: reader.read_f32::<SpawnByteOrder>()?,
      exit_min_enemy_distance: reader.read_f32::<SpawnByteOrder>()?,
      is_combat_cover: reader.read_u8()?,
      can_fire: reader.read_u8()?,
    })
  }

  /// Import smart cover object data from ini config section.
  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ini.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "ALife object '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectDynamic::import(section_name, ini)?,
      shape: Shape::import_list(section)?,
      description: read_ini_field("description", section)?,
      hold_position_time: read_ini_field("hold_position_time", section)?,
      enter_min_enemy_distance: read_ini_field("enter_min_enemy_distance", section)?,
      exit_min_enemy_distance: read_ini_field("exit_min_enemy_distance", section)?,
      is_combat_cover: read_ini_field("is_combat_cover", section)?,
      can_fire: read_ini_field("can_fire", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeObjectSmartCover {
  /// Write smart cover object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    self.base.write(writer)?;

    writer.write_shapes_list::<SpawnByteOrder>(&self.shape)?;
    writer.write_null_terminated_win_string(&self.description)?;
    writer.write_f32::<SpawnByteOrder>(self.hold_position_time)?;
    writer.write_f32::<SpawnByteOrder>(self.enter_min_enemy_distance)?;
    writer.write_f32::<SpawnByteOrder>(self.exit_min_enemy_distance)?;
    writer.write_u8(self.is_combat_cover)?;
    writer.write_u8(self.can_fire)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    self.base.export(section, ini)?;

    ini
      .with_section(section)
      .set("description", &self.description)
      .set("hold_position_time", self.hold_position_time.to_string())
      .set(
        "enter_min_enemy_distance",
        self.enter_min_enemy_distance.to_string(),
      )
      .set(
        "exit_min_enemy_distance",
        self.exit_min_enemy_distance.to_string(),
      )
      .set(
        "exit_min_enemy_distance",
        self.exit_min_enemy_distance.to_string(),
      )
      .set("is_combat_cover", self.is_combat_cover.to_string())
      .set("can_fire", self.can_fire.to_string());

    Shape::export_list(&self.shape, section, ini);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic::AlifeObjectDynamic;
  use crate::data::alife::alife_object_smart_cover::AlifeObjectSmartCover;
  use crate::data::meta::alife_object_generic::AlifeObjectWriter;
  use crate::data::meta::alife_object_reader::AlifeObjectReader;
  use crate::data::shape::Shape;
  use crate::data::vector_3d::Vector3d;
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

    let original: AlifeObjectSmartCover = AlifeObjectSmartCover {
      base: AlifeObjectDynamic {
        base: AlifeObjectAbstract {
          game_vertex_id: 1001,
          distance: 65.25,
          direct_control: 412421,
          level_vertex_id: 66231,
          flags: 33,
          custom_data: String::from("custom_data"),
          story_id: 400,
          spawn_story_id: 32,
        },
      },
      shape: vec![
        Shape::Sphere((Vector3d::new(0.5, 0.3, -0.125), 2.5)),
        Shape::Box((
          Vector3d::new(1.1, 1.1, 3.1),
          Vector3d::new(1.4, 2.2, 3.3),
          Vector3d::new(4.0, 3.0, 5.4),
          Vector3d::new(9.2, 8.3, 2.0),
        )),
      ],
      description: String::from("test-description"),
      hold_position_time: 4.532,
      enter_min_enemy_distance: 32.4,
      exit_min_enemy_distance: 25.3,
      is_combat_cover: 0,
      can_fire: 1,
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 136);

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 136);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 136 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectSmartCover::read::<SpawnByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
