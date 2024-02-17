use crate::spawn::alife_spawns_chunk::ALifeObjectsChunk;
use crate::spawn::artefact_spawns_chunk::ArtefactSpawnsChunk;
use crate::spawn::chunk::Chunk;
use crate::spawn::graphs_chunk::GraphsChunk;
use crate::spawn::header_chunk::HeaderChunk;
use crate::spawn::patrols_chunk::PatrolsChunk;
use fileslice::FileSlice;
use std::fs::File;
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
  pub chunks: Vec<Chunk>,
  pub header: HeaderChunk,
  pub alife_spawn: ALifeObjectsChunk,
  pub artefact_spawn: ArtefactSpawnsChunk,
  pub patrols: PatrolsChunk,
  pub graphs: GraphsChunk,
}

impl SpawnFile {
  pub fn from_path(path: &PathBuf) -> Result<SpawnFile, String> {
    let file: File = File::open(path).unwrap();
    let size: u64 = file.metadata().unwrap().len();
    let mut file: FileSlice = FileSlice::new(file);

    log::info!(
      "Parsing spawn file: {:?}, 0 -> {:?}",
      path.as_path(),
      file.end_pos()
    );

    let chunks: Vec<Chunk> = Chunk::read_all_children(&mut file);

    assert_eq!(
      chunks.len(),
      5,
      "Unexpected chunks count in spawn file root, expected 5."
    );

    let header: Option<HeaderChunk> = match chunks.get(0) {
      Some(chunk) => HeaderChunk::from_chunk(chunk.clone()),
      None => None,
    };

    let alife_spawns: Option<ALifeObjectsChunk> = match chunks.get(1) {
      Some(chunk) => ALifeObjectsChunk::from_chunk(chunk.clone()),
      None => None,
    };

    let artefact_spawns: Option<ArtefactSpawnsChunk> = match chunks.get(2) {
      Some(chunk) => ArtefactSpawnsChunk::from_chunk(chunk.clone()),
      None => None,
    };

    let patrols: Option<PatrolsChunk> = match chunks.get(3) {
      Some(chunk) => PatrolsChunk::from_chunk(chunk.clone()),
      None => None,
    };

    let graphs: Option<GraphsChunk> = match chunks.get(4) {
      Some(chunk) => GraphsChunk::from_chunk(chunk.clone()),
      None => None,
    };

    Ok(SpawnFile {
      size,
      chunks: chunks,
      header: header.expect("Unexpected header signature in spawn file."),
      alife_spawn: alife_spawns.expect("Unexpected alife spawns signature in spawn file."),
      artefact_spawn: artefact_spawns.expect("Unexpected artefact spawns signature in spawn file."),
      patrols: patrols.expect("Unexpected patrols signature in spawn file."),
      graphs: graphs.expect("Unexpected graphs signature in spawn file."),
    })
  }
}
