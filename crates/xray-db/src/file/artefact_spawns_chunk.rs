use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::artefact_spawn_point::ArtefactSpawnPoint;
use crate::export::file::{create_export_file, export_ini_to_file, open_ini_config};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::Ini;
use std::path::Path;
use std::{fmt, io};

/// Artefacts spawns chunks.
/// Is single plain chunk with nodes list in it.
#[derive(Clone, PartialEq)]
pub struct ArtefactSpawnsChunk {
  pub nodes: Vec<ArtefactSpawnPoint>,
}

impl ArtefactSpawnsChunk {
  /// Read header chunk by position descriptor.
  /// Parses binary data into artefact spawns chunk representation object.
  pub fn read_from_chunk<T: ByteOrder>(mut chunk: Chunk) -> io::Result<ArtefactSpawnsChunk> {
    let mut nodes: Vec<ArtefactSpawnPoint> = Vec::new();
    let count: u32 = chunk.read_u32::<T>()?;

    // Parsing CLevelPoint structure, 20 bytes per one.
    for _ in 0..count {
      nodes.push(ArtefactSpawnPoint::read_from_chunk::<T>(&mut chunk)?);
    }

    assert_eq!(nodes.len() as u64, count as u64);

    assert!(chunk.is_ended(), "Expect artefact spawns chunk to be ended");

    log::info!(
      "Parsed artefacts spawns: {:?} bytes",
      chunk.read_bytes_len(),
    );

    Ok(ArtefactSpawnsChunk { nodes })
  }

  /// Write artefact spawns into chunk writer.
  /// Writes artefact spawns data in binary format.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
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
  /// Parse ini files and populate spawn file.
  pub fn import(path: &Path) -> io::Result<ArtefactSpawnsChunk> {
    let config: Ini = open_ini_config(&path.join("artefact_spawns.ltx"))?;
    let mut nodes: Vec<ArtefactSpawnPoint> = Vec::new();

    for (section, props) in &config {
      if section.is_some() {
        nodes.push(ArtefactSpawnPoint::import(props)?);
      }
    }

    log::info!("Imported artefact spawns chunk");

    Ok(ArtefactSpawnsChunk { nodes })
  }

  /// Export artefact spawns data into provided path.
  pub fn export<T: ByteOrder>(&self, path: &Path) -> io::Result<()> {
    let mut config: Ini = Ini::new();

    for (index, node) in self.nodes.iter().enumerate() {
      node.export(&index.to_string(), &mut config);
    }

    export_ini_to_file(
      &config,
      &mut create_export_file(&path.join("artefact_spawns.ltx"))?,
    )?;

    log::info!("Exported artefact spawns chunk");

    Ok(())
  }
}

impl fmt::Debug for ArtefactSpawnsChunk {
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
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::artefact_spawn_point::ArtefactSpawnPoint;
  use crate::data::vector_3d::Vector3d;
  use crate::file::artefact_spawns_chunk::ArtefactSpawnsChunk;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;

  #[test]
  fn test_read_write_artefact_spawn_point() {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_test_chunk_file_sub_dir(file!(), "artefact_spawns.chunk");

    let spawns: ArtefactSpawnsChunk = ArtefactSpawnsChunk {
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

    spawns.write::<SpawnByteOrder>(&mut writer).unwrap();

    assert_eq!(writer.bytes_written(), 44);

    let bytes_written: usize = writer
      .flush_chunk_into_file::<SpawnByteOrder>(
        &mut overwrite_test_resource_as_file(&filename).unwrap(),
        0,
      )
      .unwrap();

    assert_eq!(bytes_written, 44);

    let file: FileSlice = open_test_resource_as_slice(&filename).unwrap();

    assert_eq!(file.bytes_remaining(), 44 + 8);

    let chunk: Chunk = Chunk::from_slice(file)
      .unwrap()
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_spawns: ArtefactSpawnsChunk =
      ArtefactSpawnsChunk::read_from_chunk::<SpawnByteOrder>(chunk).unwrap();

    assert_eq!(read_spawns, spawns);
  }
}
