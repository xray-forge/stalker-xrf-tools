use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_item_weapon::AlifeObjectItemWeapon;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use xray_ltx::Ltx;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectItemWeaponShotgun {
  pub base: AlifeObjectItemWeapon,
}

impl AlifeObjectReader<AlifeObjectItemWeaponShotgun> for AlifeObjectItemWeaponShotgun {
  /// Read shotgun object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      base: AlifeObjectItemWeapon::read::<T>(reader)?,
    })
  }

  /// Import alife object data from ini config section.
  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<Self> {
    Ok(Self {
      base: AlifeObjectItemWeapon::import(section_name, ini)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeObjectItemWeaponShotgun {
  /// Write shotgun object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    self.base.write(writer)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult {
    self.base.export(section, ini)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_item::AlifeObjectItem;
  use crate::data::alife::alife_object_item_weapon::AlifeObjectItemWeapon;
  use crate::data::alife::alife_object_item_weapon_shotgun::AlifeObjectItemWeaponShotgun;
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

    let original: AlifeObjectItemWeaponShotgun = AlifeObjectItemWeaponShotgun {
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

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 67);

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 67);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 67 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectItemWeaponShotgun::read::<SpawnByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
