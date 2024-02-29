use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::vector_3d::Vector3d;
use crate::export::file_import::read_ini_field;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use ini::{Ini, Properties};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct ArtefactSpawnPoint {
  pub position: Vector3d,
  pub level_vertex_id: u32,
  pub distance: f32,
}

impl ArtefactSpawnPoint {
  /// Read artefact spawn point from the chunk.
  pub fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<ArtefactSpawnPoint> {
    Ok(ArtefactSpawnPoint {
      position: chunk.read_f32_3d_vector::<T>()?,
      level_vertex_id: chunk.read_u32::<T>()?,
      distance: chunk.read_f32::<T>()?,
    })
  }

  /// Write artefact spawn point data into the writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_f32_3d_vector::<T>(&self.position)?;
    writer.write_u32::<T>(self.level_vertex_id)?;
    writer.write_f32::<T>(self.distance)?;

    Ok(())
  }

  /// Import artefact spawn point data from ini section.
  pub fn import(props: &Properties) -> io::Result<ArtefactSpawnPoint> {
    Ok(ArtefactSpawnPoint {
      position: read_ini_field("position", props)?,
      level_vertex_id: read_ini_field("level_vertex_id", props)?,
      distance: read_ini_field("distance", props)?,
    })
  }

  /// Export artefact spawn point data into ini.
  pub fn export(&self, section: &str, ini: &mut Ini) {
    ini
      .with_section(Some(section))
      .set("distance", self.distance.to_string())
      .set("position", self.position.to_string())
      .set("level_vertex_id", self.level_vertex_id.to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::artefact_spawn_point::ArtefactSpawnPoint;
  use crate::data::vector_3d::Vector3d;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_simple_artefact_spawn_point() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_test_chunk_file_sub_dir(file!(), "artefact_spawn_point_simple.chunk");
    let point: ArtefactSpawnPoint = ArtefactSpawnPoint {
      position: Vector3d::new(10.5, 20.3, -40.5),
      level_vertex_id: 1000,
      distance: 500.55,
    };

    point.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 20);

    let bytes_written: usize = writer
      .flush_chunk_into_file::<SpawnByteOrder>(&mut overwrite_test_resource_as_file(&filename)?, 0)
      .unwrap();

    assert_eq!(bytes_written, 20);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 20 + 8);

    let mut chunk: Chunk = Chunk::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_point: ArtefactSpawnPoint =
      ArtefactSpawnPoint::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_point, point);

    Ok(())
  }
}
