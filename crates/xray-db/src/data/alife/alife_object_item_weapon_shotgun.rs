use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_item_weapon::AlifeObjectItemWeapon;
use byteorder::ByteOrder;
use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeObjectItemWeaponShotgun {
  pub base: AlifeObjectItemWeapon,
}

impl AlifeObjectInheritedReader<AlifeObjectItemWeaponShotgun> for AlifeObjectItemWeaponShotgun {
  /// Read shotgun object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectItemWeaponShotgun> {
    Ok(AlifeObjectItemWeaponShotgun {
      base: AlifeObjectItemWeapon::read::<T>(reader)?,
    })
  }

  /// Import alife object data from ini config section.
  fn import(props: &Properties) -> io::Result<AlifeObjectItemWeaponShotgun> {
    Ok(AlifeObjectItemWeaponShotgun {
      base: AlifeObjectItemWeapon::import(props)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectItemWeaponShotgun {
  /// Write shotgun object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ini) {
    self.base.export(section, ini);
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
  use crate::data::alife::alife_object_item_weapon_shotgun::AlifeObjectItemWeaponShotgun;
  use crate::test::utils::{
    get_test_sample_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_test_sample_file_sub_dir(file!(), "alife_object_item_weapon_shotgun.chunk");

    let object: AlifeObjectItemWeaponShotgun = AlifeObjectItemWeaponShotgun {
      base: AlifeObjectItemWeapon {
        base: AlifeObjectItem {
          base: AlifeObjectDynamicVisual {
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

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 67);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 67);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 67 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectItemWeaponShotgun =
      AlifeObjectItemWeaponShotgun::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
