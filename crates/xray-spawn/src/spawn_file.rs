use crate::alife_spawns_chunk::ALifeObjectsChunk;
use crate::artefact_spawns_chunk::ArtefactSpawnsChunk;
use crate::chunk::chunk::Chunk;
use crate::graphs_chunk::GraphsChunk;
use crate::header_chunk::HeaderChunk;
use crate::patrols_chunk::PatrolsChunk;
use byteorder::ByteOrder;
use fileslice::FileSlice;
use std::fs::File;
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
  pub size: u64,
  pub header: HeaderChunk,
  pub alife_spawn: ALifeObjectsChunk,
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
    let size: u64 = file.metadata()?.len();
    let mut root_chunk: Chunk = Chunk::from_file(FileSlice::new(file)).unwrap();

    log::info!("Parsing spawn file: 0 -> {:?}", size);

    let chunks: Vec<Chunk> = Chunk::read_all_from_file(&mut root_chunk);

    assert_eq!(
      chunks.len(),
      5,
      "Unexpected chunks count in spawn file root, expected 5."
    );

    let header: HeaderChunk =
      HeaderChunk::read_from_chunk::<T>(chunks.get(0).expect("Header chunk to exist.").clone())
        .expect("Header chunk to be read.");

    let alife_spawn: ALifeObjectsChunk = ALifeObjectsChunk::read_from_chunk::<T>(
      chunks.get(1).expect("Alife spawns chunk to exist.").clone(),
    )
    .expect("Alife spawns chunk to be read.");

    let artefact_spawn: ArtefactSpawnsChunk = ArtefactSpawnsChunk::read_from_chunk::<T>(
      chunks
        .get(2)
        .expect("Artefact spawns chunk to exist.")
        .clone(),
    )
    .expect("Artefact spawns chunk to exist.");

    let patrols: PatrolsChunk =
      PatrolsChunk::read_from_chunk::<T>(chunks.get(3).expect("Patrol chunk to exist.").clone())
        .expect("Patrols chunk to be read.");

    let graphs: GraphsChunk =
      GraphsChunk::read_from_chunk::<T>(chunks.get(4).expect("Level chunk to exist.").clone())
        .expect("Level chunk to be read");

    assert!(root_chunk.is_ended(), "Expected whole file to be read.");

    Ok(SpawnFile {
      size,
      header,
      alife_spawn,
      artefact_spawn,
      patrols,
      graphs,
    })
  }
}
