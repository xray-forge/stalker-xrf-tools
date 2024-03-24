use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::export::file_import::read_ini_field;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectItem {
  pub base: AlifeObjectDynamicVisual,
  pub condition: f32,
  pub upgrades_count: u32,
}

impl AlifeObjectInheritedReader<AlifeObjectItem> for AlifeObjectItem {
  /// Read alife item object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectItem> {
    let object: AlifeObjectItem = AlifeObjectItem {
      base: AlifeObjectDynamicVisual::read::<T>(reader)?,
      condition: reader.read_f32::<SpawnByteOrder>()?,
      upgrades_count: reader.read_u32::<SpawnByteOrder>()?,
    };

    assert_eq!(
      object.upgrades_count, 0,
      "Unexpected upgraded item provided"
    );

    Ok(object)
  }

  /// Import alife item object data from ini config section.
  fn import(section: &Section) -> io::Result<AlifeObjectItem> {
    Ok(AlifeObjectItem {
      base: AlifeObjectDynamicVisual::import(section)?,
      condition: read_ini_field("condition", section)?,
      upgrades_count: read_ini_field("upgrades_count", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectItem {
  /// Write item data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_f32::<SpawnByteOrder>(self.condition)?;
    writer.write_u32::<SpawnByteOrder>(self.upgrades_count)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) {
    self.base.export(section, ini);

    ini
      .with_section(section)
      .set("condition", self.condition.to_string())
      .set("is_closed", self.upgrades_count.to_string())
      .set("upgrades_count", self.upgrades_count.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_item::AlifeObjectItem;
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "alife_object_item.chunk");

    let object: AlifeObjectItem = AlifeObjectItem {
      base: AlifeObjectDynamicVisual {
        base: AlifeObjectAbstract {
          game_vertex_id: 1002,
          distance: 65.25,
          direct_control: 41262,
          level_vertex_id: 618923,
          flags: 32,
          custom_data: String::from("custom_data"),
          story_id: 500,
          spawn_story_id: 35,
        },
        visual_name: String::from("abcd"),
        visual_flags: 33,
      },
      condition: 0.5,
      upgrades_count: 0,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 52);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 52);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 52 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectItem = AlifeObjectItem::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
