use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::alife::alife_object_item_weapon::AlifeObjectItemWeapon;
use byteorder::ByteOrder;
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectItemWeaponShotgun {
  pub base: AlifeObjectItemWeapon,
}

impl AlifeObjectInheritedReader<AlifeObjectItemWeaponShotgun> for AlifeObjectItemWeaponShotgun {
  /// Read shotgun object data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectItemWeaponShotgun> {
    let base: AlifeObjectItemWeapon = AlifeObjectItemWeapon::read_from_chunk::<T>(chunk)?;

    Ok(AlifeObjectItemWeaponShotgun { base })
  }

  /// Write shotgun object data into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write::<T>(writer)?;

    Ok(())
  }
}

impl AlifeObjectGeneric for AlifeObjectItemWeaponShotgun {}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_item::AlifeObjectItem;
  use crate::data::alife::alife_object_item_weapon::AlifeObjectItemWeapon;
  use crate::data::alife::alife_object_item_weapon_shotgun::AlifeObjectItemWeaponShotgun;
  use crate::data::alife::alife_object_visual::AlifeObjectVisual;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_test_chunk_file_sub_dir(
      file!(),
      &String::from("alife_object_item_weapon_shotgun.chunk"),
    );

    let object: AlifeObjectItemWeaponShotgun = AlifeObjectItemWeaponShotgun {
      base: AlifeObjectItemWeapon {
        base: AlifeObjectItem {
          base: AlifeObjectVisual {
            base: AlifeObjectAbstract {
              game_vertex_id: 36426,
              distance: 54.132,
              direct_control: 441,
              level_vertex_id: 23513,
              flags: 33,
              custom_data: String::from("custom-data"),
              story_id: 35426,
              spawn_story_id: 267845,
            },
            visual_name: String::from("visual-name"),
            visual_flags: 253,
          },
          condition: 1.0,
          upgrades_count: 0,
        },
        ammo_current: 20,
        ammo_elapsed: 10,
        weapon_state: 3,
        addon_flags: 36,
        ammo_type: 1,
        elapsed_grenades: 1,
      },
    };

    object.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 67);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 67);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 67 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectItemWeaponShotgun =
      AlifeObjectItemWeaponShotgun::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}