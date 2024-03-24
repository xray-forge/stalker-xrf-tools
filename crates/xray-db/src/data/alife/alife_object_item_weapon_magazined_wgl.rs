use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::data::alife::alife_object_item_weapon_magazined::AlifeObjectItemWeaponMagazined;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io;
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectItemWeaponMagazinedWgl {
  pub base: AlifeObjectItemWeaponMagazined,
}

impl AlifeObjectInheritedReader<AlifeObjectItemWeaponMagazinedWgl>
  for AlifeObjectItemWeaponMagazinedWgl
{
  /// Read magazined weapon with launcher from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectItemWeaponMagazinedWgl> {
    Ok(AlifeObjectItemWeaponMagazinedWgl {
      base: AlifeObjectItemWeaponMagazined::read::<T>(reader)?,
    })
  }

  /// Read magazined weapon with launcher from ini config section.
  fn import(section: &Section) -> io::Result<AlifeObjectItemWeaponMagazinedWgl> {
    Ok(AlifeObjectItemWeaponMagazinedWgl {
      base: AlifeObjectItemWeaponMagazined::import(section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectItemWeaponMagazinedWgl {
  /// Write magazined weapon with launcher data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) {
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
  use crate::data::alife::alife_object_item_weapon_magazined::AlifeObjectItemWeaponMagazined;
  use crate::data::alife::alife_object_item_weapon_magazined_wgl::AlifeObjectItemWeaponMagazinedWgl;
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
      get_relative_test_sample_file_path(file!(), "alife_object_item_weapon_magazined_wgl.chunk");

    let object: AlifeObjectItemWeaponMagazinedWgl = AlifeObjectItemWeaponMagazinedWgl {
      base: AlifeObjectItemWeaponMagazined {
        base: AlifeObjectItemWeapon {
          base: AlifeObjectItem {
            base: AlifeObjectDynamicVisual {
              base: AlifeObjectAbstract {
                game_vertex_id: 2413,
                distance: 53.1213,
                direct_control: 3643,
                level_vertex_id: 25313,
                flags: 32,
                custom_data: String::from("custom-data"),
                story_id: 3486,
                spawn_story_id: 37663,
              },
              visual_name: String::from("visual-name"),
              visual_flags: 36,
            },
            condition: 1.0,
            upgrades_count: 0,
          },
          ammo_current: 20,
          ammo_elapsed: 10,
          weapon_state: 1,
          addon_flags: 1,
          ammo_type: 1,
          elapsed_grenades: 0,
        },
      },
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 67);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 67);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 67 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectItemWeaponMagazinedWgl =
      AlifeObjectItemWeaponMagazinedWgl::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
