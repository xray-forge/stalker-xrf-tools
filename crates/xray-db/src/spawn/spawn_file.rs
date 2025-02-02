use crate::export::FileImportExport;
use crate::spawn::chunks::spawn_alife_spawns_chunk::SpawnALifeSpawnsChunk;
use crate::spawn::chunks::spawn_artefact_spawns_chunk::SpawnArtefactSpawnsChunk;
use crate::spawn::chunks::spawn_graphs_chunk::SpawnGraphsChunk;
use crate::spawn::chunks::spawn_header_chunk::SpawnHeaderChunk;
use crate::spawn::chunks::spawn_patrols_chunk::SpawnPatrolsChunk;
use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use xray_chunk::{find_required_chunk_by_id, ChunkReader, ChunkWriter};
use xray_error::XRayResult;
use xray_utils::{assert_equal, open_export_file};

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
  pub fn read_from_path<T: ByteOrder, P: AsRef<Path>>(path: &P) -> XRayResult<Self> {
    Self::read_from_file::<T>(File::open(path)?)
  }

  /// Read spawn file from file.
  pub fn read_from_file<T: ByteOrder>(file: File) -> XRayResult<Self> {
    Self::read_from_chunks::<T>(&ChunkReader::from_file(file)?.read_children())
  }

  /// Read spawn file from chunks.
  pub fn read_from_chunks<T: ByteOrder>(chunks: &[ChunkReader]) -> XRayResult<Self> {
    assert_equal(
      chunks.len(),
      5,
      "Unexpected chunks count in spawn file root, expected 5",
    )?;

    let spawn_file: Self = {
      Self {
        header: find_required_chunk_by_id(chunks, SpawnHeaderChunk::CHUNK_ID)?.read_xr::<T, _>()?,
        alife_spawn: find_required_chunk_by_id(chunks, SpawnALifeSpawnsChunk::CHUNK_ID)?
          .read_xr::<T, _>()?,
        artefact_spawn: find_required_chunk_by_id(chunks, SpawnArtefactSpawnsChunk::CHUNK_ID)?
          .read_xr::<T, _>()?,
        patrols: find_required_chunk_by_id(chunks, SpawnPatrolsChunk::CHUNK_ID)?
          .read_xr::<T, _>()?,
        graphs: find_required_chunk_by_id(chunks, SpawnGraphsChunk::CHUNK_ID)?.read_xr::<T, _>()?,
      }
    };

    assert_equal(
      spawn_file.header.objects_count,
      spawn_file.alife_spawn.objects.len() as u32,
      "Expected correct objects count",
    )?;

    assert_equal(
      spawn_file.header.levels_count,
      spawn_file.graphs.header.levels_count as u32,
      "Expected correct levels count",
    )?;

    Ok(spawn_file)
  }

  /// Write spawn file data to the file by provided path.
  pub fn write_to_path<T: ByteOrder, P: AsRef<Path>>(&self, path: &P) -> XRayResult {
    fs::create_dir_all(path.as_ref().parent().expect("Spawn file parent directory"))?;
    self.write_to::<T>(&mut open_export_file(path)?)
  }

  /// Write spawn file data to the writer.
  pub fn write_to<T: ByteOrder>(&self, writer: &mut dyn Write) -> XRayResult {
    let mut header_chunk_writer: ChunkWriter = ChunkWriter::new();
    header_chunk_writer.write_xr::<T, _>(&self.header)?;
    header_chunk_writer.flush_chunk_into::<T>(writer, SpawnHeaderChunk::CHUNK_ID)?;

    let mut alife_spawn_chunk_writer: ChunkWriter = ChunkWriter::new();
    alife_spawn_chunk_writer.write_xr::<T, _>(&self.alife_spawn)?;
    alife_spawn_chunk_writer.flush_chunk_into::<T>(writer, SpawnALifeSpawnsChunk::CHUNK_ID)?;

    let mut artefact_spawn_chunk_writer: ChunkWriter = ChunkWriter::new();
    artefact_spawn_chunk_writer.write_xr::<T, _>(&self.artefact_spawn)?;
    artefact_spawn_chunk_writer
      .flush_chunk_into::<T>(writer, SpawnArtefactSpawnsChunk::CHUNK_ID)?;

    let mut patrols_chunk_writer: ChunkWriter = ChunkWriter::new();
    patrols_chunk_writer.write_xr::<T, _>(&self.patrols)?;
    patrols_chunk_writer.flush_chunk_into::<T>(writer, SpawnPatrolsChunk::CHUNK_ID)?;

    let mut graphs_chunk_writer: ChunkWriter = ChunkWriter::new();
    graphs_chunk_writer.write_xr::<T, _>(&self.graphs)?;
    graphs_chunk_writer.flush_chunk_into::<T>(writer, SpawnGraphsChunk::CHUNK_ID)?;

    Ok(())
  }

  /// Read spawn file from provided path.
  pub fn import_from_path<T: ByteOrder, P: AsRef<Path>>(path: &P) -> XRayResult<Self> {
    Ok(Self {
      header: SpawnHeaderChunk::import(path)?,
      alife_spawn: SpawnALifeSpawnsChunk::import(path)?,
      artefact_spawn: SpawnArtefactSpawnsChunk::import(path)?,
      patrols: SpawnPatrolsChunk::import(path)?,
      graphs: SpawnGraphsChunk::import(path)?,
    })
  }

  /// Export unpacked ALife spawn file into provided path.
  pub fn export_to_path<T: ByteOrder, P: AsRef<Path>>(&self, path: &P) -> XRayResult {
    fs::create_dir_all(path)?;

    self.header.export(path)?;
    self.alife_spawn.export(path)?;
    self.artefact_spawn.export(path)?;
    self.patrols.export(path)?;
    self.graphs.export(path)?;

    Ok(())
  }
}
