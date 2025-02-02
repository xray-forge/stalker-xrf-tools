use crate::constants::META_TYPE_FIELD;
use crate::data::particles::particle_group::ParticleGroup;
use crate::export::FileImportExport;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;
use xray_ltx::Ltx;
use xray_utils::{assert, open_export_file};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticlesGroupsChunk {
  pub groups: Vec<ParticleGroup>,
}

impl ParticlesGroupsChunk {
  pub const CHUNK_ID: u32 = 4;
}

impl ChunkReadWrite for ParticlesGroupsChunk {
  /// Read effects chunk by position descriptor.
  /// Parses binary data into version chunk representation object.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    let chunks: Vec<ChunkReader> = reader.read_children();
    let mut groups: Vec<ParticleGroup> = Vec::new();

    log::info!(
      "Parsed groups chunk, {} bytes, {} chunks",
      reader.read_bytes_len(),
      chunks.len()
    );

    for mut chunk in chunks {
      groups.push(ParticleGroup::read::<T>(&mut chunk)?);
    }

    groups.sort_by(|first, second| first.name.cmp(&second.name));

    assert(reader.is_ended(), "Expect groups chunk to be ended")?;

    Ok(Self { groups })
  }

  /// Write particle groups data into chunk writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    for (index, group) in self.groups.iter().enumerate() {
      let mut group_writer: ChunkWriter = ChunkWriter::new();

      group.write::<T>(&mut group_writer)?;

      writer.write_all(
        group_writer
          .flush_chunk_into_buffer::<T>(index as u32)?
          .as_slice(),
      )?;
    }

    log::info!("Written groups chunk, {} bytes", writer.bytes_written());

    Ok(())
  }
}

impl FileImportExport for ParticlesGroupsChunk {
  /// Import particles groups data from provided path.
  fn import<P: AsRef<Path>>(path: &P) -> XRayResult<Self> {
    log::info!("Importing particles groups: {}", path.as_ref().display());

    let ltx: Ltx = Ltx::read_from_path(path.as_ref().join("groups.ltx"))?;
    let mut groups: Vec<ParticleGroup> = Vec::new();

    for (section_name, section) in &ltx {
      if let Some(meta_field) = section.get(META_TYPE_FIELD) {
        if meta_field == ParticleGroup::META_TYPE {
          groups.push(ParticleGroup::import(section_name, &ltx)?);
        }
      }
    }

    groups.sort_by(|first, second| first.name.cmp(&second.name));

    Ok(Self { groups })
  }

  /// Export particles groups data into provided path.
  fn export<P: AsRef<Path>>(&self, path: &P) -> XRayResult {
    let mut particles_effects_ltx: Ltx = Ltx::new();

    for group in &self.groups {
      group.export(&group.name, &mut particles_effects_ltx)?;
    }

    particles_effects_ltx.write_to(&mut open_export_file(path.as_ref().join("groups.ltx"))?)?;

    log::info!("Exported groups chunk");

    Ok(())
  }
}
