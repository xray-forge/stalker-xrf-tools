use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::Ini;
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectItemWeapon {
  pub base: AlifeObjectItem,
  pub ammo_current: u16,
  pub ammo_elapsed: u16,
  pub weapon_state: u8,
  pub addon_flags: u8,
  pub ammo_type: u8,
  pub elapsed_grenades: u8,
}

impl AlifeObjectInheritedReader<AlifeObjectItemWeapon> for AlifeObjectItemWeapon {
  /// Read alife item object data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectItemWeapon> {
    let base: AlifeObjectItem = AlifeObjectItem::read_from_chunk::<T>(chunk)?;

    let ammo_current: u16 = chunk.read_u16::<SpawnByteOrder>()?;
    let ammo_elapsed: u16 = chunk.read_u16::<SpawnByteOrder>()?;
    let weapon_state: u8 = chunk.read_u8()?;
    let addon_flags: u8 = chunk.read_u8()?;
    let ammo_type: u8 = chunk.read_u8()?;
    let elapsed_grenades: u8 = chunk.read_u8()?;

    Ok(AlifeObjectItemWeapon {
      base,
      ammo_current,
      ammo_elapsed,
      weapon_state,
      addon_flags,
      ammo_type,
      elapsed_grenades,
    })
  }
}

impl AlifeObjectGeneric for AlifeObjectItemWeapon {
  type Order = SpawnByteOrder;

  /// Write item data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_u16::<Self::Order>(self.ammo_current)?;
    writer.write_u16::<Self::Order>(self.ammo_elapsed)?;
    writer.write_u8(self.weapon_state)?;
    writer.write_u8(self.addon_flags)?;
    writer.write_u8(self.ammo_type)?;
    writer.write_u8(self.elapsed_grenades)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ini) {
    self.base.export(section, ini);

    ini
      .with_section(Some(section))
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
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_item::AlifeObjectItem;
  use crate::data::alife::alife_object_item_weapon::AlifeObjectItemWeapon;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_test_chunk_file_sub_dir(file!(), "alife_object_item_weapon.chunk");

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
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 62);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 62 + 8);

    let mut chunk: Chunk = Chunk::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectItemWeapon =
      AlifeObjectItemWeapon::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
