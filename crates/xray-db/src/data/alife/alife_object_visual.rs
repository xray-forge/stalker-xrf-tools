use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlifeObjectVisual {
  pub visual_name: String,
  pub visual_flags: u8,
}

impl AlifeObjectInheritedReader<AlifeObjectVisual> for AlifeObjectVisual {
  /// Read visual object data from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<AlifeObjectVisual> {
    Ok(AlifeObjectVisual {
      visual_name: reader.read_null_terminated_win_string()?,
      visual_flags: reader.read_u8()?,
    })
  }

  /// Import visual object data from ini config section.
  fn import(props: &Properties) -> io::Result<AlifeObjectVisual> {
    Ok(AlifeObjectVisual {
      visual_name: read_ini_field("visual_name", props)?,
      visual_flags: read_ini_field("visual_flags", props)?,
    })
  }
}

#[typetag::serde]
impl AlifeObjectGeneric for AlifeObjectVisual {
  /// Write visual alife object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_null_terminated_win_string(&self.visual_name)?;
    writer.write_u8(self.visual_flags)?;

    Ok(())
  }

  /// Export object data into ini file.
  fn export(&self, section: &str, ini: &mut Ini) {
    ini
      .with_section(Some(section))
      .set("visual_name", &self.visual_name)
      .set("visual_flags", self.visual_flags.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_visual::AlifeObjectVisual;
  use crate::export::file::{export_ini_to_file, open_ini_config};
  use crate::test::assertions::files_are_equal_by_path;
  use crate::test::utils::{
    get_absolute_test_resource_path, get_relative_test_sample_file_path,
    open_test_resource_as_slice, overwrite_test_relative_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use ini::Ini;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_relative_test_sample_file_path(file!(), "alife_object_visual.chunk");

    let object: AlifeObjectVisual = AlifeObjectVisual {
      visual_name: String::from("visual-name"),
      visual_flags: 33,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 13);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 13);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 13 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectVisual = AlifeObjectVisual::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_object, object);

    Ok(())
  }

  #[test]
  fn test_import_export_object() -> io::Result<()> {
    let first: AlifeObjectVisual = AlifeObjectVisual {
      visual_name: String::from("visual-name-example"),
      visual_flags: 33,
    };

    let second: AlifeObjectVisual = AlifeObjectVisual {
      visual_name: String::from(""),
      visual_flags: 33,
    };

    let exported_filename: String = get_relative_test_sample_file_path(file!(), "exported.ini");
    let mut exported: Ini = Ini::new();

    first.export("first", &mut exported);
    second.export("second", &mut exported);

    export_ini_to_file(
      &exported,
      &mut overwrite_test_relative_resource_as_file(&exported_filename)?,
    )?;

    let source: Ini = open_ini_config(&get_absolute_test_resource_path(&exported_filename))?;

    let read_first: AlifeObjectVisual =
      AlifeObjectVisual::import(source.section(Some("first")).unwrap())?;
    let read_second: AlifeObjectVisual =
      AlifeObjectVisual::import(source.section(Some("second")).unwrap())?;

    assert_eq!(read_first, first);
    assert_eq!(read_second, second);

    let imported_filename: String = get_relative_test_sample_file_path(file!(), "imported.ini");
    let mut imported: Ini = Ini::new();

    read_first.export("first", &mut imported);
    read_second.export("second", &mut imported);

    export_ini_to_file(
      &imported,
      &mut overwrite_test_relative_resource_as_file(&imported_filename)?,
    )?;

    assert!(files_are_equal_by_path(
      get_absolute_test_resource_path(&exported_filename),
      get_absolute_test_resource_path(&imported_filename)
    )?);

    Ok(())
  }
}
