use crate::chunk::reader::ChunkReader;
use crate::data::particle::particle_action::particle_action_generic::ParticleActionGeneric;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use xray_ltx::Ltx;

/// Generic trait describing possibility to read object data from chunk.
pub trait ParticleActionReader<T: ParticleActionGeneric = Self> {
  /// Read particle action data from chunk reader.
  fn read<B: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<T>;

  /// Import particle action data from generic ini properties section.
  fn import(section_name: &str, ini: &Ltx) -> DatabaseResult<T>;
}
