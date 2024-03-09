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
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeObjectItemWeapon {
  #[serde(rename = "base")]
  pub base: AlifeObjectItem,
  #[serde(rename = "ammoCurrent")]
  pub ammo_current: u16,
  #[serde(rename = "ammoElapsed")]
  pub ammo_elapsed: u16,
  #[serde(rename = "weaponState")]
  pub weapon_state: u8,
  #[serde(rename = "addonFlags")]
  pub addon_flags: u8,
  #[serde(rename = "ammoType")]
  pub ammo_type: u8,
  #[serde(rename = "elapsedGrenades")]
  pub elapsed_grenades: u8,
}

impl AlifeObjectInheritedReader<AlifeObjectItemWeapon> for AlifeObjectItemWeapon {
  /// Read alife item object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectItemWeapon> {
    Ok(AlifeObjectItemWeapon {
      base: AlifeObjectItem::read::<T>(reader)?,
      ammo_current: reader.read_u16::<SpawnByteOrder>()?,
      ammo_elapsed: reader.read_u16::<SpawnByteOrder>()?,
      weapon_state: reader.read_u8()?,
      addon_flags: reader.read_u8()?,
      ammo_type: reader.read_u8()?,
      elapsed_grenades: reader.read_u8()?,
    })
  }

  /// Import alife weapon item object data from ini config section.
  fn import(section: &Section) -> io::Result<AlifeObjectItemWeapon> {
    Ok(AlifeObjectItemWeapon {
      base: AlifeObjectItem::import(section)?,
      ammo_current: read_ini_field("ammo_current", section)?,
      ammo_elapsed: read_ini_field("ammo_elapsed", section)?,
      weapon_state: read_ini_field("weapon_state", section)?,
      addon_flags: read_ini_field("addon_flags", section)?,
      ammo_type: read_ini_field("ammo_type", section)?,
      elapsed_grenades: read_ini_field("elapsed_grenades", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectItemWeapon {
  /// Write item data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_u16::<SpawnByteOrder>(self.ammo_current)?;
    writer.write_u16::<SpawnByteOrder>(self.ammo_elapsed)?;
    writer.write_u8(self.weapon_state)?;
    writer.write_u8(self.addon_flags)?;
    writer.write_u8(self.ammo_type)?;
    writer.write_u8(self.elapsed_grenades)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) {
    self.base.export(section, ini);

    ini
      .with_section(section)
      .set("ammo_current", self.ammo_current.to_string())
      .set("ammo_elapsed", self.ammo_elapsed.to_string())
      .set("weapon_state", self.weapon_state.to_string())
      .set("addon_flags", self.addon_flags.to_string())
      .set("ammo_type", self.ammo_type.to_string())
      .set("elapsed_grenades", self.elapsed_grenades.to_string());
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
  use crate::data::alife::alife_object_item_weapon::AlifeObjectItemWeapon;
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
    let filename: String =
      get_relative_test_sample_file_path(file!(), "alife_object_item_weapon.chunk");

    let object: AlifeObjectItemWeapon = AlifeObjectItemWeapon {
      base: AlifeObjectItem {
        base: AlifeObjectDynamicVisual {
          base: AlifeObjectAbstract {
            game_vertex_id: 1011,
            distance: 234.2511,
            direct_control: 623354,
            level_vertex_id: 455313,
            flags: 43,
            custom_data: String::from("custom_data"),
            story_id: 512,
            spawn_story_id: 31,
          },
          visual_name: String::from("asdfgh"),
          visual_flags: 33,
        },
        condition: 0.84,
        upgrades_count: 0,
      },
      ammo_current: 25,
      ammo_elapsed: 5,
      weapon_state: 1,
      addon_flags: 0,
      ammo_type: 1,
      elapsed_grenades: 0,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 62);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 62);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 62 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectItemWeapon =
      AlifeObjectItemWeapon::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
