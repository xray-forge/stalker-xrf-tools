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
pub struct AlifeObjectPhysic {
  pub base: AlifeObjectDynamicVisual,
  pub skeleton: AlifeObjectSkeleton,
  pub physic_type: u32,
  pub mass: f32,
  pub fixed_bones: String,
}

impl AlifeObjectReader<AlifeObjectPhysic> for AlifeObjectPhysic {
  /// Read alife physic object from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      base: AlifeObjectDynamicVisual::read::<T>(reader)?,
      skeleton: AlifeObjectSkeleton::read::<T>(reader)?,
      physic_type: reader.read_u32::<SpawnByteOrder>()?,
      mass: reader.read_f32::<SpawnByteOrder>()?,
      fixed_bones: reader.read_null_terminated_win_string()?,
    })
  }

  /// Import alife physic object data from ini config section.
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
      physic_type: read_ini_field("physic_type", section)?,
      mass: read_ini_field("mass", section)?,
      fixed_bones: read_ini_field("fixed_bones", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeObjectPhysic {
  /// Write alife physic object into the chunk.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult<()> {
    self.base.write(writer)?;
    self.skeleton.write(writer)?;

    writer.write_u32::<SpawnByteOrder>(self.physic_type)?;
    writer.write_f32::<SpawnByteOrder>(self.mass)?;
    writer.write_null_terminated_win_string(&self.fixed_bones)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult<()> {
    self.base.export(section, ini)?;
    self.skeleton.export(section, ini)?;

    ini
      .with_section(section)
      .set("physic_type", self.physic_type.to_string())
      .set("mass", self.mass.to_string())
      .set("fixed_bones", &self.fixed_bones);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_physic::AlifeObjectPhysic;
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

    let original: AlifeObjectPhysic = AlifeObjectPhysic {
      base: AlifeObjectDynamicVisual {
        base: AlifeObjectAbstract {
          game_vertex_id: 35794,
          distance: 25.23,
          direct_control: 1243,
          level_vertex_id: 34623,
          flags: 62,
          custom_data: String::from("custom-data"),
          story_id: 825679,
          spawn_story_id: 1452,
        },
        visual_name: String::from("visual-name"),
        visual_flags: 34,
      },
      skeleton: AlifeObjectSkeleton {
        name: String::from("skeleton-name"),
        flags: 0,
        source_id: 2153,
      },
      physic_type: 6,
      mass: 5.0,
      fixed_bones: String::from("fixed-bones"),
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 88);

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 88);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 88 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectPhysic::read::<SpawnByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
