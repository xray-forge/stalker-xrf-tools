use crate::spawn_file::chunks::spawn_alife_spawns_chunk::SpawnALifeSpawnsChunk;
use crate::spawn_file::chunks::spawn_artefact_spawns_chunk::SpawnArtefactSpawnsChunk;
use crate::spawn_file::chunks::spawn_graphs_chunk::SpawnGraphsChunk;
use crate::spawn_file::chunks::spawn_header_chunk::SpawnHeaderChunk;
use crate::spawn_file::chunks::spawn_patrols_chunk::SpawnPatrolsChunk;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use xray_chunk::{find_required_chunk_by_id, ChunkReader, ChunkWriter};
use xray_error::XRayResult;
use xray_utils::open_export_file;

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
  pub fn read_from_path<T: ByteOrder, P: AsRef<Path>>(path: P) -> XRayResult<Self> {
    Self::read_from_file::<T>(File::open(path)?)
  }

  /// Read spawn file from file.
  pub fn read_from_file<T: ByteOrder>(file: File) -> XRayResult<Self> {
    let mut reader: ChunkReader = ChunkReader::from_file(file)?;

    Self::read_from_chunks::<T>(&reader.read_children())
  }

  /// Read spawn file from chunks.
  pub fn read_from_chunks<T: ByteOrder>(chunks: &[ChunkReader]) -> XRayResult<Self> {
    assert_eq!(
      chunks.len(),
      5,
      "Unexpected chunks count in spawn file root, expected 5"
    );

    let spawn_file: Self = {
      Self {
        header: SpawnHeaderChunk::read::<T>(&mut find_required_chunk_by_id(
          chunks,
          SpawnHeaderChunk::CHUNK_ID,
        )?)?,
        alife_spawn: SpawnALifeSpawnsChunk::read::<T>(&mut find_required_chunk_by_id(
          chunks,
          SpawnALifeSpawnsChunk::CHUNK_ID,
        )?)?,
        artefact_spawn: SpawnArtefactSpawnsChunk::read::<T>(&mut find_required_chunk_by_id(
          chunks,
          SpawnArtefactSpawnsChunk::CHUNK_ID,
        )?)?,
        patrols: SpawnPatrolsChunk::read::<T>(&mut find_required_chunk_by_id(
          chunks,
          SpawnPatrolsChunk::CHUNK_ID,
        )?)?,
        graphs: SpawnGraphsChunk::read::<T>(&mut find_required_chunk_by_id(
          chunks,
          SpawnGraphsChunk::CHUNK_ID,
        )?)?,
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
  pub fn write_to_path<T: ByteOrder, P: AsRef<Path>>(&self, path: P) -> XRayResult {
    fs::create_dir_all(path.as_ref().parent().expect("Parent directory"))?;
    self.write_to::<T>(&mut open_export_file(path)?)
  }

  /// Write spawn file data to the writer.
  pub fn write_to<T: ByteOrder>(&self, writer: &mut dyn Write) -> XRayResult {
    let mut header_chunk_writer: ChunkWriter = ChunkWriter::new();
    self.header.write::<T>(&mut header_chunk_writer)?;
    header_chunk_writer.flush_chunk_into::<T>(writer, SpawnHeaderChunk::CHUNK_ID)?;

    let mut alife_spawn_chunk_writer: ChunkWriter = ChunkWriter::new();
    self.alife_spawn.write::<T>(&mut alife_spawn_chunk_writer)?;
    alife_spawn_chunk_writer.flush_chunk_into::<T>(writer, SpawnALifeSpawnsChunk::CHUNK_ID)?;

    let mut artefact_spawn_chunk_writer: ChunkWriter = ChunkWriter::new();
    self
      .artefact_spawn
      .write::<T>(&mut artefact_spawn_chunk_writer)?;
    artefact_spawn_chunk_writer
      .flush_chunk_into::<T>(writer, SpawnArtefactSpawnsChunk::CHUNK_ID)?;

    let mut patrols_chunk_writer: ChunkWriter = ChunkWriter::new();
    self.patrols.write::<T>(&mut patrols_chunk_writer)?;
    patrols_chunk_writer.flush_chunk_into::<T>(writer, SpawnPatrolsChunk::CHUNK_ID)?;

    let mut graphs_chunk_writer: ChunkWriter = ChunkWriter::new();
    self.graphs.write::<T>(&mut graphs_chunk_writer)?;
    graphs_chunk_writer.flush_chunk_into::<T>(writer, SpawnGraphsChunk::CHUNK_ID)?;

    Ok(())
  }

  /// Read spawn file from provided path.
  pub fn import_from_path<T: ByteOrder, P: AsRef<Path>>(path: P) -> XRayResult<Self> {
    Ok(Self {
      header: SpawnHeaderChunk::import(path.as_ref())?,
      alife_spawn: SpawnALifeSpawnsChunk::import(path.as_ref())?,
      artefact_spawn: SpawnArtefactSpawnsChunk::import(path.as_ref())?,
      patrols: SpawnPatrolsChunk::import(path.as_ref())?,
      graphs: SpawnGraphsChunk::import::<T, P>(path)?,
    })
  }

  /// Export unpacked ALife spawn file into provided path.
  pub fn export_to_path<T: ByteOrder, P: AsRef<Path>>(&self, path: P) -> XRayResult {
    fs::create_dir_all(path.as_ref())?;

    self.header.export(path.as_ref())?;
    self.alife_spawn.export(path.as_ref())?;
    self.artefact_spawn.export(path.as_ref())?;
    self.patrols.export(path.as_ref())?;
    self.graphs.export::<T, P>(path)?;

    Ok(())
  }
}
