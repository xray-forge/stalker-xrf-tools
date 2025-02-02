use std::path::Path;
use xray_error::XRayResult;
use xray_ltx::Ltx;

pub trait FileImportExport: Sized {
  fn import<P: AsRef<Path>>(path: &P) -> XRayResult<Self>;

  fn export<P: AsRef<Path>>(&self, path: &P) -> XRayResult;
}

pub trait LtxImportExport: Sized {
  fn import(section_name: &str, ltx: &Ltx) -> XRayResult<Self>;

  fn import_optional(section_name: &str, ltx: &Ltx) -> XRayResult<Option<Self>> {
    if ltx.has_section(section_name) {
      Self::import(section_name, ltx).map(Some)
    } else {
      Ok(None)
    }
  }

  fn export(&self, section_name: &str, ltx: &mut Ltx) -> XRayResult;

  fn export_optional(section_name: &str, ltx: &mut Ltx, data: Option<&Self>) -> XRayResult {
    if let Some(data) = data {
      data.export(section_name, ltx)
    } else {
      Ok(())
    }
  }
}
