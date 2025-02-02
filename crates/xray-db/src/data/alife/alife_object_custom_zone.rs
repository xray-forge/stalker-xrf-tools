use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectCustomZone {
  pub base: AlifeObjectSpaceRestrictor,
  pub max_power: f32,
  pub owner_id: u32,
  pub enabled_time: u32,
  pub disabled_time: u32,
  pub start_time_shift: u32,
}

impl AlifeObjectReader<AlifeObjectCustomZone> for AlifeObjectCustomZone {
  /// Read ALife custom zone object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      base: AlifeObjectSpaceRestrictor::read::<T>(reader)?,
      max_power: reader.read_f32::<XRayByteOrder>()?,
      owner_id: reader.read_u32::<XRayByteOrder>()?,
      enabled_time: reader.read_u32::<XRayByteOrder>()?,
      disabled_time: reader.read_u32::<XRayByteOrder>()?,
      start_time_shift: reader.read_u32::<XRayByteOrder>()?,
    })
  }

  /// Import ALife custom zone object data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "ALife object '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectSpaceRestrictor::import(section_name, ltx)?,
      max_power: read_ltx_field("max_power", section)?,
      owner_id: read_ltx_field("owner_id", section)?,
      enabled_time: read_ltx_field("enabled_time", section)?,
      disabled_time: read_ltx_field("disabled_time", section)?,
      start_time_shift: read_ltx_field("start_time_shift", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeObjectCustomZone {
  /// Write custom zone object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> XRayResult {
    self.base.write(writer)?;

    writer.write_f32::<XRayByteOrder>(self.max_power)?;
    writer.write_u32::<XRayByteOrder>(self.owner_id)?;
    writer.write_u32::<XRayByteOrder>(self.enabled_time)?;
    writer.write_u32::<XRayByteOrder>(self.disabled_time)?;
    writer.write_u32::<XRayByteOrder>(self.start_time_shift)?;

    Ok(())
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    self.base.export(section_name, ltx)?;

    ltx
      .with_section(section_name)
      .set("max_power", self.max_power.to_string())
      .set("owner_id", self.owner_id.to_string())
      .set("enabled_time", self.enabled_time.to_string())
      .set("disabled_time", self.disabled_time.to_string())
      .set("start_time_shift", self.start_time_shift.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
  use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::generic::shape::Shape;
  use crate::data::generic::vector_3d::Vector3d;
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

    let original: AlifeObjectCustomZone = AlifeObjectCustomZone {
      base: AlifeObjectSpaceRestrictor {
        base: AlifeObjectAbstract {
          game_vertex_id: 42343,
          distance: 255.4,
          direct_control: 3,
          level_vertex_id: 1003,
          flags: 32,
          custom_data: String::from("custom-data"),
          story_id: 441,
          spawn_story_id: 254,
        },
        shape: vec![
          Shape::Sphere((Vector3d::new(2.5, 3.5, 1.5), 1.0)),
          Shape::Box((
            Vector3d::new(1.1, 1.1, 3.1),
            Vector3d::new(1.1, 2.2, 3.3),
            Vector3d::new(4.0, 5.0, 6.4),
            Vector3d::new(9.2, 8.3, 7.0),
          )),
        ],
        restrictor_type: 3,
      },
      max_power: 2.0,
      owner_id: 553,
      enabled_time: 100,
      disabled_time: 220,
      start_time_shift: 300,
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 126);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 126);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 126 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectCustomZone::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
