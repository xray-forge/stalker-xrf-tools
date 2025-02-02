use std::path::Path;
use xray_error::XRayResult;
use xray_ltx::Ltx;

pub trait FileImportExport: Sized {
  fn import<P: AsRef<Path>>(path: &P) -> XRayResult<Self>;

  fn export<P: AsRef<Path>>(&self, path: &P) -> XRayResult;
}

pub trait LtxImportExport: Sized {
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self>;

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult;
}
