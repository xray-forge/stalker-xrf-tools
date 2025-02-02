use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::data::meta::alife_object_reader::AlifeObjectReader;
use crate::file_import::read_ltx_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use xray_chunk::{ChunkReader, ChunkWriter};
use xray_error::{XRayError, XRayResult};
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlifeObjectInventoryBox {
  pub base: AlifeObjectDynamicVisual,
  pub can_take: u8,
  pub is_closed: u8,
  pub tip: String,
}

impl AlifeObjectReader for AlifeObjectInventoryBox {
  /// Read inventory object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    Ok(Self {
      base: AlifeObjectDynamicVisual::read::<T>(reader)?,
      can_take: reader.read_u8()?,
      is_closed: reader.read_u8()?,
      tip: reader.read_w1251_string()?,
    })
  }

  /// Import alife inventory box object from ltx config section.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self> {
    let section: &Section = ltx.section(section_name).ok_or_else(|| {
      XRayError::new_parsing_error(format!(
        "ALife object '{}' should be defined in ltx file ({})",
        section_name,
        file!()
      ))
    })?;

    Ok(Self {
      base: AlifeObjectDynamicVisual::import(section_name, ltx)?,
      can_take: read_ltx_field("can_take", section)?,
      is_closed: read_ltx_field("is_closed", section)?,
      tip: read_ltx_field("tip", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectWriter for AlifeObjectInventoryBox {
  /// Write inventory object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> XRayResult {
    self.base.write(writer)?;

    writer.write_u8(self.can_take)?;
    writer.write_u8(self.is_closed)?;
    writer.write_w1251_string(&self.tip)?;

    Ok(())
  }

  /// Export object data into ltx file.
  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult {
    self.base.export(section_name, ltx)?;

    ltx
      .with_section(section_name)
      .set("can_take", self.can_take.to_string())
      .set("is_closed", self.is_closed.to_string())
      .set("tip", &self.tip);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_inventory_box::AlifeObjectInventoryBox;
  use crate::data::meta::alife_object_generic::AlifeObjectWriter;
  use crate::data::meta::alife_object_reader::AlifeObjectReader;
  use xray_chunk::{ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };
  use xray_test_utils::FileSlice;

  #[test]
  fn test_read_write() -> XRayResult {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: AlifeObjectInventoryBox = AlifeObjectInventoryBox {
      base: AlifeObjectDynamicVisual {
        base: AlifeObjectAbstract {
          game_vertex_id: 2463,
          distance: 12.0,
          direct_control: 5634,
          level_vertex_id: 2533,
          flags: 64,
          custom_data: String::from("custom-data"),
          story_id: 2136,
          spawn_story_id: 0,
        },
        visual_name: String::from("visual-name"),
        visual_flags: 0,
      },
      can_take: 0,
      is_closed: 1,
      tip: String::from("some-tip"),
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
      AlifeObjectInventoryBox::read::<XRayByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
