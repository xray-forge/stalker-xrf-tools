use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectVisual {
  pub visual_name: String,
  pub visual_flags: u8,
}

impl AlifeObjectInheritedReader<AlifeObjectVisual> for AlifeObjectVisual {
  /// Read visual object data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectVisual> {
    let visual_name: String = chunk.read_null_terminated_string()?;
    let visual_flags: u8 = chunk.read_u8()?;

    Ok(AlifeObjectVisual {
      visual_name,
      visual_flags,
    })
  }
}

impl AlifeObjectGeneric for AlifeObjectVisual {
  type Order = SpawnByteOrder;

  /// Write visual alife object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_null_terminated_string(&self.visual_name)?;
    writer.write_u8(self.visual_flags)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_visual::AlifeObjectVisual;
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
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_object_visual.chunk"));

    let object: AlifeObjectVisual = AlifeObjectVisual {
      visual_name: String::from("visual-name"),
      visual_flags: 33,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 13);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 13);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 13 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectVisual =
      AlifeObjectVisual::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
