use std::fmt::Debug;
use xray_chunk::ChunkWriter;
use xray_error::XRayResult;
use xray_ltx::Ltx;

#[typetag::serde(tag = "type")]
pub trait AlifeObjectWriter: Debug + Send + Sync {
  fn write(&self, writer: &mut ChunkWriter) -> XRayResult;

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult;
}
