use serde::Serialize;

#[derive(Serialize)]
pub struct ArchiveUnpackResult {
  #[serde(rename = "archives")]
  pub archives: Vec<String>,
  #[serde(rename = "duration")]
  pub duration: u128,
  #[serde(rename = "destination")]
  pub destination: String,
  #[serde(rename = "prepareDuration")]
  pub prepare_duration: u128,
  #[serde(rename = "unpackedSize")]
  pub unpacked_size: u64,
  #[serde(rename = "unpackDuration")]
  pub unpack_duration: u128,
}
