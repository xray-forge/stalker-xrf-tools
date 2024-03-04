use crate::chunk::iterator::ChunkSizePackedIterator;
use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::Write;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphCrossTable {
  pub version: u32,
  pub nodes_count: u32,
  pub vertex_count: u32,
  pub level_guid: Uuid,
  pub game_guid: Uuid,
  #[serde(skip_serializing)]
  pub data: Vec<u8>,
}

impl GraphCrossTable {
  /// Read cross tables list data from the chunk.
  pub fn read_list<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<Vec<GraphCrossTable>> {
    let mut cross_tables: Vec<GraphCrossTable> = Vec::new();

    for mut cross_table_chunk in ChunkSizePackedIterator::new(reader) {
      cross_tables.push(GraphCrossTable::read::<T>(&mut cross_table_chunk)?);

      assert!(
        cross_table_chunk.is_ended(),
        "Expect cross table chunk to be ended"
      );
    }

    Ok(cross_tables)
  }

  /// Read cross table data from the chunk.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> io::Result<GraphCrossTable> {
    Ok(GraphCrossTable {
      version: reader.read_u32::<T>()?,
      nodes_count: reader.read_u32::<T>()?,
      vertex_count: reader.read_u32::<T>()?,
      level_guid: Uuid::from_u128(reader.read_u128::<T>()?),
      game_guid: Uuid::from_u128(reader.read_u128::<T>()?),
      data: reader.read_bytes(reader.read_bytes_remain() as usize)?,
    })
  }

  /// Write cross tables list data into the writer.
  pub fn write_list<T: ByteOrder>(
    cross_tables: &Vec<GraphCrossTable>,
    writer: &mut ChunkWriter,
  ) -> io::Result<()> {
    for table in cross_tables {
      let mut table_writer: ChunkWriter = ChunkWriter::new();

      table.write::<T>(&mut table_writer)?;

      writer.write_u32::<T>(table_writer.bytes_written() as u32 + 4)?;
      writer.write_all(&table_writer.buffer)?;
    }

    Ok(())
  }

  /// Write cross table data into the writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    writer.write_u32::<T>(self.version)?;
    writer.write_u32::<T>(self.nodes_count)?;
    writer.write_u32::<T>(self.vertex_count)?;
    writer.write_u128::<T>(self.level_guid.as_u128())?;
    writer.write_u128::<T>(self.game_guid.as_u128())?;
    writer.write_all(&self.data)?;

    Ok(())
  }

  /// Export cross-tables as separate chunk file.
  pub fn import_list<T: ByteOrder>(file: File) -> io::Result<Vec<GraphCrossTable>> {
    let mut cross_tables: Vec<GraphCrossTable> = Vec::new();

    for mut cross_table_reader in ChunkSizePackedIterator::new(&mut ChunkReader::from_file(file)?) {
      cross_tables.push(GraphCrossTable::read::<T>(&mut cross_table_reader)?);

      assert!(
        cross_table_reader.is_ended(),
        "Expect cross table chunk to be ended"
      );
    }

    Ok(cross_tables)
  }
  /// Export cross-tables as separate chunk file.
  pub fn export_list<T: ByteOrder>(
    cross_tables: &Vec<GraphCrossTable>,
    file: &mut File,
  ) -> io::Result<()> {
    let mut cross_tables_writer: ChunkWriter = ChunkWriter::new();

    for cross_table in cross_tables {
      let mut cross_table_writer: ChunkWriter = ChunkWriter::new();

      cross_table.write::<T>(&mut cross_table_writer)?;

      cross_tables_writer.write_u32::<T>(cross_table_writer.bytes_written() as u32 + 4)?;
      cross_tables_writer.write_all(&cross_table_writer.flush_raw_into_buffer()?)?;
    }

    cross_tables_writer.flush_raw_into_file(file)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::graph::graph_cross_table::GraphCrossTable;
  use crate::test::utils::{
    get_test_sample_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;
  use uuid::uuid;

  #[test]
  fn test_read_write_cross_table() -> io::Result<()> {
    let filename: String = String::from("cross_table.chunk");
    let mut writer: ChunkWriter = ChunkWriter::new();

    let cross_table: GraphCrossTable = GraphCrossTable {
      version: 16,
      nodes_count: 51,
      vertex_count: 4000,
      level_guid: uuid!("78e55023-10b1-426f-9247-bb680e5fe0b7"),
      game_guid: uuid!("78e55023-10b1-426f-9247-bb680e5fe0b7"),
      data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    };

    cross_table.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 55);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&get_test_sample_file_sub_dir(file!(), &filename))?,
      0,
    )?;

    assert_eq!(bytes_written, 55);

    let file: FileSlice =
      open_test_resource_as_slice(&get_test_sample_file_sub_dir(file!(), &filename))?;

    assert_eq!(file.bytes_remaining(), 55 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read_cross_table: GraphCrossTable = GraphCrossTable::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read_cross_table, cross_table);

    Ok(())
  }
}
