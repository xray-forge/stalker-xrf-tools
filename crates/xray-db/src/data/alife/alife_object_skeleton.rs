use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::constants::FLAG_SKELETON_SAVED_DATA;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectSkeleton {
  pub name: String,
  pub flags: u8,
  pub source_id: u16,
}

impl AlifeObjectInheritedReader<AlifeObjectSkeleton> for AlifeObjectSkeleton {
  /// Read skeleton data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectSkeleton> {
    let name: String = chunk.read_null_terminated_string()?;
    let flags: u8 = chunk.read_u8()?;
    let source_id: u16 = chunk.read_u16::<SpawnByteOrder>()?;

    if flags & FLAG_SKELETON_SAVED_DATA != 0 {
      todo!("Extend skeleton parsing to include bones.")
    }

    Ok(AlifeObjectSkeleton {
      name,
      flags,
      source_id,
    })
  }

  /// Write skeleton data into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_null_terminated_string(&self.name)?;
    writer.write_u8(self.flags)?;
    writer.write_u16::<T>(self.source_id)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
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
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_object_skeleton.chunk"));

    let object: AlifeObjectSkeleton = AlifeObjectSkeleton {
      name: String::from("test-name"),
      flags: 33,
      source_id: 753,
    };

    object.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 13);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 13);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 13 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectSkeleton =
      AlifeObjectSkeleton::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
