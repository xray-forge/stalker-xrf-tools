use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::artefact_spawn_point::ArtefactSpawnPoint;
use crate::export::file::{create_export_file, open_ini_config};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{fmt, io};
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
  pub fn read<T: ByteOrder>(mut reader: ChunkReader) -> io::Result<SpawnArtefactSpawnsChunk> {
    let mut nodes: Vec<ArtefactSpawnPoint> = Vec::new();
    let count: u32 = reader.read_u32::<T>()?;

    // Parsing CLevelPoint structure, 20 bytes per one.
    for _ in 0..count {
      nodes.push(ArtefactSpawnPoint::read::<T>(&mut reader)?);
    }

    assert_eq!(nodes.len() as u64, count as u64);

    assert!(
      reader.is_ended(),
      "Expect artefact spawns chunk to be ended"
    );

    log::info!(
      "Parsed artefacts spawns: {:?} bytes",
      reader.read_bytes_len(),
    );

    Ok(SpawnArtefactSpawnsChunk { nodes })
  }

  /// Write artefact spawns into chunk writer.
  /// Writes artefact spawns data in binary format.
  pub fn write<T: ByteOrder>(&self, mut writer: ChunkWriter) -> io::Result<ChunkWriter> {
    writer.write_u32::<T>(self.nodes.len() as u32)?;

    for node in &self.nodes {
      node.write::<T>(&mut writer)?;
    }

    log::info!(
      "Written artefact spawns chunk, {:?} bytes",
      writer.bytes_written()
    );

    Ok(writer)
  }

  /// Import artefact spawns data from provided path.
  /// Parse ini files and populate spawn file.
  pub fn import(path: &Path) -> io::Result<SpawnArtefactSpawnsChunk> {
    let config: Ltx = open_ini_config(&path.join("artefact_spawns.ltx"))?;
    let mut nodes: Vec<ArtefactSpawnPoint> = Vec::new();

    for (_, props) in &config {
      nodes.push(ArtefactSpawnPoint::import(props)?);
    }

    log::info!("Imported artefact spawns chunk");

    Ok(SpawnArtefactSpawnsChunk { nodes })
  }

  /// Export artefact spawns data into provided path.
  pub fn export<T: ByteOrder>(&self, path: &Path) -> io::Result<()> {
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
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::artefact_spawn_point::ArtefactSpawnPoint;
  use crate::data::vector_3d::Vector3d;
  use crate::spawn_file::spawn_artefact_spawns_chunk::SpawnArtefactSpawnsChunk;
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write_artefact_spawn_point() -> io::Result<()> {
    let filename: String = get_relative_test_sample_file_path(file!(), "artefact_spawns.chunk");

    let spawns: SpawnArtefactSpawnsChunk = SpawnArtefactSpawnsChunk {
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

    let mut writer: ChunkWriter = spawns.write::<SpawnByteOrder>(ChunkWriter::new())?;

    assert_eq!(writer.bytes_written(), 44);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename).unwrap(),
      0,
    )?;

    assert_eq!(bytes_written, 44);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 44 + 8);

    let reader: ChunkReader = ChunkReader::from_slice(file)
      .unwrap()
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_spawns: SpawnArtefactSpawnsChunk =
      SpawnArtefactSpawnsChunk::read::<SpawnByteOrder>(reader)?;

    assert_eq!(read_spawns, spawns);

    Ok(())
  }
}
