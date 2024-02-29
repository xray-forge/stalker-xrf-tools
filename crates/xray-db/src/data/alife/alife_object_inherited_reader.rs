use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use byteorder::ByteOrder;
use ini::Properties;
use std::io;

/// Generic trait describing possibility to read object data from chunk.
pub trait AlifeObjectInheritedReader<T: AlifeObjectGeneric> {
  /// Read alife object data from chunk reader.
  fn read_from_chunk<B: ByteOrder>(chunk: &mut Chunk) -> io::Result<T>;

  /// Import alife object data from generic ini properties section.
  fn import(props: &Properties) -> io::Result<T>;

  fn verify(chunk: &Chunk) {
    assert!(
      chunk.is_ended(),
      "Expected inherited alife object data chunk to be ended"
    );
  }
}
