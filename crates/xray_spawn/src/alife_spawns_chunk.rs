use crate::chunk::chunk::Chunk;
use crate::chunk::iterator::ChunkIterator;
use crate::data::alife_object_base::AlifeObjectBase;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;
use std::fmt;

pub struct ALifeObjectsChunk {
  pub chunk: Chunk,
  pub objects: Vec<AlifeObjectBase>,
}

/// ALife spawns chunk contains3 children entries.
/// 1 - count of objects
/// 2 - vertices
/// 3 - edges
impl ALifeObjectsChunk {
  /// Read spawns chunk by position descriptor.
  pub fn from_chunk(mut chunk: Chunk) -> Option<ALifeObjectsChunk> {
    let mut objects: Vec<AlifeObjectBase> = Vec::new();

    log::info!(
      "Parsing alife spawns chunk, {:?} -> {:?}",
      chunk.start_pos(),
      chunk.end_pos()
    );

    let mut count_chunk: Chunk = chunk.read_child_by_index(0).unwrap();
    let count: u32 = count_chunk.read_u32::<SpawnByteOrder>().unwrap();

    let objects_chunk: Chunk = chunk.read_child_by_index(1).unwrap();
    for mut object_chunk in ChunkIterator::new(&mut objects_chunk.file.clone()) {
      objects.push(AlifeObjectBase::from_chunk(&mut object_chunk))
    }

    Self::advance_placeholder_chunk(&mut chunk);

    log::info!("Parsed alife spawns chunk, {count} objects processed");

    assert_eq!(objects.len(), count as usize);
    assert_eq!(chunk.read_bytes_remain(), 0);

    return Some(ALifeObjectsChunk { chunk, objects });
  }

  /// Empty chunk declared as placeholder, unknown purpose.
  fn advance_placeholder_chunk(chunk: &mut Chunk) -> () {
    let edges_chunk: Chunk = chunk.read_child_by_index(2).unwrap();

    assert_eq!(
      edges_chunk.read_bytes_remain(),
      0,
      "Parsing of edges in spawn chunk is not implemented."
    )
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
