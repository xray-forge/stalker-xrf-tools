use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
use crate::data::alife::alife_object_motion::AlifeObjectMotion;
use crate::data::generic::time::Time;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::file_import::read_ltx_field;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectTorridZone {
  pub base: AlifeObjectCustomZone,
  pub motion: AlifeObjectMotion,
  pub last_spawn_time: Option<Time>,
}

impl AlifeObjectReader<AlifeObjectTorridZone> for AlifeObjectTorridZone {
  /// Read zone object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      base: AlifeObjectCustomZone::read::<T>(reader)?,
      motion: AlifeObjectMotion::read::<T>(reader)?,
      last_spawn_time: reader.read_xr_optional::<T, Time>()?,
    })
  }

  /// Import torrid zone object data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "ALife object '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectCustomZone::import(section_name, ltx)?,
      motion: AlifeObjectMotion::import(section_name, ltx)?,
      last_spawn_time: Time::from_str_optional(&read_ltx_field::<String>(
        "last_spawn_time",
        section,
      )?)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeObjectTorridZone {
  /// Write zone object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> XRayResult {
    self.base.write(writer)?;
    self.motion.write(writer)?;

    writer.write_xr_optional::<XRayByteOrder, Time>(self.last_spawn_time.as_ref())?;

    Ok(())
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    self.base.export(section_name, ltx)?;
    self.motion.export(section_name, ltx)?;

    ltx.with_section(section_name).set(
      "last_spawn_time",
      Time::export_to_string(self.last_spawn_time.as_ref()),
    );

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
  use crate::data::alife::alife_object_motion::AlifeObjectMotion;
  use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::alife::alife_object_torrid_zone::AlifeObjectTorridZone;
  use crate::data::generic::time::Time;
  use crate::data::meta::alife_object_generic::AlifeObjectWriter;
  use crate::data::meta::alife_object_reader::AlifeObjectReader;
  use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };
  use xray_test_utils::FileSlice;

  #[test]
  fn test_read_write() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: AlifeObjectTorridZone = AlifeObjectTorridZone {
      base: AlifeObjectCustomZone {
        base: AlifeObjectSpaceRestrictor {
          base: AlifeObjectAbstract {
            game_vertex_id: 8469,
            distance: 85.323,
            direct_control: 203678,
            level_vertex_id: 8726,
            flags: 76,
            custom_data: String::from("custom-data"),
            story_id: 295786,
            spawn_story_id: 620,
          },
          shape: vec![],
          restrictor_type: 4,
        },
        max_power: 1.0,
        owner_id: 286,
        enabled_time: 1,
        disabled_time: 1,
        start_time_shift: 1,
      },
      motion: AlifeObjectMotion {
        motion_name: String::from("motion-name"),
      },
      last_spawn_time: Some(Time {
        year: 12,
        month: 6,
        day: 3,
        hour: 24,
        minute: 3,
        second: 30,
        millis: 500,
      }),
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 81);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 81);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 81 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectTorridZone::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
