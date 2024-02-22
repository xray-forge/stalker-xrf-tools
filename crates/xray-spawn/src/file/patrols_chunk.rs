use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::patrol::patrol::Patrol;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io::Write;
use std::{fmt, io};

/// `CPatrolPathStorage::load` in xray.
#[derive(Clone, PartialEq)]
pub struct PatrolsChunk {
  pub patrols: Vec<Patrol>,
}

impl PatrolsChunk {
  /// Read patrols list from the chunk.
  pub fn read_from_chunk<T: ByteOrder>(mut chunk: Chunk) -> io::Result<PatrolsChunk> {
    log::info!(
      "Parsing patrols: {:?} -> {:?}",
      chunk.start_pos(),
      chunk.end_pos()
    );

    let mut meta_chunk: Chunk = chunk.read_child_by_index(0)?;
    let mut data_chunk: Chunk = chunk.read_child_by_index(1)?;

    assert_eq!(meta_chunk.size, 4);

    let count: u32 = meta_chunk.read_u32::<T>()?;
    let patrols: Vec<Patrol> = Patrol::read_list_from_chunk::<T>(&mut data_chunk, count)?;

    assert_eq!(count, patrols.len() as u32);
    assert!(chunk.is_ended());

    log::info!(
      "Parsed patrols: {:?} / {count}, {:?} bytes",
      patrols.len(),
      chunk.read_bytes_len()
    );

    Ok(PatrolsChunk { patrols })
  }

  /// Write patrols data into chunk writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    let mut meta_writer: ChunkWriter = ChunkWriter::new();
    let mut data_writer: ChunkWriter = ChunkWriter::new();

    meta_writer.write_u32::<T>(self.patrols.len() as u32)?;
    Patrol::write_list::<T>(&self.patrols, &mut data_writer)?;

    writer.write_all(meta_writer.flush_chunk_into_buffer::<T>(0)?.as_slice())?;
    writer.write_all(data_writer.flush_chunk_into_buffer::<T>(1)?.as_slice())?;

    Ok(())
  }
}

impl fmt::Debug for PatrolsChunk {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "PatrolsChunk {{ patrols: Vector[{}] }}",
      self.patrols.len(),
    )
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::patrol::patrol::Patrol;
  use crate::data::patrol::patrol_link::PatrolLink;
  use crate::data::patrol::patrol_point::PatrolPoint;
  use crate::file::patrols_chunk::PatrolsChunk;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_patrols_chunk() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_test_chunk_file_sub_dir(file!(), &String::from("patrols_list.chunk"));

    let patrols_chunk: PatrolsChunk = PatrolsChunk {
      patrols: vec![
        Patrol {
          name: String::from("patrol-1"),
          points: vec![
            PatrolPoint {
              name: String::from("patrol-point-1"),
              position: (1.5, -2.3, 1.0),
              flags: 33,
              level_vertex_id: 250,
              game_vertex_id: 555,
            },
            PatrolPoint {
              name: String::from("patrol-point-2"),
              position: (2.5, -5.3, 3.0),
              flags: 64,
              level_vertex_id: 5500,
              game_vertex_id: 666,
            },
          ],
          links: vec![PatrolLink {
            index: 0,
            links: vec![(10, 50.5), (15, 60.25)],
          }],
        },
        Patrol {
          name: String::from("patrol-2"),
          points: vec![
            PatrolPoint {
              name: String::from("patrol-point-1"),
              position: (7.5, -4.3, 3.0),
              flags: 1,
              level_vertex_id: 601,
              game_vertex_id: 541,
            },
            PatrolPoint {
              name: String::from("patrol-point-2"),
              position: (2.5, -5.3, 3.0),
              flags: 0,
              level_vertex_id: 600,
              game_vertex_id: 542,
            },
          ],
          links: vec![PatrolLink {
            index: 0,
            links: vec![(10, 50.5), (15, 60.25)],
          }],
        },
      ],
    };

    patrols_chunk.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 450);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 450);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 450 + 8);

    let chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_patrols_chunk: PatrolsChunk = PatrolsChunk::read_from_chunk::<SpawnByteOrder>(chunk)?;

    assert_eq!(read_patrols_chunk, patrols_chunk);

    Ok(())
  }
}
