use crate::spawn::chunk::chunk::Chunk;
use crate::spawn::chunk::iterator::ChunkIterator;
use crate::spawn::data::alife_object::AlifeObject;
use crate::spawn::types::SpawnByteOrder;
use byteorder::ReadBytesExt;
use std::fmt;

pub struct ALifeObjectsChunk {
  pub id: u32,
  pub objects: Vec<AlifeObject>,
}

/// ALife spawns chunk contains3 children entries.
/// 1 - count of objects
/// 2 - vertices
/// 3 - edges
impl ALifeObjectsChunk {
  /// Read spawns chunk by position descriptor.
  pub fn from_chunk(mut chunk: Chunk) -> Option<ALifeObjectsChunk> {
    let mut objects: Vec<AlifeObject> = Vec::new();

    log::info!(
      "Parsing alife spawns chunk, {:?} -> {:?}",
      chunk.start_pos(),
      chunk.end_pos()
    );

    let mut count_chunk: Chunk = chunk.read_child_by_index(0).unwrap();
    let count: u32 = count_chunk.file.read_u32::<SpawnByteOrder>().unwrap();

    let objects_chunk: Chunk = chunk.read_child_by_index(1).unwrap();
    for mut object_chunk in ChunkIterator::new(&mut objects_chunk.file.clone()) {
      objects.push(AlifeObject::from_chunk(&mut object_chunk))
    }

    Self::advance_placeholder_chunk(&mut chunk);

    log::info!("Parsed alife spawns chunk, {count} objects processed");

    assert_eq!(objects.len(), count as usize);
    assert_eq!(chunk.read_bytes_remain(), 0);

    return Some(ALifeObjectsChunk {
      id: chunk.id,
      objects,
    });
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
      "ALifeObjectsChunk {{ id: {}, objects: Vector[{}] }}",
      self.id,
      self.objects.len(),
    )
  }
}
