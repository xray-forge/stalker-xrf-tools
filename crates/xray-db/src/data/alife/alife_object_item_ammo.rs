use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::export::file_import::read_ini_field;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use xray_ltx::{Ltx, Properties};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeObjectItemAmmo {
  #[serde(rename = "base")]
  pub base: AlifeObjectItem,
  #[serde(rename = "ammoLeft")]
  pub ammo_left: u16,
}

impl AlifeObjectInheritedReader<AlifeObjectItemAmmo> for AlifeObjectItemAmmo {
  /// Read alife item object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectItemAmmo> {
    Ok(AlifeObjectItemAmmo {
      base: AlifeObjectItem::read::<T>(reader)?,
      ammo_left: reader.read_u16::<SpawnByteOrder>()?,
    })
  }

  /// Import alife ammo item data from ini config section.
  fn import(props: &Properties) -> io::Result<AlifeObjectItemAmmo> {
    Ok(AlifeObjectItemAmmo {
      base: AlifeObjectItem::import(props)?,
      ammo_left: read_ini_field("ammo_left", props)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectItemAmmo {
  /// Write item data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_u16::<SpawnByteOrder>(self.ammo_left)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) {
    self.base.export(section, ini);

    ini
      .with_section(Some(section))
      .set("ammo_left", self.ammo_left.to_string());
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
  use crate::data::alife::alife_object_item_ammo::AlifeObjectItemAmmo;
  use crate::test::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_relative_test_sample_file_path(file!(), "alife_object_item_ammo.chunk");

    let object: AlifeObjectItemAmmo = AlifeObjectItemAmmo {
      base: AlifeObjectItem {
        base: AlifeObjectDynamicVisual {
          base: AlifeObjectAbstract {
            game_vertex_id: 1003,
            distance: 65.25,
            direct_control: 412242,
            level_vertex_id: 463752354,
            flags: 40,
            custom_data: String::from("custom_data"),
            story_id: 2563,
            spawn_story_id: 413,
          },
          visual_name: String::from("cdef"),
          visual_flags: 33,
        },
        condition: 0.6,
        upgrades_count: 0,
      },
      ammo_left: 12,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 54);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 54);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 54 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectItemAmmo =
      AlifeObjectItemAmmo::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
