use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::generic::shape::Shape;
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
pub struct AlifeObjectSpaceRestrictor {
  pub base: AlifeObjectAbstract,
  pub shape: Vec<Shape>,
  pub restrictor_type: u8,
}

impl AlifeObjectReader for AlifeObjectSpaceRestrictor {
  /// Read generic space restrictor data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      base: AlifeObjectAbstract::read::<T>(reader)?,
      shape: Shape::read_list::<T>(reader)?,
      restrictor_type: reader.read_u8()?,
    })
  }

  /// Import generic space restrictor data from the chunk.
  fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      DatabaseError::new_parse_error(format!(
        "ALife object '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectAbstract::import(section_name, ltx)?,
      shape: Shape::import_list(section)?,
      restrictor_type: read_ltx_field("restrictor_type", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeObjectSpaceRestrictor {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    self.base.write(writer)?;

    Shape::write_list::<XRayByteOrder>(&self.shape, writer)?;

    writer.write_u8(self.restrictor_type)?;

    Ok(())
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    self.base.export(section_name, ltx)?;

    ltx
      .with_section(section_name)
      .set("restrictor_type", self.restrictor_type.to_string());

    Shape::export_list(&self.shape, section_name, ltx);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::generic::shape::Shape;
  use crate::data::generic::vector_3d::Vector3d;
  use crate::data::meta::alife_object_generic::AlifeObjectWriter;
  use crate::data::meta::alife_object_reader::AlifeObjectReader;
  use crate::export::file::open_ltx_config;
  use crate::types::DatabaseResult;
  use fileslice::FileSlice;
  use std::fs::File;
  use std::path::Path;
  use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_ltx::Ltx;
  use xray_test_utils::utils::{
    get_absolute_test_sample_file_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_file, overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor {
      base: AlifeObjectAbstract {
        game_vertex_id: 1001,
        distance: 65.25,
        direct_control: 412421,
        level_vertex_id: 66231,
        flags: 33,
        custom_data: String::from("custom_data"),
        story_id: 400,
        spawn_story_id: 25,
      },
      shape: vec![
        Shape::Sphere((Vector3d::new(0.5, 0.5, 0.5), 1.0)),
        Shape::Box((
          Vector3d::new(1.1, 1.1, 3.1),
          Vector3d::new(1.1, 2.2, 3.3),
          Vector3d::new(4.0, 5.0, 6.4),
          Vector3d::new(9.2, 8.3, 7.0),
        )),
      ],
      restrictor_type: 2,
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 106);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 106);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 106 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectSpaceRestrictor::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }

  #[test]
  fn test_import_export() -> DatabaseResult {
    let config_path: &Path = &get_absolute_test_sample_file_path(file!(), "import_export.ltx");
    let mut file: File = overwrite_file(config_path)?;
    let mut ltx: Ltx = Ltx::new();

    let first: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor {
      base: AlifeObjectAbstract {
        game_vertex_id: 2593,
        distance: 34.7,
        direct_control: 235,
        level_vertex_id: 245423,
        flags: 32,
        custom_data: String::from("test-custom-data"),
        story_id: 253423,
        spawn_story_id: 457,
      },
      shape: vec![],
      restrictor_type: 3,
    };

    let second: AlifeObjectSpaceRestrictor = AlifeObjectSpaceRestrictor {
      base: AlifeObjectAbstract {
        game_vertex_id: 45724,
        distance: 43.0,
        direct_control: 236623,
        level_vertex_id: 2364,
        flags: 75,
        custom_data: String::new(),
        story_id: 253,
        spawn_story_id: 7546,
      },
      shape: vec![
        Shape::Sphere((Vector3d::new(54.5, 0.5, 11.5), 1.0)),
        Shape::Box((
          Vector3d::new(3.5, 2.5, 73.1),
          Vector3d::new(55.1, 1.2, 2.3),
          Vector3d::new(51.0, 7.0, 3.4),
          Vector3d::new(59.2, 3.3, 4.1),
        )),
      ],
      restrictor_type: 4,
    };

    first.export("first", &mut ltx)?;
    second.export("second", &mut ltx)?;

    ltx.write_to(&mut file)?;

    let source: Ltx = open_ltx_config(config_path)?;

    assert_eq!(AlifeObjectSpaceRestrictor::import("first", &source)?, first);
    assert_eq!(
      AlifeObjectSpaceRestrictor::import("second", &source)?,
      second
    );

    Ok(())
  }
}
