use crate::chunk::chunk::Chunk;
use crate::chunk::iterator::ChunkIterator;
use crate::data::alife_object_base::AlifeObjectBase;
use byteorder::{ByteOrder, ReadBytesExt};
use std::{fmt, io};

/// ALife spawns chunk has the following structure:
/// 0 - count of objects
/// 1 - vertices
/// 2 - edges
pub struct ALifeObjectsChunk {
  pub chunk: Chunk,
  pub objects: Vec<AlifeObjectBase>,
}

impl ALifeObjectsChunk {
  /// Read spawns chunk by position descriptor.
  pub fn read_from_chunk<T: ByteOrder>(mut chunk: Chunk) -> io::Result<ALifeObjectsChunk> {
    log::info!(
      "Parsing alife spawns chunk, {:?} -> {:?}",
      chunk.start_pos(),
      chunk.end_pos()
    );

    let mut count_chunk: Chunk = chunk.read_child_by_index(0)?;
    let mut objects_chunk: Chunk = chunk.read_child_by_index(1)?;
    let edges_chunk: Chunk = chunk.read_child_by_index(2)?;

    let count: u32 = count_chunk.read_u32::<T>()?;
    let mut objects: Vec<AlifeObjectBase> = Vec::new();

    for mut object_chunk in ChunkIterator::new(&mut objects_chunk) {
      objects.push(AlifeObjectBase::read_from_chunk::<T>(&mut object_chunk)?)
    }

    assert_eq!(objects.len(), count as usize);
    assert!(count_chunk.is_ended(), "Expect count chunk to be ended.");
    assert!(
      objects_chunk.is_ended(),
      "Expect objects chunk to be ended."
    );
    assert!(
      edges_chunk.is_ended(),
      "Parsing of edges in spawn chunk is not implemented."
    );
    assert!(chunk.is_ended(), "Expect alife spawns chunk to be ended.");

    log::info!("Parsed alife spawns chunk, {count} objects processed");

    Ok(ALifeObjectsChunk { chunk, objects })
  }
}

impl fmt::Debug for ALifeObjectsChunk {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "ALifeObjectsChunk {{ chunk: {:?}, objects: Vector[{}] }}",
      self.chunk,
      self.objects.len(),
    )
  }
}
