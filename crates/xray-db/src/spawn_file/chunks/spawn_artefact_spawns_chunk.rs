use crate::data::artefact_spawn::artefact_spawn_point::ArtefactSpawnPoint;
use crate::export::file::{create_export_file, open_ltx_config};
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::Path;
use xray_chunk::{ChunkReader, ChunkWriter};
use xray_ltx::Ltx;

/// Artefacts spawns samples.
/// Is single plain chunk with nodes list in it.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnArtefactSpawnsChunk {
  pub nodes: Vec<ArtefactSpawnPoint>,
}

impl SpawnArtefactSpawnsChunk {
  pub const CHUNK_ID: u32 = 2;

  /// Read header chunk by position descriptor.
  /// Parses binary data into artefact spawns chunk representation object.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    log::info!(
      "Reading artefacts spawns chunk: {:?} bytes",
      reader.read_bytes_remain()
    );

    let mut nodes: Vec<ArtefactSpawnPoint> = Vec::new();
    let count: u32 = reader.read_u32::<T>()?;

    // Parsing CLevelPoint structure, 20 bytes per one.
    for _ in 0..count {
      nodes.push(ArtefactSpawnPoint::read::<T>(reader)?);
    }

    assert_eq!(nodes.len() as u64, count as u64);

    assert!(
      reader.is_ended(),
      "Expect artefact spawns chunk to be ended, {} remain",
      reader.read_bytes_remain()
    );

    Ok(Self { nodes })
  }

  /// Write artefact spawns into chunk writer.
  /// Writes artefact spawns data in binary format.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    writer.write_u32::<T>(self.nodes.len() as u32)?;

    for node in &self.nodes {
      node.write::<T>(writer)?;
    }

    log::info!(
      "Written artefact spawns chunk, {:?} bytes",
      writer.bytes_written()
    );

    Ok(())
  }

  /// Import artefact spawns data from provided path.
  /// Parse ltx files and populate spawn file.
  pub fn import(path: &Path) -> DatabaseResult<Self> {
    let ltx: Ltx = open_ltx_config(&path.join("artefact_spawns.ltx"))?;
    let mut nodes: Vec<ArtefactSpawnPoint> = Vec::new();

    for (_, props) in &ltx {
      nodes.push(ArtefactSpawnPoint::import(props)?);
    }

    log::info!("Imported artefact spawns chunk");

    Ok(Self { nodes })
  }

  /// Export artefact spawns data into provided path.
  pub fn export(&self, path: &Path) -> DatabaseResult {
    let mut ltx: Ltx = Ltx::new();

    for (index, node) in self.nodes.iter().enumerate() {
      node.export(&index.to_string(), &mut ltx);
    }

    ltx.write_to(&mut create_export_file(&path.join("artefact_spawns.ltx"))?)?;

    log::info!("Exported artefact spawns chunk");

    Ok(())
  }
}

impl fmt::Debug for SpawnArtefactSpawnsChunk {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "ArtefactSpawnsChunk {{ nodes: Vector[{}] }}",
      self.nodes.len()
    )
  }
}

#[cfg(test)]
mod tests {
  use crate::data::artefact_spawn::artefact_spawn_point::ArtefactSpawnPoint;
  use crate::data::generic::vector_3d::Vector3d;
  use crate::spawn_file::chunks::spawn_artefact_spawns_chunk::SpawnArtefactSpawnsChunk;
  use crate::types::{DatabaseResult, SpawnByteOrder};
  use fileslice::FileSlice;
  use xray_chunk::{ChunkReader, ChunkWriter};
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write() -> DatabaseResult {
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: SpawnArtefactSpawnsChunk = SpawnArtefactSpawnsChunk {
      nodes: vec![
        ArtefactSpawnPoint {
          position: Vector3d::new(55.5, 44.4, -33.3),
          level_vertex_id: 255,
          distance: 450.30,
        },
        ArtefactSpawnPoint {
          position: Vector3d::new(-21.0, 13.5, -4.0),
          level_vertex_id: 13,
          distance: 25.11,
        },
      ],
    };

    let mut writer: ChunkWriter = ChunkWriter::new();

    original.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 44);

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename).unwrap(),
      0,
    )?;

    assert_eq!(bytes_written, 44);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 44 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    assert_eq!(
      SpawnArtefactSpawnsChunk::read::<SpawnByteOrder>(&mut reader)?,
      original
    );

    Ok(())
  }
}
