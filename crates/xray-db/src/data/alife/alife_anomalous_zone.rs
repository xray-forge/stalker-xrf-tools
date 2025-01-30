use crate::data::alife::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
use crate::data::generic::time::Time;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::export::file_import::read_ltx_field;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeAnomalousZone {
  pub base: AlifeObjectAnomalyZone,
  pub last_spawn_time: Option<Time>,
}

impl AlifeObjectReader for AlifeAnomalousZone {
  /// Read anomalous zone object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      base: AlifeObjectAnomalyZone::read::<T>(reader)?,
      last_spawn_time: Time::read_optional::<T>(reader)?,
    })
  }

  /// Import anomalous zone object data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "ALife object '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectAnomalyZone::import(section_name, ltx)?,
      last_spawn_time: Time::import_from_string(&read_ltx_field::<String>(
        "last_spawn_time",
        section,
      )?)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeAnomalousZone {
  /// Write alife anomalous zone data to the writer.
  fn write(&self, writer: &mut ChunkWriter) -> XRayResult {
    self.base.write(writer)?;

    Time::write_optional::<XRayByteOrder>(self.last_spawn_time.as_ref(), writer)?;

    Ok(())
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    self.base.export(section_name, ltx)?;

    ltx.with_section(section_name).set(
      "last_spawn_time",
      Time::export_to_string(self.last_spawn_time.as_ref()),
    );

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::alife_anomalous_zone::AlifeAnomalousZone;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
  use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
  use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::generic::shape::Shape;
  use crate::data::generic::time::Time;
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

    let original: AlifeAnomalousZone = AlifeAnomalousZone {
      base: AlifeObjectAnomalyZone {
        base: AlifeObjectCustomZone {
          base: AlifeObjectSpaceRestrictor {
            base: AlifeObjectAbstract {
              game_vertex_id: 34565,
              distance: 234.0,
              direct_control: 2346,
              level_vertex_id: 7357,
              flags: 55,
              custom_data: String::from("custom-data"),
              story_id: 8567,
              spawn_story_id: 7685,
            },
            shape: vec![
              Shape::Sphere((Vector3d::new(2.5, 5.1, 1.5), 1.0)),
              Shape::Box((
                Vector3d::new(4.1, 1.1, 3.1),
                Vector3d::new(1.1, 3.2, 3.3),
                Vector3d::new(4.0, 5.0, 6.4),
                Vector3d::new(9.2, 8.3, 3.0),
              )),
            ],
            restrictor_type: 4,
          },
          max_power: 1.0,
          owner_id: 64,
          enabled_time: 235,
          disabled_time: 3457,
          start_time_shift: 253,
        },
        offline_interactive_radius: 330.0,
        artefact_spawn_count: 4,
        artefact_position_offset: 12,
      },
      last_spawn_time: Some(Time {
        year: 22,
        month: 10,
        day: 24,
        hour: 20,
        minute: 30,
        second: 50,
        millis: 250,
      }),
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 145);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 145);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 145 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeAnomalousZone::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
