use crate::chunk::chunk::Chunk;
use byteorder::ByteOrder;
use std::io;

pub trait AlifeObjectInheritedReader<T> {
  fn read_from_chunk<B: ByteOrder>(chunk: &mut Chunk) -> io::Result<T>;

  fn verify(chunk: &Chunk) {
    assert!(
      chunk.is_ended(),
      "Expected inherited alife object data chunk to be ended."
    );
  }
}

pub trait AlifeObjectGeneric {}
