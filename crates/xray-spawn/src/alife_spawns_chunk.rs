use crate::chunk::chunk::Chunk;
use crate::chunk::iterator::ChunkIterator;
use crate::data::alife_object_base::AlifeObjectBase;
use byteorder::{ByteOrder, ReadBytesExt};
use std::{fmt, io};

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
  pub fn read_from_chunk<T: ByteOrder>(mut chunk: Chunk) -> io::Result<ALifeObjectsChunk> {
    let mut objects: Vec<AlifeObjectBase> = Vec::new();

    log::info!(
      "Parsing alife spawns chunk, {:?} -> {:?}",
      chunk.start_pos(),
      chunk.end_pos()
    );

    let mut count_chunk: Chunk = chunk.read_child_by_index(0)?;
    let count: u32 = count_chunk.read_u32::<T>().unwrap();

    let mut objects_chunk: Chunk = chunk.read_child_by_index(1)?;
    for mut object_chunk in ChunkIterator::new(&mut objects_chunk) {
      objects.push(AlifeObjectBase::read_from_chunk::<T>(&mut object_chunk)?)
    }

    Self::advance_placeholder_chunk(&mut chunk)?;

    log::info!("Parsed alife spawns chunk, {count} objects processed");

    assert_eq!(objects.len(), count as usize);
    assert!(chunk.is_ended());

    Ok(ALifeObjectsChunk { chunk, objects })
  }

  /// Empty chunk declared as placeholder, unknown purpose.
  fn advance_placeholder_chunk(chunk: &mut Chunk) -> io::Result<()> {
    let edges_chunk: Chunk = chunk.read_child_by_index(2)?;

    assert_eq!(
      edges_chunk.read_bytes_remain(),
      0,
      "Parsing of edges in spawn chunk is not implemented."
    );

    Ok(())
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
