use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::types::Vector3d;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Debug)]
pub struct ArtefactSpawnPoint {
  pub position: (f32, f32, f32),
  pub level_vertex_id: u32,
  pub distance: f32,
}

impl ArtefactSpawnPoint {
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<ArtefactSpawnPoint> {
    let position: Vector3d = chunk.read_f32_3d_vector::<T>()?;
    let level_vertex_id: u32 = chunk.read_u32::<T>()?;
    let distance: f32 = chunk.read_f32::<T>()?;

    Ok(ArtefactSpawnPoint {
      position,
      level_vertex_id,
      distance,
    })
  }

  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_f32_3d_vector::<T>(&self.position)?;
    writer.write_u32::<T>(self.level_vertex_id)?;
    writer.write_f32::<T>(self.distance)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::artefact_spawn_point::ArtefactSpawnPoint;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;

  #[test]
  fn test_read_write_simple_artefact_spawn_point() {
    let mut writer: ChunkWriter = ChunkWriter::new();

    ArtefactSpawnPoint {
      position: (10.5, 20.3, -40.5),
      level_vertex_id: 1000,
      distance: 500.55,
    }
    .write::<SpawnByteOrder>(&mut writer)
    .unwrap();

    assert_eq!(writer.bytes_written(), 20);

    let bytes_written: usize = writer
      .flush_chunk::<SpawnByteOrder>(
        &mut overwrite_test_resource_as_file(get_test_chunk_file_sub_dir(
          file!(),
          String::from("artefact_spawn_point_simple.chunk"),
        ))
        .unwrap(),
        0,
      )
      .unwrap();

    assert_eq!(bytes_written, 20);

    let file: FileSlice = open_test_resource_as_slice(get_test_chunk_file_sub_dir(
      file!(),
      String::from("artefact_spawn_point_simple.chunk"),
    ))
    .unwrap();

    assert_eq!(file.bytes_remaining(), 20 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)
      .unwrap()
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let point: ArtefactSpawnPoint =
      ArtefactSpawnPoint::read_from_chunk::<SpawnByteOrder>(&mut chunk).unwrap();

    assert_eq!(point.position.0, 10.5);
    assert_eq!(point.position.1, 20.3);
    assert_eq!(point.position.2, -40.5);
    assert_eq!(point.level_vertex_id, 1000);
    assert_eq!(point.distance, 500.55);
  }
}
