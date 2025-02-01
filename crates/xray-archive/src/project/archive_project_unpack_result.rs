use serde::Serialize;

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
