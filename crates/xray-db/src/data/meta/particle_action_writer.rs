use crate::chunk::writer::ChunkWriter;
use crate::types::DatabaseResult;
use std::fmt::Debug;
use xray_ltx::Ltx;

#[typetag::serde(tag = "type")]
pub trait ParticleActionWriter: Debug + Send + Sync {
  fn write(&self, writer: &mut ChunkWriter) -> DatabaseResult;

  fn export(&self, section: &str, ini: &mut Ltx) -> DatabaseResult;
}
