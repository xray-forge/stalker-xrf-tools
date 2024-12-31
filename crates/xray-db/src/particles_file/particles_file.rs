use crate::chunk::reader::ChunkReader;
use crate::chunk::utils::find_chunk_by_id;
use crate::chunk::writer::ChunkWriter;
use crate::export::file::create_export_file;
use crate::particles_file::chunks::particles_effects_chunk::ParticlesEffectsChunk;
use crate::particles_file::chunks::particles_firstgen_chunk::ParticlesFirstgenChunk;
use crate::particles_file::chunks::particles_groups_chunk::ParticlesGroupsChunk;
use crate::particles_file::chunks::particles_header_chunk::ParticlesHeaderChunk;
use crate::spawn_file::chunks::spawn_alife_spawns_chunk::SpawnALifeSpawnsChunk;
use crate::spawn_file::chunks::spawn_artefact_spawns_chunk::SpawnArtefactSpawnsChunk;
use crate::spawn_file::chunks::spawn_graphs_chunk::SpawnGraphsChunk;
use crate::spawn_file::chunks::spawn_header_chunk::SpawnHeaderChunk;
use crate::spawn_file::chunks::spawn_patrols_chunk::SpawnPatrolsChunk;
use crate::spawn_file::spawn_file::SpawnFile;
use crate::types::DatabaseResult;
use byteorder::ByteOrder;
use fileslice::FileSlice;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticlesFile {
  pub header: ParticlesHeaderChunk,
  pub effects: ParticlesEffectsChunk,
  pub groups: ParticlesGroupsChunk,
}

impl ParticlesFile {
  /// Read particles xr file from provided path.
  pub fn read_from_path<T: ByteOrder>(path: &Path) -> DatabaseResult<ParticlesFile> {
    Self::read_from_file::<T>(File::open(path)?)
  }

  /// Read particles xr from file.
  pub fn read_from_file<T: ByteOrder>(file: File) -> DatabaseResult<ParticlesFile> {
    let mut reader: ChunkReader = ChunkReader::from_slice(FileSlice::new(file))?;
    let chunks: Vec<ChunkReader> = ChunkReader::read_all_from_file(&mut reader);
    let chunk_ids: Vec<u32> = chunks.iter().map(|it| it.id).collect();

    log::info!(
      "Parsed particles file meta, {} chunks, {} bytes, {:?} chunks",
      chunks.len(),
      reader.read_bytes_len(),
      chunk_ids
    );

    assert!(
      !chunk_ids.contains(&ParticlesFirstgenChunk::CHUNK_ID),
      "Unexpected first-gen chunk in particles file, unpacking not implemented"
    );

    Ok(ParticlesFile {
      header: ParticlesHeaderChunk::read::<T>(
        &mut find_chunk_by_id(&chunks, ParticlesHeaderChunk::CHUNK_ID)
          .expect("Particle version chunk not found"),
      )?,
      effects: ParticlesEffectsChunk::read::<T>(
        &mut find_chunk_by_id(&chunks, ParticlesEffectsChunk::CHUNK_ID)
          .expect("Particle effects chunk not found"),
      )?,
      groups: ParticlesGroupsChunk::read::<T>(
        &mut find_chunk_by_id(&chunks, ParticlesGroupsChunk::CHUNK_ID)
          .expect("Particle groups chunk not found"),
      )?,
    })
  }

  /// Write particles file data to the file by provided path.
  pub fn write_to_path<T: ByteOrder>(&self, path: &Path) -> DatabaseResult<()> {
    fs::create_dir_all(path.parent().expect("Parent directory"))?;
    self.write_to_file::<T>(&mut create_export_file(path)?)
  }

  /// Write particles file data to the file.
  pub fn write_to_file<T: ByteOrder>(&self, file: &mut File) -> DatabaseResult<()> {
    let mut header_chunk_writer: ChunkWriter = ChunkWriter::new();
    let mut effects_chunk_writer: ChunkWriter = ChunkWriter::new();
    let mut group_chunk_writer: ChunkWriter = ChunkWriter::new();

    self.header.write::<T>(&mut header_chunk_writer)?;
    self.effects.write::<T>(&mut effects_chunk_writer)?;
    self.groups.write::<T>(&mut group_chunk_writer)?;

    header_chunk_writer.flush_chunk_into_file::<T>(file, ParticlesHeaderChunk::CHUNK_ID)?;
    effects_chunk_writer.flush_chunk_into_file::<T>(file, ParticlesEffectsChunk::CHUNK_ID)?;
    group_chunk_writer.flush_chunk_into_file::<T>(file, ParticlesGroupsChunk::CHUNK_ID)?;

    Ok(())
  }

  /// Read spawn file from provided path.
  pub fn import_from_path(path: &Path) -> DatabaseResult<ParticlesFile> {
    Ok(ParticlesFile {
      header: ParticlesHeaderChunk::import(path)?,
      effects: ParticlesEffectsChunk::import(path)?,
      groups: ParticlesGroupsChunk::import(path)?,
    })
  }

  /// Export unpacked alife spawn file into provided path.
  pub fn export_to_path(&self, path: &Path) -> DatabaseResult<()> {
    fs::create_dir_all(path)?;

    self.header.export(path)?;
    self.effects.export(path)?;
    self.groups.export(path)?;

    Ok(())
  }
}
