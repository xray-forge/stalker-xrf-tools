use crate::chunk::chunk::Chunk;

pub trait AlifeObjectInheritedReader<T> {
  fn read_from_chunk(chunk: &mut Chunk) -> T;

  fn verify(chunk: &Chunk) {
    assert!(
      chunk.is_ended(),
      "Expected inherited alife object data chunk to be ended."
    );
  }
}

pub trait AlifeObjectGeneric {}
