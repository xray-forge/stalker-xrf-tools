use crate::spawn::chunk::{Chunk, ChunkIterator};
use byteorder::{LittleEndian, ReadBytesExt};
use fileslice::FileSlice;

#[derive(Debug)]
pub struct ALifeSpawnsChunk {
  pub id: u32,
  pub count: u32,
}

/// ALife spawns chunk contains3 children entries.
/// 1 - count of objects
/// 2 - vertices
/// 3 - edges
impl ALifeSpawnsChunk {
  /// Read spawns chunk by position descriptor.
  pub fn from_chunk(file: &FileSlice, chunk: &Chunk) -> Option<ALifeSpawnsChunk> {
    let mut file: FileSlice = chunk.in_slice(file);

    let count: u32 = Self::read_entities_count(&mut file);
    let parsed_vertices_count: u32 = Self::read_vertices(&mut file);

    Self::read_edges(&mut file);

    log::info!("Parsed alife spawns chunk, {count} objects processed");

    assert_eq!(count, parsed_vertices_count);
    assert_eq!(file.cursor_pos(), file.end_pos());

    return Some(ALifeSpawnsChunk {
      id: chunk.id,
      count,
    });
  }

  fn read_entities_count(file: &mut FileSlice) -> u32 {
    let (mut base_slice, _) = Chunk::open_by_index(file, 0).unwrap();
    base_slice.read_u32::<LittleEndian>().unwrap()
  }

  fn read_vertices(file: &mut FileSlice) -> u32 {
    let (vertices_slice, _) = Chunk::open_by_index(file, 1).unwrap();
    let mut parsed_vertices_count: u32 = 0;

    for it in ChunkIterator::new(&mut vertices_slice.clone()) {
      let mut vertex_slice: FileSlice = it.in_slice(&vertices_slice);

      let (mut vertex_id_slice, _) =
        Chunk::open_by_index(&mut vertex_slice, 0).expect("Expected vertex ID chunk to exist.");

      let vertex_id: u16 = vertex_id_slice.read_u16::<LittleEndian>().unwrap();

      let (mut vertex_data_slice, _) =
        Chunk::open_by_index(&mut vertex_slice, 1).expect("Expected vertex ID chunk to exist.");

      Self::read_object_data(&mut vertex_data_slice);

      // todo: Collect object data here.

      parsed_vertices_count += 1;
    }

    parsed_vertices_count
  }

  fn read_object_data(file: &mut FileSlice) -> () {
    let (mut id_slice, _) =
      Chunk::open_by_index(file, 1).expect("Expected id chunk to exist in object definition.");

    let id: u32 = id_slice.read_u32::<LittleEndian>().unwrap();

    let (mut data_slice, data_chunk) =
      Chunk::open_by_index(file, 0).expect("Expected data chunk to exist in object definition.");

    let data_length: u16 = data_slice.read_u16::<LittleEndian>().unwrap();

    assert_eq!(data_length as u32 + 2, data_chunk.size);

    // todo: Parse object.
  }

  fn read_edges(file: &mut FileSlice) -> () {
    let (edges_slice, _) = Chunk::open_by_index(file, 2).unwrap();

    if !edges_slice.is_empty() {
      panic!("Parsing of edges in spawn chunk is not implemented.");
    }
  }
}
