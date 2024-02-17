use crate::spawn::chunk::{Chunk, ChunkSliceIterator};
use crate::spawn::data::alife_object::AlifeObject;
use byteorder::{LittleEndian, ReadBytesExt};
use fileslice::FileSlice;
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
  pub fn from_chunk(file: &FileSlice, chunk: &Chunk) -> Option<ALifeObjectsChunk> {
    let mut file: FileSlice = chunk.in_slice(file);
    let mut objects: Vec<AlifeObject> = Vec::new();

    log::info!(
      "Parsing alife spawns chunk, {:?} -> {:?}",
      file.start_pos(),
      file.end_pos()
    );

    let (mut count_slice, _) = Chunk::open_by_index(&mut file, 0).unwrap();
    let count: u32 = count_slice.read_u32::<LittleEndian>().unwrap();

    let (objects_slice, _) = Chunk::open_by_index(&mut file, 1).unwrap();
    for (mut object_slice, _) in ChunkSliceIterator::new(&mut objects_slice.clone()) {
      objects.push(AlifeObject::from_file(&mut object_slice))
    }

    Self::advance_placeholder_chunk(&mut file);

    log::info!("Parsed alife spawns chunk, {count} objects processed");

    assert_eq!(objects.len(), count as usize);
    assert_eq!(file.cursor_pos(), file.end_pos());

    return Some(ALifeObjectsChunk {
      id: chunk.id,
      objects,
    });
  }

  /// Empty chunk declared as placeholder, unknown purpose.
  fn advance_placeholder_chunk(file: &mut FileSlice) -> () {
    let (edges_slice, _) = Chunk::open_by_index(file, 2).unwrap();

    assert!(
      edges_slice.is_empty(),
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
