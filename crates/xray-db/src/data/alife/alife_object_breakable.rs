use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::error::database_parse_error::DatabaseParseError;
use crate::export::file_import::read_ltx_field;
use crate::types::{DatabaseResult, SpawnByteOrder};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectBreakable {
  pub base: AlifeObjectDynamicVisual,
  pub health: f32,
}

impl AlifeObjectReader for AlifeObjectBreakable {
  /// Read alife breakable object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      base: AlifeObjectDynamicVisual::read::<T>(reader)?,
      health: reader.read_f32::<SpawnByteOrder>()?,
    })
  }

  /// Import alife breakable object data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      DatabaseParseError::new_database_error(format!(
        "ALife object '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectDynamicVisual::import(section_name, ltx)?,
      health: read_ltx_field("health", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeObjectBreakable {
  /// Write alife breakable object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    self.base.write(writer)?;

    writer.write_f32::<SpawnByteOrder>(self.health)?;

    Ok(())
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    self.base.export(section_name, ltx)?;

    ltx
      .with_section(section_name)
      .set("health", self.health.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_breakable::AlifeObjectBreakable;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::meta::alife_object_generic::AlifeObjectWriter;
  use crate::data::meta::alife_object_reader::AlifeObjectReader;
  use crate::types::{DatabaseResult, SpawnByteOrder};
  use fileslice::FileSlice;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let object: AlifeObjectBreakable = AlifeObjectBreakable {
      base: AlifeObjectDynamicVisual {
        base: AlifeObjectAbstract {
          game_vertex_id: 3200,
          distance: 50.0,
          direct_control: 6219,
          level_vertex_id: 239805,
          flags: 562,
          custom_data: String::from("custom-data"),
          story_id: 92378,
          spawn_story_id: 235067,
        },
        visual_name: String::from("visual-name"),
        visual_flags: 237,
      },
      health: 1.0,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 55);

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 55);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 55 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectBreakable::read::<SpawnByteOrder>(&mut reader)?,
      object
    );

    Ok(())
  }
}
