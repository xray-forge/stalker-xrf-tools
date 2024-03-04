use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeObjectInventoryBox {
  pub base: AlifeObjectDynamicVisual,
  pub can_take: u8,
  pub is_closed: u8,
  pub tip: String,
}

impl AlifeObjectInheritedReader<AlifeObjectInventoryBox> for AlifeObjectInventoryBox {
  /// Read inventory object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectInventoryBox> {
    Ok(AlifeObjectInventoryBox {
      base: AlifeObjectDynamicVisual::read::<T>(reader)?,
      can_take: reader.read_u8()?,
      is_closed: reader.read_u8()?,
      tip: reader.read_null_terminated_win_string()?,
    })
  }

  /// Import alife inventory box object from ini config section.
  fn import(props: &Properties) -> io::Result<AlifeObjectInventoryBox> {
    Ok(AlifeObjectInventoryBox {
      base: AlifeObjectDynamicVisual::import(props)?,
      can_take: read_ini_field("can_take", props)?,
      is_closed: read_ini_field("is_closed", props)?,
      tip: read_ini_field("tip", props)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectInventoryBox {
  /// Write inventory object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_u8(self.can_take)?;
    writer.write_u8(self.is_closed)?;
    writer.write_null_terminated_win_string(&self.tip)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ini) {
    self.base.export(section, ini);

    ini
      .with_section(Some(section))
      .set("can_take", self.can_take.to_string())
      .set("is_closed", self.is_closed.to_string())
      .set("tip", &self.tip);
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
  use crate::data::alife::alife_object_inventory_box::AlifeObjectInventoryBox;
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
      get_test_sample_file_sub_dir(file!(), "alife_object_inventory_box.chunk");

    let object: AlifeObjectInventoryBox = AlifeObjectInventoryBox {
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

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 62);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 62);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 62 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectInventoryBox =
      AlifeObjectInventoryBox::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
