use crate::chunk::reader::ChunkReader;
use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use xray_ltx::Section;

/// Generic trait describing possibility to read object data from chunk.
pub trait AlifeObjectReader<T: AlifeObjectWriter = Self> {
  /// Read alife object data from chunk reader.
  fn read<B: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<T>;

  /// Import alife object data from generic ini properties section.
  fn import(section: &Section) -> DatabaseResult<T>;

  fn verify(reader: &ChunkReader) {
    assert!(
      reader.is_ended(),
      "Expected inherited alife object data chunk to be ended"
    );
  }
}
