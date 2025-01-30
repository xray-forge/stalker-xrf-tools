use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::export::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectItemWeapon {
  pub base: AlifeObjectItem,
  pub ammo_current: u16,
  pub ammo_elapsed: u16,
  pub weapon_state: u8,
  pub addon_flags: u8,
  pub ammo_type: u8,
  pub elapsed_grenades: u8,
}

impl AlifeObjectReader for AlifeObjectItemWeapon {
  /// Read alife item object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      base: AlifeObjectItem::read::<T>(reader)?,
      ammo_current: reader.read_u16::<XRayByteOrder>()?,
      ammo_elapsed: reader.read_u16::<XRayByteOrder>()?,
      weapon_state: reader.read_u8()?,
      addon_flags: reader.read_u8()?,
      ammo_type: reader.read_u8()?,
      elapsed_grenades: reader.read_u8()?,
    })
  }

  /// Import alife weapon item object data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "ALife object '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectItem::import(section_name, ltx)?,
      ammo_current: read_ltx_field("ammo_current", section)?,
      ammo_elapsed: read_ltx_field("ammo_elapsed", section)?,
      weapon_state: read_ltx_field("weapon_state", section)?,
      addon_flags: read_ltx_field("addon_flags", section)?,
      ammo_type: read_ltx_field("ammo_type", section)?,
      elapsed_grenades: read_ltx_field("elapsed_grenades", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeObjectItemWeapon {
  /// Write item data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> XRayResult {
    self.base.write(writer)?;

    writer.write_u16::<XRayByteOrder>(self.ammo_current)?;
    writer.write_u16::<XRayByteOrder>(self.ammo_elapsed)?;
    writer.write_u8(self.weapon_state)?;
    writer.write_u8(self.addon_flags)?;
    writer.write_u8(self.ammo_type)?;
    writer.write_u8(self.elapsed_grenades)?;

    Ok(())
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    self.base.export(section_name, ltx)?;

    ltx
      .with_section(section_name)
      .set("ammo_current", self.ammo_current.to_string())
      .set("ammo_elapsed", self.ammo_elapsed.to_string())
      .set("weapon_state", self.weapon_state.to_string())
      .set("addon_flags", self.addon_flags.to_string())
      .set("ammo_type", self.ammo_type.to_string())
      .set("elapsed_grenades", self.elapsed_grenades.to_string());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_item::AlifeObjectItem;
  use crate::data::alife::alife_object_item_weapon::AlifeObjectItemWeapon;
  use crate::data::meta::alife_object_generic::AlifeObjectWriter;
  use crate::data::meta::alife_object_reader::AlifeObjectReader;
  use fileslice::FileSlice;
  use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: AlifeObjectItemWeapon = AlifeObjectItemWeapon {
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

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 62);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 62);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 62 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectItemWeapon::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
