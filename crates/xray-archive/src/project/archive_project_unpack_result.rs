use serde::Serialize;

#[cfg_attr(
  feature = "typescript-bindings",
  derive(ts_rs::TS),
  ts(export, export_to = "xray-archive.ts")
)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveUnpackResult {
  pub archives: Vec<String>,
  pub duration: u128,
  pub destination: String,
  pub prepare_duration: u128,
  pub unpacked_size: u64,
  pub unpack_duration: u128,
}
