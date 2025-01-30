use crate::data::meta::particle_action_writer::ParticleActionWriter;
use byteorder::ByteOrder;
use xray_chunk::ChunkReader;
use xray_error::XRayResult;
use xray_ltx::Ltx;

/// Generic trait describing possibility to read object data from chunk.
pub trait ParticleActionReader<T: ParticleActionWriter = Self> {
  /// Read particle action data from chunk reader.
  fn read<B: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<T>;

  /// Import particle action data from generic ltx properties section.
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<T>;
}
