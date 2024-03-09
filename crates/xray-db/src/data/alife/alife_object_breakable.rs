use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::export::file_import::read_ini_field;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use xray_ltx::{Ltx, Section};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeObjectBreakable {
  #[serde(rename = "base")]
  pub base: AlifeObjectDynamicVisual,
  #[serde(rename = "health")]
  pub health: f32,
}

impl AlifeObjectInheritedReader<AlifeObjectBreakable> for AlifeObjectBreakable {
  /// Read alife breakable object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectBreakable> {
    Ok(AlifeObjectBreakable {
      base: AlifeObjectDynamicVisual::read::<T>(reader)?,
      health: reader.read_f32::<SpawnByteOrder>()?,
    })
  }

  /// Import alife breakable object data from ini config section.
  fn import(section: &Section) -> io::Result<AlifeObjectBreakable> {
    Ok(AlifeObjectBreakable {
      base: AlifeObjectDynamicVisual::import(section)?,
      health: read_ini_field("health", section)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectBreakable {
  /// Write alife breakable object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_f32::<SpawnByteOrder>(self.health)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ltx) {
    self.base.export(section, ini);

    ini
      .with_section(section)
      .set("health", self.health.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_breakable::AlifeObjectBreakable;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::test::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_relative_test_sample_file_path(file!(), "alife_object_breakable.chunk");

    let object: AlifeObjectBreakable = AlifeObjectBreakable {
      base: AlifeObjectDynamicVisual {
        base: AlifeObjectAbstract {
          game_vertex_id: 3200,
          distance: 50.0,
          direct_control: 6219,
          level_vertex_id: 239805,
          flags: 562,
          custom_data: String::from("custom-data"),
          story_id: 92378,
          spawn_story_id: 235067,
        },
        visual_name: String::from("visual-name"),
        visual_flags: 237,
      },
      health: 1.0,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 55);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 55);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 55 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectBreakable =
      AlifeObjectBreakable::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
