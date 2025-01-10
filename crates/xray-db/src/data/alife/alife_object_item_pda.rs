use crate::data::alife::alife_object_item::AlifeObjectItem;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::error::DatabaseError;
use crate::export::file_import::read_ltx_field;
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectItemPda {
  pub base: AlifeObjectItem,
  pub owner: u16,
  pub character: String,
  pub info_portion: String,
}

impl AlifeObjectReader for AlifeObjectItemPda {
  /// Read pda object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    Ok(Self {
      base: AlifeObjectItem::read::<T>(reader)?,
      owner: reader.read_u16::<XRayByteOrder>()?,
      character: reader.read_null_terminated_win_string()?,
      info_portion: reader.read_null_terminated_win_string()?,
    })
  }

  /// Import pda object data from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      DatabaseError::new_parse_error(format!(
        "ALife object '{section_name}' should be defined in ltx file ({})",
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectItem::import(section_name, ltx)?,
      owner: read_ltx_field("owner", section)?,
      character: read_ltx_field("character", section)?,
      info_portion: read_ltx_field("info_portion", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeObjectItemPda {
  /// Write item data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    self.base.write(writer)?;

    writer.write_u16::<XRayByteOrder>(self.owner)?;
    writer.write_null_terminated_win_string(&self.character)?;
    writer.write_null_terminated_win_string(&self.info_portion)?;

    Ok(())
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> DatabaseResult {
    self.base.export(section_name, ltx)?;

    ltx
      .with_section(section_name)
      .set("owner", self.owner.to_string())
      .set("character", &self.character)
      .set("info_portion", &self.info_portion);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_item::AlifeObjectItem;
  use crate::data::alife::alife_object_item_pda::AlifeObjectItemPda;
  use crate::data::meta::alife_object_generic::AlifeObjectWriter;
  use crate::data::meta::alife_object_reader::AlifeObjectReader;
  use crate::types::DatabaseResult;
  use fileslice::FileSlice;
  use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: AlifeObjectItemPda = AlifeObjectItemPda {
      base: AlifeObjectItem {
        base: AlifeObjectDynamicVisual {
          base: AlifeObjectAbstract {
            game_vertex_id: 1011,
            distance: 35.2511,
            direct_control: 835,
            level_vertex_id: 3381,
            flags: 45,
            custom_data: String::from("custom_data"),
            story_id: 513,
            spawn_story_id: 35,
          },
          visual_name: String::from("zxcvb"),
          visual_flags: 43,
        },
        condition: 0.15,
        upgrades_count: 0,
      },
      owner: 0,
      character: String::from("character"),
      info_portion: String::from("info-portion"),
    };

    original.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 78);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 78);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 78 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;

    assert_eq!(
      AlifeObjectItemPda::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
