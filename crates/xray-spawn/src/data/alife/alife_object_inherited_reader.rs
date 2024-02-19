use crate::chunk::chunk::Chunk;

pub trait AlifeObjectInheritedReader<T> {
  fn from_chunk(chunk: &mut Chunk) -> T;

  fn verify(chunk: &Chunk) {
    assert_eq!(
      chunk.read_bytes_remain(),
      0,
      "Expected all data to be read from chunk."
    );
  }
}

pub trait AlifeObjectGeneric {}
