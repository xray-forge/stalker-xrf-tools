use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::export::file_import::read_ini_field;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectDynamicVisual {
  pub base: AlifeObjectAbstract,
  pub visual_name: String,
  pub visual_flags: u8,
}

impl AlifeObjectInheritedReader<AlifeObjectDynamicVisual> for AlifeObjectDynamicVisual {
  /// Read visual object data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectDynamicVisual> {
    Ok(AlifeObjectDynamicVisual {
      base: AlifeObjectAbstract::read_from_chunk::<T>(chunk)?,
      visual_name: chunk.read_null_terminated_win_string()?,
      visual_flags: chunk.read_u8()?,
    })
  }

  /// Import visual object data from ini config section.
  fn import(props: &Properties) -> io::Result<AlifeObjectDynamicVisual> {
    Ok(AlifeObjectDynamicVisual {
      base: AlifeObjectAbstract::import(props)?,
      visual_name: read_ini_field("visual_name", props)?,
      visual_flags: read_ini_field("visual_flags", props)?,
    })
  }
}

impl AlifeObjectGeneric for AlifeObjectDynamicVisual {
  type Order = SpawnByteOrder;

  /// Write visual alife object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_null_terminated_win_string(&self.visual_name)?;
    writer.write_u8(self.visual_flags)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ini) {
    self.base.export(section, ini);

    ini
      .with_section(Some(section))
      .set("visual_name", &self.visual_name)
      .set("visual_flags", self.visual_flags.to_string());
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
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_test_chunk_file_sub_dir(file!(), "alife_object_dynamic_visual.chunk");

    let object: AlifeObjectDynamicVisual = AlifeObjectDynamicVisual {
      base: AlifeObjectAbstract {
        game_vertex_id: 1001,
        distance: 65.25,
        direct_control: 412421,
        level_vertex_id: 66231,
        flags: 33,
        custom_data: String::from("custom_data"),
        story_id: 400,
        spawn_story_id: 25,
      },
      visual_name: String::from("visual-name"),
      visual_flags: 33,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 51);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 51);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 51 + 8);

    let mut chunk: Chunk = Chunk::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectDynamicVisual =
      AlifeObjectDynamicVisual::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
