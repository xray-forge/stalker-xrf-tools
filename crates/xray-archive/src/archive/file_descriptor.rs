use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Clone, Serialize)]
pub struct ArchiveFileDescriptor {
  #[serde(rename = "crc")]
  pub crc: u32,
  #[serde(rename = "name")]
  pub name: String,
  #[serde(rename = "offset")]
  pub offset: u32,
  #[serde(rename = "sizeCompressed")]
  pub size_compressed: u32,
  #[serde(rename = "sizeReal")]
  pub size_real: u32,
}

#[derive(Clone, Serialize)]
pub struct ArchiveFileReplicationDescriptor {
  #[serde(rename = "crc")]
  pub crc: u32,
  #[serde(rename = "source")]
  pub source: PathBuf,
  #[serde(rename = "destination")]
  pub destination: PathBuf,
  #[serde(rename = "name")]
  pub name: String,
  #[serde(rename = "offset")]
  pub offset: u32,
  #[serde(rename = "sizeCompressed")]
  pub size_compressed: u32,
  #[serde(rename = "sizeReal")]
  pub size_real: u32,
}

impl ArchiveFileReplicationDescriptor {
  pub fn from_descriptor(
    descriptor: &ArchiveFileDescriptor,
    source: &Path,
    destination: &Path,
  ) -> ArchiveFileReplicationDescriptor {
    ArchiveFileReplicationDescriptor {
      crc: descriptor.crc,
      source: source.into(),
      destination: destination.into(),
      name: descriptor.name.clone(),
      offset: descriptor.offset,
      size_compressed: descriptor.size_compressed,
      size_real: descriptor.size_real,
    }
  }
}
