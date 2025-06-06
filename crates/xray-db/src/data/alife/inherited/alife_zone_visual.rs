use crate::data::alife::inherited::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
use crate::data::alife::inherited::alife_object_visual::AlifeObjectVisual;
use crate::data::generic::time::Time;
use crate::export::LtxImportExport;
use crate::file_import::read_ltx_field;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeZoneVisual {
  pub base: AlifeObjectAnomalyZone,
  pub visual: AlifeObjectVisual,
  pub idle_animation: String,
  pub attack_animation: String,
  pub last_spawn_time: Option<Time>,
}

impl ChunkReadWrite for AlifeZoneVisual {
  /// Read visual zone data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      base: reader.read_xr::<T, _>()?,
      visual: reader.read_xr::<T, _>()?,
      idle_animation: reader
        .has_data()
        .then(|| reader.read_w1251_string().unwrap())
        .unwrap_or(String::new()),
      attack_animation: reader
        .has_data()
        .then(|| reader.read_w1251_string().unwrap())
        .unwrap_or(String::new()),
      last_spawn_time: reader.read_xr_optional::<T, Time>()?,
    })
  }

  /// Write visual zone data into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    writer.write_xr::<T, _>(&self.base)?;
    writer.write_xr::<T, _>(&self.visual)?;
    writer.write_w1251_string(&self.idle_animation)?;
    writer.write_w1251_string(&self.attack_animation)?;
    writer.write_xr_optional::<T, Time>(self.last_spawn_time.as_ref())?;

    Ok(())
  }
}

impl LtxImportExport for AlifeZoneVisual {
  /// Import visual zone data from ltx config section.
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
      visual: AlifeObjectVisual::import(section_name, ltx)?,
      idle_animation: read_ltx_field("zone_visual.idle_animation", section)?,
      attack_animation: read_ltx_field("zone_visual.attack_animation", section)?,
      last_spawn_time: Time::from_str_optional(&read_ltx_field::<String>(
        "zone_visual.last_spawn_time",
        section,
      )?)?,
    })
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    self.base.export(section_name, ltx)?;
    self.visual.export(section_name, ltx)?;

    ltx
      .with_section(section_name)
      .set("zone_visual.idle_animation", &self.idle_animation)
      .set("zone_visual.attack_animation", &self.attack_animation)
      .set(
        "zone_visual.last_spawn_time",
        Time::export_to_string(self.last_spawn_time.as_ref()),
      );

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::inherited::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::inherited::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
  use crate::data::alife::inherited::alife_object_custom_zone::AlifeObjectCustomZone;
  use crate::data::alife::inherited::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::alife::inherited::alife_object_visual::AlifeObjectVisual;
  use crate::data::alife::inherited::alife_zone_visual::AlifeZoneVisual;
  use crate::data::generic::shape::Shape;
  use crate::data::generic::time::Time;
  use crate::data::generic::vector_3d::Vector3d;
  use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };
  use xray_test_utils::FileSlice;

  #[test]
  fn test_read_write_empty() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write_empty.chunk");

    let original: AlifeZoneVisual = AlifeZoneVisual {
      base: AlifeObjectAnomalyZone {
        base: AlifeObjectCustomZone {
          base: AlifeObjectSpaceRestrictor {
            base: AlifeObjectAbstract {
              game_vertex_id: 465,
              distance: 6.125,
              direct_control: 1,
              level_vertex_id: 2135,
              flags: 253,
              custom_data: String::from("custom_data"),
              story_id: 364,
              spawn_story_id: 754,
            },
            shape: vec![
              Shape::Sphere((Vector3d::new(53.5, 50.5, 11.5), 2.0)),
              Shape::Box((
                Vector3d::new(51.5, 71.1, 53.1),
                Vector3d::new(51.1, 72.2, 53.3),
                Vector3d::new(54.0, 75.0, 56.4),
                Vector3d::new(59.2, 78.3, 53.0),
              )),
            ],
            restrictor_type: 2,
          },
          max_power: 36.0,
          owner_id: 63,
          enabled_time: 636,
          disabled_time: 266,
          start_time_shift: 6,
        },
        offline_interactive_radius: 3.255,
        artefact_spawn_count: 36,
        artefact_position_offset: 25,
      },
      visual: AlifeObjectVisual {
        visual_name: String::from("name"),
        visual_flags: 65,
      },
      idle_animation: String::from(""),
      attack_animation: String::from(""),
      last_spawn_time: None,
    };

    original.write::<XRayByteOrder>(&mut writer)?;

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
      AlifeZoneVisual::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_read_write() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: AlifeZoneVisual = AlifeZoneVisual {
      base: AlifeObjectAnomalyZone {
        base: AlifeObjectCustomZone {
          base: AlifeObjectSpaceRestrictor {
            base: AlifeObjectAbstract {
              game_vertex_id: 4500,
              distance: 3.125,
              direct_control: 1,
              level_vertex_id: 40,
              flags: 33,
              custom_data: String::from("custom_data"),
              story_id: 420,
              spawn_story_id: 101,
            },
            shape: vec![
              Shape::Sphere((Vector3d::new(3.5, 0.5, 11.5), 1.0)),
              Shape::Box((
                Vector3d::new(1.5, 1.1, 3.1),
                Vector3d::new(1.1, 2.2, 3.3),
                Vector3d::new(4.0, 5.0, 6.4),
                Vector3d::new(9.2, 8.3, 3.0),
              )),
            ],
            restrictor_type: 2,
          },
          max_power: 12.0,
          owner_id: 638,
          enabled_time: 100,
          disabled_time: 253,
          start_time_shift: 40,
        },
        offline_interactive_radius: 274.621,
        artefact_spawn_count: 3,
        artefact_position_offset: 40,
      },
      visual: AlifeObjectVisual {
        visual_name: String::from("visual_name"),
        visual_flags: 36,
      },
      idle_animation: String::from("idle_animation"),
      attack_animation: String::from("attack_animation"),
      last_spawn_time: Some(Time::new_mock()),
    };

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 190);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 190);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 190 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeZoneVisual::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
