use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::artefact_spawn_point::ArtefactSpawnPoint;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::{fmt, io};

/// Artefacts spawns chunks.
/// Is single plain chunk with nodes list in it.
pub struct ArtefactSpawnsChunk {
  pub nodes: Vec<ArtefactSpawnPoint>,
}

impl ArtefactSpawnsChunk {
  /// Read header chunk by position descriptor.
  pub fn read_from_chunk<T: ByteOrder>(mut chunk: Chunk) -> io::Result<ArtefactSpawnsChunk> {
    log::info!(
      "Parsing artefacts chunk, {:?} -> {:?}",
      chunk.start_pos(),
      chunk.end_pos(),
    );

    let mut nodes: Vec<ArtefactSpawnPoint> = Vec::new();
    let count: u32 = chunk.read_u32::<T>()?;

    // Parsing CLevelPoint structure, 20 bytes per one.
    for _ in 0..count {
      nodes.push(ArtefactSpawnPoint::read_from_chunk::<T>(&mut chunk)?);
    }

    assert_eq!(nodes.len() as u64, count as u64);

    assert!(
      chunk.is_ended(),
      "Expect artefact spawns chunk to be ended."
    );

    log::info!(
      "Parsed artefacts spawns: {:?} processed, {:?} remain",
      chunk.read_bytes_len(),
      chunk.read_bytes_remain(),
    );

    Ok(ArtefactSpawnsChunk { nodes })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_u32::<T>(self.nodes.len() as u32)?;

    for node in &self.nodes {
      node.write::<T>(writer)?;
    }

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
  use crate::file::artefact_spawns_chunk::ArtefactSpawnsChunk;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;

  #[test]
  fn test_read_write_simple_artefact_spawn_point() {
    let mut writer: ChunkWriter = ChunkWriter::new();

    ArtefactSpawnsChunk {
      nodes: vec![
        ArtefactSpawnPoint {
          position: (55.5, 44.4, -33.3),
          level_vertex_id: 255,
          distance: 450.30,
        },
        ArtefactSpawnPoint {
          position: (-21.0, 13.5, -4.0),
          level_vertex_id: 13,
          distance: 25.11,
        },
      ],
    }
    .write::<SpawnByteOrder>(&mut writer)
    .unwrap();

    assert_eq!(writer.bytes_written(), 44);

    let bytes_written: usize = writer
      .flush_chunk::<SpawnByteOrder>(
        &mut overwrite_test_resource_as_file(get_test_chunk_file_sub_dir(
          file!(),
          String::from("artefact_spawns_simple.chunk"),
        ))
        .unwrap(),
        0,
      )
      .unwrap();

    assert_eq!(bytes_written, 44);

    let file: FileSlice = open_test_resource_as_slice(get_test_chunk_file_sub_dir(
      file!(),
      String::from("artefact_spawns_simple.chunk"),
    ))
    .unwrap();

    assert_eq!(file.bytes_remaining(), 44 + 8);

    let chunk: Chunk = Chunk::from_file(file)
      .unwrap()
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let artefact_spawns: ArtefactSpawnsChunk =
      ArtefactSpawnsChunk::read_from_chunk::<SpawnByteOrder>(chunk).unwrap();

    assert_eq!(artefact_spawns.nodes.len(), 2);

    assert_eq!(artefact_spawns.nodes.get(0).unwrap().position.0, 55.5);
    assert_eq!(artefact_spawns.nodes.get(0).unwrap().position.1, 44.4);
    assert_eq!(artefact_spawns.nodes.get(0).unwrap().position.2, -33.3);
    assert_eq!(artefact_spawns.nodes.get(0).unwrap().level_vertex_id, 255);
    assert_eq!(artefact_spawns.nodes.get(0).unwrap().distance, 450.30);

    assert_eq!(artefact_spawns.nodes.get(1).unwrap().position.0, -21.0);
    assert_eq!(artefact_spawns.nodes.get(1).unwrap().position.1, 13.5);
    assert_eq!(artefact_spawns.nodes.get(1).unwrap().position.2, -4.0);
    assert_eq!(artefact_spawns.nodes.get(1).unwrap().level_vertex_id, 13);
    assert_eq!(artefact_spawns.nodes.get(1).unwrap().distance, 25.11);
  }
}
