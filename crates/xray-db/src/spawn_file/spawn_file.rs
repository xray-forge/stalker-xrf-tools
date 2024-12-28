use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::export::file::create_export_file;
use crate::spawn_file::spawn_alife_spawns_chunk::SpawnALifeSpawnsChunk;
use crate::spawn_file::spawn_artefact_spawns_chunk::SpawnArtefactSpawnsChunk;
use crate::spawn_file::spawn_graphs_chunk::SpawnGraphsChunk;
use crate::spawn_file::spawn_header_chunk::SpawnHeaderChunk;
use crate::spawn_file::spawn_patrols_chunk::SpawnPatrolsChunk;
use byteorder::ByteOrder;
use fileslice::FileSlice;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use std::{fs, io};

/// Descriptor of generic spawn file used by xray game engine.
///
/// Root level samples by ID:
/// 0 - header
/// 1 - alife spawns
/// 2 - alife objects
/// 3 - patrols
/// 4 - game graphs
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnFile {
  pub header: SpawnHeaderChunk,
  pub alife_spawn: SpawnALifeSpawnsChunk,
  pub artefact_spawn: SpawnArtefactSpawnsChunk,
  pub patrols: SpawnPatrolsChunk,
  pub graphs: SpawnGraphsChunk,
}

impl SpawnFile {
  /// Read spawn file from provided path.
  pub fn read_from_path<T: ByteOrder>(path: &Path) -> io::Result<SpawnFile> {
    Self::read_from_file::<T>(File::open(path)?)
  }

  /// Read spawn file from file.
  pub fn read_from_file<T: ByteOrder>(file: File) -> io::Result<SpawnFile> {
    let mut reader: ChunkReader = ChunkReader::from_slice(FileSlice::new(file))?;
    let chunks: Vec<ChunkReader> = ChunkReader::read_all_from_file(&mut reader);

    assert_eq!(
      chunks.len(),
      5,
      "Unexpected chunks count in spawn file root, expected 5"
    );

    let spawn_file: SpawnFile = {
      SpawnFile {
        header: SpawnHeaderChunk::read::<T>(
          chunks
            .get(SpawnHeaderChunk::CHUNK_ID as usize)
            .unwrap()
            .clone(),
        )?,
        alife_spawn: SpawnALifeSpawnsChunk::read::<T>(
          chunks
            .get(SpawnALifeSpawnsChunk::CHUNK_ID as usize)
            .unwrap()
            .clone(),
        )?,
        artefact_spawn: SpawnArtefactSpawnsChunk::read::<T>(
          chunks
            .get(SpawnArtefactSpawnsChunk::CHUNK_ID as usize)
            .unwrap()
            .clone(),
        )?,
        patrols: SpawnPatrolsChunk::read::<T>(
          chunks
            .get(SpawnPatrolsChunk::CHUNK_ID as usize)
            .unwrap()
            .clone(),
        )?,
        graphs: SpawnGraphsChunk::read::<T>(
          chunks
            .get(SpawnGraphsChunk::CHUNK_ID as usize)
            .unwrap()
            .clone(),
        )?,
      }
    };

    assert_eq!(
      spawn_file.header.objects_count,
      spawn_file.alife_spawn.objects.len() as u32,
      "Expected correct objects count"
    );
    assert_eq!(
      spawn_file.header.levels_count, spawn_file.graphs.header.levels_count as u32,
      "Expected correct level count"
    );

    Ok(spawn_file)
  }

  /// Write spawn file data to the file by provided path.
  pub fn write_to_path<T: ByteOrder>(&self, path: &Path) -> io::Result<()> {
    fs::create_dir_all(path.parent().expect("Parent directory"))?;
    self.write_to_file::<T>(&mut create_export_file(path)?)
  }

  /// Write spawn file data to the file.
  pub fn write_to_file<T: ByteOrder>(&self, file: &mut File) -> io::Result<()> {
    self
      .header
      .write::<T>(ChunkWriter::new())?
      .flush_chunk_into_file::<T>(file, 0)?;
    self
      .alife_spawn
      .write::<T>(ChunkWriter::new())?
      .flush_chunk_into_file::<T>(file, 1)?;
    self
      .artefact_spawn
      .write::<T>(ChunkWriter::new())?
      .flush_chunk_into_file::<T>(file, 2)?;
    self
      .patrols
      .write::<T>(ChunkWriter::new())?
      .flush_chunk_into_file::<T>(file, 3)?;
    self
      .graphs
      .write::<T>(ChunkWriter::new())?
      .flush_chunk_into_file::<T>(file, 4)?;

    Ok(())
  }

  /// Read spawn file from provided path.
  pub fn import_from_path<T: ByteOrder>(path: &Path) -> io::Result<SpawnFile> {
    Ok(SpawnFile {
      header: SpawnHeaderChunk::import(path)?,
      alife_spawn: SpawnALifeSpawnsChunk::import(path)?,
      artefact_spawn: SpawnArtefactSpawnsChunk::import(path)?,
      patrols: SpawnPatrolsChunk::import(path)?,
      graphs: SpawnGraphsChunk::import::<T>(path)?,
    })
  }

  /// Export unpacked alife spawn file into provided path.
  pub fn export_to_path<T: ByteOrder>(&self, path: &Path) -> io::Result<()> {
    fs::create_dir_all(path)?;

    self.header.export::<T>(path)?;
    self.alife_spawn.export::<T>(path)?;
    self.artefact_spawn.export::<T>(path)?;
    self.patrols.export::<T>(path)?;
    self.graphs.export::<T>(path)?;

    Ok(())
  }
}
