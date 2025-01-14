use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
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
pub struct AlifeObjectAnomalyZone {
  pub base: AlifeObjectCustomZone,
  pub offline_interactive_radius: f32,
  pub artefact_spawn_count: u16,
  pub artefact_position_offset: u32,
}

impl AlifeObjectReader for AlifeObjectAnomalyZone {
  /// Read anomaly zone object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      base: AlifeObjectCustomZone::read::<T>(reader)?,
      offline_interactive_radius: reader.read_f32::<T>()?,
      artefact_spawn_count: reader.read_u16::<T>()?,
      artefact_position_offset: reader.read_u32::<T>()?,
    })
  }

  /// Import anomaly zone object data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      DatabaseError::new_parse_error(format!(
        "ALife object '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectCustomZone::import(section_name, ltx)?,
      offline_interactive_radius: read_ltx_field("offline_interactive_radius", section)?,
      artefact_spawn_count: read_ltx_field("artefact_spawn_count", section)?,
      artefact_position_offset: read_ltx_field("artefact_position_offset", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeObjectAnomalyZone {
  /// Write anomaly zone object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    self.base.write(writer)?;

    writer.write_f32::<XRayByteOrder>(self.offline_interactive_radius)?;
    writer.write_u16::<XRayByteOrder>(self.artefact_spawn_count)?;
    writer.write_u32::<XRayByteOrder>(self.artefact_position_offset)?;

    Ok(())
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    self.base.export(section_name, ltx)?;

    ltx
      .with_section(section_name)
      .set(
        "offline_interactive_radius",
        self.offline_interactive_radius.to_string(),
      )
      .set(
        "artefact_spawn_count",
        self.artefact_spawn_count.to_string(),
      )
      .set(
        "artefact_position_offset",
        self.artefact_position_offset.to_string(),
      );

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
  use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
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

    let original: AlifeObjectAnomalyZone = AlifeObjectAnomalyZone {
      base: AlifeObjectCustomZone {
        base: AlifeObjectSpaceRestrictor {
          base: AlifeObjectAbstract {
            game_vertex_id: 241,
            distance: 55.3,
            direct_control: 4,
            level_vertex_id: 87,
            flags: 12,
            custom_data: "".to_string(),
            story_id: 6211,
            spawn_story_id: 143,
          },
          shape: vec![
            Shape::Sphere((Vector3d::new(0.5, 0.5, 0.5), 1.0)),
            Shape::Box((
              Vector3d::new(4.1, 1.1, 32.1),
              Vector3d::new(1.1, 2.2, 3.3),
              Vector3d::new(4.0, 5.0, 1.4),
              Vector3d::new(9.2, 8.3, 1.0),
            )),
          ],
          restrictor_type: 4,
        },
        max_power: 255.33,
        owner_id: 1,
        enabled_time: 3312,
        disabled_time: 521,
        start_time_shift: 250,
      },
      offline_interactive_radius: -3231.1,
      artefact_spawn_count: 3,
      artefact_position_offset: 5,
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 125);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 125);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 125 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectAnomalyZone::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
