use crate::chunk::writer::ChunkWriter;
use std::fmt::Debug;
use std::io;
use xray_ltx::Ini;

#[typetag::serde(tag = "type")]
pub trait AlifeObjectGeneric: Debug + Send + Sync {
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()>;

  fn export(&self, section: &str, config: &mut Ini);
}
