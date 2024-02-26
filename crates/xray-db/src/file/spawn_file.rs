use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::file::alife_spawns_chunk::ALifeSpawnsChunk;
use crate::file::artefact_spawns_chunk::ArtefactSpawnsChunk;
use crate::file::graphs_chunk::GraphsChunk;
use crate::file::header_chunk::HeaderChunk;
use crate::file::patrols_chunk::PatrolsChunk;
use byteorder::ByteOrder;
use fileslice::FileSlice;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::PathBuf;

/// Descriptor of generic spawn file used by xray game engine.
///
/// Root level chunks by ID:
/// 0 - header
/// 1 - alife spawns
/// 2 - alife objects
/// 3 - patrols
/// 4 - game graphs
///
#[derive(Debug)]
pub struct SpawnFile {
  pub header: HeaderChunk,
  pub alife_spawn: ALifeSpawnsChunk,
  pub artefact_spawn: ArtefactSpawnsChunk,
  pub patrols: PatrolsChunk,
  pub graphs: GraphsChunk,
}

impl SpawnFile {
  /// Read spawn file from provided path.
  pub fn read_from_path<T: ByteOrder>(path: &PathBuf) -> io::Result<SpawnFile> {
    Self::read_from_file::<T>(File::open(path)?)
  }

  /// Read spawn file from file.
  pub fn read_from_file<T: ByteOrder>(file: File) -> io::Result<SpawnFile> {
    let mut root_chunk: Chunk = Chunk::from_file(FileSlice::new(file)).unwrap();

    let chunks: Vec<Chunk> = Chunk::read_all_from_file(&mut root_chunk);

    assert_eq!(
      chunks.len(),
      5,
      "Unexpected chunks count in spawn file root, expected 5."
    );

    let header_chunk: Chunk = chunks.get(0).expect("Header chunk to exist.").clone();
    let alife_chunk: Chunk = chunks.get(1).expect("Alife chunk to exist.").clone();
    let artefacts_chunk: Chunk = chunks.get(2).expect("Artefacts chunk to exist.").clone();
    let patrols_chunk: Chunk = chunks.get(3).expect("Patrols chunk to exist.").clone();
    let graphs_chunk: Chunk = chunks.get(4).expect("Graphs chunk to exist.").clone();

    let header: HeaderChunk =
      HeaderChunk::read_from_chunk::<T>(header_chunk).expect("Header chunk to be read.");

    let alife_spawn: ALifeSpawnsChunk =
      ALifeSpawnsChunk::read_from_chunk::<T>(alife_chunk).expect("Alife spawns chunk to be read.");

    let artefact_spawn: ArtefactSpawnsChunk =
      ArtefactSpawnsChunk::read_from_chunk::<T>(artefacts_chunk)
        .expect("Artefact spawns chunk to exist.");

    let patrols: PatrolsChunk =
      PatrolsChunk::read_from_chunk::<T>(patrols_chunk).expect("Patrols chunk to be read.");

    let graphs: GraphsChunk =
      GraphsChunk::read_from_chunk::<T>(graphs_chunk).expect("Level chunk to be read");

    assert!(root_chunk.is_ended(), "Expected spawn file to be ended.");

    Ok(SpawnFile {
      header,
      alife_spawn,
      artefact_spawn,
      patrols,
      graphs,
    })
  }

  /// Write spawn file data to the file by provided path.
  pub fn write_to_path<T: ByteOrder>(&self, path: &PathBuf) -> io::Result<()> {
    std::fs::create_dir_all(path.parent().expect("Parent directory"))?;

    let mut file: File = match OpenOptions::new()
      .create(true)
      .write(true)
      .truncate(true)
      .open(path.clone())
    {
      Ok(file) => Ok(file),
      Err(error) => Err(io::Error::new(
        error.kind(),
        format!("Failed to open file for all-spawn creation {:?}", path),
      )),
    }?;

    self.write_to_file::<T>(&mut file)
  }

  /// Write spawn file data to the file.
  pub fn write_to_file<T: ByteOrder>(&self, file: &mut File) -> io::Result<()> {
    let mut header_writer: ChunkWriter = ChunkWriter::new();
    let mut alife_writer: ChunkWriter = ChunkWriter::new();
    let mut artefacts_writer: ChunkWriter = ChunkWriter::new();
    let mut patrols_writer: ChunkWriter = ChunkWriter::new();
    let mut graphs_writer: ChunkWriter = ChunkWriter::new();

    self.header.write::<T>(&mut header_writer)?;
    self.alife_spawn.write::<T>(&mut alife_writer)?;
    self.artefact_spawn.write::<T>(&mut artefacts_writer)?;
    self.patrols.write::<T>(&mut patrols_writer)?;
    self.graphs.write::<T>(&mut graphs_writer)?;

    header_writer.flush_chunk_into_file::<T>(file, 0)?;
    alife_writer.flush_chunk_into_file::<T>(file, 1)?;
    artefacts_writer.flush_chunk_into_file::<T>(file, 2)?;
    patrols_writer.flush_chunk_into_file::<T>(file, 3)?;
    graphs_writer.flush_chunk_into_file::<T>(file, 4)?;

    Ok(())
  }
}
