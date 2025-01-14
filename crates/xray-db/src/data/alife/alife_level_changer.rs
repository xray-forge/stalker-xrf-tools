use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
use crate::data::generic::vector_3d::Vector3d;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::error::DatabaseError;
use crate::export::file_import::read_ltx_field;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeLevelChanger {
  pub base: AlifeObjectSpaceRestrictor,
  pub dest_game_vertex_id: u16,
  pub dest_level_vertex_id: u32,
  pub dest_position: Vector3d<f32>,
  pub dest_direction: Vector3d<f32>,
  pub angle_y: f32,
  pub dest_level_name: String,
  pub dest_graph_point: String,
  pub silent_mode: u8,
  pub enabled: u8,
  pub hint: String,
  pub save_marker: u16,
}

impl AlifeObjectReader for AlifeLevelChanger {
  /// Read alife level changer object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let object: Self = Self {
      base: AlifeObjectSpaceRestrictor::read::<T>(reader)?,
      dest_game_vertex_id: reader.read_u16::<T>()?,
      dest_level_vertex_id: reader.read_u32::<T>()?,
      dest_position: Vector3d::read::<T>(reader)?,
      dest_direction: Vector3d::read::<T>(reader)?,
      angle_y: reader.read_f32::<T>()?,
      dest_level_name: reader.read_null_terminated_win_string()?,
      dest_graph_point: reader.read_null_terminated_win_string()?,
      silent_mode: reader.read_u8()?,
      enabled: reader.read_u8()?,
      hint: reader.read_null_terminated_win_string()?,
      save_marker: reader.read_u16::<T>()?,
    };

    assert_eq!(
      object.save_marker, 26,
      "Unexpected script data provided for level changer"
    );

    Ok(object)
  }

  /// Import alife level changer object data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      DatabaseError::new_parse_error(format!(
        "ALife object '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectSpaceRestrictor::import(section_name, ltx)?,
      dest_game_vertex_id: read_ltx_field("dest_game_vertex_id", section)?,
      dest_level_vertex_id: read_ltx_field("dest_level_vertex_id", section)?,
      dest_position: read_ltx_field("dest_position", section)?,
      dest_direction: read_ltx_field("dest_direction", section)?,
      angle_y: read_ltx_field("angle_y", section)?,
      dest_level_name: read_ltx_field("dest_level_name", section)?,
      dest_graph_point: read_ltx_field("dest_graph_point", section)?,
      silent_mode: read_ltx_field("silent_mode", section)?,
      enabled: read_ltx_field("enabled", section)?,
      hint: read_ltx_field("hint", section)?,
      save_marker: read_ltx_field("save_marker", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeLevelChanger {
  /// Write object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    self.base.write(writer)?;

    writer.write_u16::<XRayByteOrder>(self.dest_game_vertex_id)?;
    writer.write_u32::<XRayByteOrder>(self.dest_level_vertex_id)?;

    self.dest_position.write::<XRayByteOrder>(writer)?;
    self.dest_direction.write::<XRayByteOrder>(writer)?;

    writer.write_f32::<XRayByteOrder>(self.angle_y)?;
    writer.write_null_terminated_win_string(&self.dest_level_name)?;
    writer.write_null_terminated_win_string(&self.dest_graph_point)?;
    writer.write_u8(self.silent_mode)?;

    writer.write_u8(self.enabled)?;
    writer.write_null_terminated_win_string(&self.hint)?;
    writer.write_u16::<XRayByteOrder>(self.save_marker)?;

    Ok(())
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    self.base.export(section_name, ltx)?;

    ltx
      .with_section(section_name)
      .set("dest_game_vertex_id", self.dest_game_vertex_id.to_string())
      .set(
        "dest_level_vertex_id",
        self.dest_level_vertex_id.to_string(),
      )
      .set("dest_position", self.dest_position.to_string())
      .set("dest_direction", self.dest_direction.to_string())
      .set("angle_y", self.angle_y.to_string())
      .set("dest_level_name", self.dest_level_name.to_string())
      .set("dest_graph_point", self.dest_graph_point.to_string())
      .set("silent_mode", self.silent_mode.to_string())
      .set("enabled", self.enabled.to_string())
      .set("hint", self.hint.to_string())
      .set("save_marker", self.save_marker.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::alife_level_changer::AlifeLevelChanger;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::generic::shape::Shape;
  use crate::data::generic::vector_3d::Vector3d;
  use crate::data::meta::alife_object_generic::AlifeObjectWriter;
  use crate::data::meta::alife_object_reader::AlifeObjectReader;
  use crate::types::DatabaseResult;
  use fileslice::FileSlice;
  use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: AlifeLevelChanger = AlifeLevelChanger {
      base: AlifeObjectSpaceRestrictor {
        base: AlifeObjectAbstract {
          game_vertex_id: 12451,
          distance: 253.0,
          direct_control: 12,
          level_vertex_id: 331,
          flags: 33,
          custom_data: String::from("custom-data"),
          story_id: 4553,
          spawn_story_id: 213,
        },
        shape: vec![
          Shape::Sphere((Vector3d::new(4.5, 0.5, 11.5), 1.0)),
          Shape::Box((
            Vector3d::new(1.5, 2.5, 73.1),
            Vector3d::new(5.1, 3.2, 2.3),
            Vector3d::new(1.0, 3.0, 6.4),
            Vector3d::new(9.2, 3.3, 3.0),
          )),
        ],
        restrictor_type: 3,
      },
      dest_game_vertex_id: 312,
      dest_level_vertex_id: 3312,
      dest_position: Vector3d::new(4.0, 3.0, 2.0),
      dest_direction: Vector3d::new(1.0, 2.0, 3.0),
      angle_y: 35.0,
      dest_level_name: String::from("dest-level"),
      dest_graph_point: String::from("dest-graph-point"),
      silent_mode: 1,
      enabled: 1,
      hint: String::from("hint"),
      save_marker: 26,
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 177);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 177);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 177 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeLevelChanger = AlifeLevelChanger::read::<XRayByteOrder>(&mut reader)?;

    assert_eq!(read_object, original);

    Ok(())
  }
}
