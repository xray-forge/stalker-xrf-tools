use crate::data::alife::alife_object_smart_cover::AlifeObjectSmartCover;
use crate::data::alife::alife_smart_cover_loophole::AlifeSmartCoverLoophole;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::export::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

/// Represents script extension of base server smart cover class.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeSmartCover {
  pub base: AlifeObjectSmartCover,
  pub last_description: String,
  pub loopholes: Vec<AlifeSmartCoverLoophole>,
}

impl AlifeObjectReader for AlifeSmartCover {
  /// Read smart cover data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let base: AlifeObjectSmartCover = AlifeObjectSmartCover::read::<T>(reader)?;

    let last_description: String = reader.read_null_terminated_win_string()?;
    let count: u8 = reader.read_u8()?;
    let mut loopholes: Vec<AlifeSmartCoverLoophole> = Vec::new();

    for _ in 0..count {
      let name: String = reader.read_null_terminated_win_string()?;
      let enabled: u8 = reader.read_u8()?;

      loopholes.push(AlifeSmartCoverLoophole { name, enabled })
    }

    Ok(Self {
      base,
      last_description,
      loopholes,
    })
  }

  /// Import smart cover data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "ALife object '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectSmartCover::import(section_name, ltx)?,
      last_description: read_ltx_field("last_description", section)?,
      loopholes: AlifeSmartCoverLoophole::string_to_list(&read_ltx_field::<String>(
        "loopholes",
        section,
      )?)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeSmartCover {
  /// Write smart cover data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> XRayResult {
    self.base.write(writer)?;

    writer.write_null_terminated_win_string(&self.last_description)?;
    writer.write_u8(self.loopholes.len() as u8)?;

    for loophole in &self.loopholes {
      writer.write_null_terminated_win_string(&loophole.name)?;
      writer.write_u8(loophole.enabled)?;
    }

    Ok(())
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    self.base.export(section_name, ltx)?;

    ltx
      .with_section(section_name)
      .set("last_description", &self.last_description)
      .set("loopholes", self.loopholes.len().to_string())
      .set(
        "loopholes",
        AlifeSmartCoverLoophole::list_to_string(&self.loopholes),
      );

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic::AlifeObjectDynamic;
  use crate::data::alife::alife_object_smart_cover::AlifeObjectSmartCover;
  use crate::data::generic::shape::Shape;
  use crate::data::generic::vector_3d::Vector3d;
  use crate::data::meta::alife_object_generic::AlifeObjectWriter;
  use crate::data::meta::alife_object_reader::AlifeObjectReader;
  use fileslice::FileSlice;
  use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: AlifeObjectSmartCover = AlifeObjectSmartCover {
      base: AlifeObjectDynamic {
        base: AlifeObjectAbstract {
          game_vertex_id: 6734,
          distance: 38.287,
          direct_control: 234760,
          level_vertex_id: 29836,
          flags: 68,
          custom_data: String::from("custom-data"),
          story_id: 8723,
          spawn_story_id: 160278,
        },
      },
      shape: vec![
        Shape::Sphere((Vector3d::new(2.5, 1.3, -4.125), 5.5)),
        Shape::Box((
          Vector3d::new(1.1, 1.1, 6.1),
          Vector3d::new(1.4, 2.2, 6.3),
          Vector3d::new(4.0, 3.0, 6.4),
          Vector3d::new(9.2, 8.3, 6.0),
        )),
      ],
      description: String::from("description"),
      hold_position_time: 34.0,
      enter_min_enemy_distance: 23.0,
      exit_min_enemy_distance: 36.0,
      is_combat_cover: 1,
      can_fire: 1,
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 131);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 131);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 131 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectSmartCover::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
