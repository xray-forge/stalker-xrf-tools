use crate::chunk::reader::ChunkReader;
use crate::data::meta::particle_action_writer::ParticleActionWriter;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use xray_ltx::Ltx;

/// Generic trait describing possibility to read object data from chunk.
pub trait ParticleActionReader<T: ParticleActionWriter = Self> {
  /// Read particle action data from chunk reader.
  fn read<B: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<T>;

  /// Import particle action data from generic ltx properties section.
  fn import(section_name: &str, ltx: &Ltx) -> DatabaseResult<T>;
}
