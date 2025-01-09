use crate::data::meta::alife_object_generic::AlifeObjectWriter;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use xray_chunk::ChunkReader;
use xray_ltx::Ltx;

/// Generic trait describing possibility to read object data from chunk.
pub trait AlifeObjectReader<T: AlifeObjectWriter = Self> {
  /// Read alife object data from chunk reader.
  fn read<B: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<T>;

  /// Import alife object data from generic ltx properties section.
  fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<T>;

  fn verify(reader: &ChunkReader) {
    assert!(
      reader.is_ended(),
      "Expected inherited alife object data chunk to be ended"
    );
  }
}
