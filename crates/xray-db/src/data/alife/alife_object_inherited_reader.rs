use crate::chunk::reader::ChunkReader;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use byteorder::ByteOrder;
use std::io;
use xray_ltx::Section;

/// Generic trait describing possibility to read object data from chunk.
pub trait AlifeObjectInheritedReader<T: AlifeObjectGeneric> {
  /// Read alife object data from chunk reader.
  fn read<B: ByteOrder>(reader: &mut ChunkReader) -> io::Result<T>;

  /// Import alife object data from generic ini properties section.
  fn import(section: &Section) -> io::Result<T>;

  fn verify(reader: &ChunkReader) {
    assert!(
      reader.is_ended(),
      "Expected inherited alife object data chunk to be ended"
    );
  }
}
