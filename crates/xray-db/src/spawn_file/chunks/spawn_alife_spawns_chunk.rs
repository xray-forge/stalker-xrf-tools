use crate::chunk::iterator::ChunkIterator;
use crate::chunk::reader::ChunkReader;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_base::AlifeObjectBase;
use crate::export::file::{create_export_file, open_ltx_config};
use crate::types::DatabaseResult;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::Write;
use std::path::Path;
use xray_ltx::Ltx;

/// ALife spawns chunk has the following structure:
/// 0 - count
/// 1 - objects
/// 2 - edges
#[derive(Serialize, Deserialize)]
pub struct SpawnALifeSpawnsChunk {
  pub objects: Vec<AlifeObjectBase>,
}

impl SpawnALifeSpawnsChunk {
  pub const CHUNK_ID: u32 = 1;

  /// Read spawns chunk by position descriptor from the chunk.
  pub fn read<T: ByteOrder>(reader: &mut ChunkReader) -> DatabaseResult<Self> {
    let mut count_reader: ChunkReader = reader.read_child_by_index(0)?;
    let mut objects_reader: ChunkReader = reader.read_child_by_index(1)?;
    let edges_reader: ChunkReader = reader.read_child_by_index(2)?;

    let count: u32 = count_reader.read_u32::<T>()?;
    let mut objects: Vec<AlifeObjectBase> = Vec::new();

    for mut object_reader in ChunkIterator::new(&mut objects_reader) {
      objects.push(AlifeObjectBase::read::<T>(&mut object_reader)?)
    }

    assert_eq!(objects.len(), count as usize);
    assert!(count_reader.is_ended(), "Expect count chunk to be ended");
    assert!(
      objects_reader.is_ended(),
      "Expect objects chunk to be ended"
    );
    assert!(
      edges_reader.is_ended(),
      "Parsing of edges in spawn chunk is not implemented"
    );
    assert!(reader.is_ended(), "Expect alife spawns chunk to be ended");

    log::info!("Parsed alife spawns chunk, {:?} bytes", reader.size);

    Ok(Self { objects })
  }

  /// Write alife chunk data into the writer.
  pub fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> DatabaseResult {
    let mut count_writer: ChunkWriter = ChunkWriter::new();
    let mut objects_writer: ChunkWriter = ChunkWriter::new();
    let mut vertex_writer: ChunkWriter = ChunkWriter::new();

    count_writer.write_u32::<T>(self.objects.len() as u32)?;

    for (index, object) in self.objects.iter().enumerate() {
      let mut object_writer = ChunkWriter::new();

      object.write::<T>(&mut object_writer)?;

      objects_writer.write_all(
        object_writer
          .flush_chunk_into_buffer::<T>(index)?
          .as_slice(),
      )?;
    }

    writer.write_all(count_writer.flush_chunk_into_buffer::<T>(0)?.as_slice())?;
    writer.write_all(objects_writer.flush_chunk_into_buffer::<T>(1)?.as_slice())?;
    writer.write_all(vertex_writer.flush_chunk_into_buffer::<T>(2)?.as_slice())?;

    log::info!(
      "Written alife spawns chunk, {:?} bytes",
      writer.bytes_written()
    );

    Ok(())
  }

  /// Import alife spawns data from provided path.
  pub fn import(path: &Path) -> DatabaseResult<Self> {
    let ltx: Ltx = open_ltx_config(&path.join("alife_spawns.ltx"))?;
    let mut objects: Vec<AlifeObjectBase> = Vec::new();

    for (section_name, _) in ltx.iter() {
      objects.push(AlifeObjectBase::import(section_name, &ltx)?);
    }

    log::info!("Imported alife spawns chunk");

    Ok(Self { objects })
  }

  /// Export alife spawns data into provided path.
  pub fn export(&self, path: &Path) -> DatabaseResult {
    let mut ltx: Ltx = Ltx::new();

    for object in &self.objects {
      object.export(&object.index.to_string(), &mut ltx)?;
    }

    ltx.write_to(&mut create_export_file(&path.join("alife_spawns.ltx"))?)?;

    log::info!("Exported alife spawns chunk");

    Ok(())
  }
}

impl fmt::Debug for SpawnALifeSpawnsChunk {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      formatter,
      "ALifeObjectsChunk {{ objects: Vector[{}] }}",
      self.objects.len(),
    )
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::reader::ChunkReader;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_base::AlifeObjectBase;
  use crate::data::alife::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::alife_object_item::AlifeObjectItem;
  use crate::data::alife::alife_object_item_custom_outfit::AlifeObjectItemCustomOutfit;
  use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::generic::vector_3d::Vector3d;
  use crate::data::meta::cls_id::ClsId;
  use crate::spawn_file::chunks::spawn_alife_spawns_chunk::SpawnALifeSpawnsChunk;
  use crate::types::{DatabaseResult, SpawnByteOrder};
  use fileslice::FileSlice;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };

  #[test]
  fn test_read_write_empty() -> DatabaseResult {
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write_empty.chunk");

    let original: SpawnALifeSpawnsChunk = SpawnALifeSpawnsChunk { objects: vec![] };

    let mut writer: ChunkWriter = ChunkWriter::new();

    original.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 28);

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 28);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 28 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read: SpawnALifeSpawnsChunk = SpawnALifeSpawnsChunk::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read.objects.len(), original.objects.len());

    Ok(())
  }

  #[test]
  fn test_read_write() -> DatabaseResult {
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: SpawnALifeSpawnsChunk = SpawnALifeSpawnsChunk {
      objects: vec![
        AlifeObjectBase {
          index: 21,
          id: 2334,
          net_action: 1,
          section: String::from("exo_outfit"),
          clsid: ClsId::EStlk,
          name: String::from("test-outfit-object"),
          script_game_id: 2,
          script_rp: 3,
          position: Vector3d::new(1.4, 2.0, 3.0),
          direction: Vector3d::new(3.0, 2.5, 1.0),
          respawn_time: 50000,
          parent_id: 2143,
          phantom_id: 0,
          script_flags: 33,
          version: 128,
          game_type: 1,
          script_version: 10,
          client_data_size: 0,
          spawn_id: 2354,
          inherited: Box::new(AlifeObjectItemCustomOutfit {
            base: AlifeObjectItem {
              base: AlifeObjectDynamicVisual {
                base: AlifeObjectAbstract {
                  game_vertex_id: 43543,
                  distance: 523.33,
                  direct_control: 423,
                  level_vertex_id: 142,
                  flags: 34,
                  custom_data: String::from("custom-data"),
                  story_id: 256973,
                  spawn_story_id: 356490,
                },
                visual_name: String::from("visual-name"),
                visual_flags: 0,
              },
              condition: 1.0,
              upgrades_count: 0,
            },
          }),
          update_data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        },
        AlifeObjectBase {
          index: 22,
          id: 2335,
          net_action: 1,
          section: String::from("space_restrictor"),
          clsid: ClsId::SpcRsS,
          name: String::from("test-restrictor-object"),
          script_game_id: 5,
          script_rp: 52,
          position: Vector3d::new(1.0, 2.0, 4.0),
          direction: Vector3d::new(5.0, 2.0, 1.0),
          respawn_time: 50000,
          parent_id: 2463,
          phantom_id: 0,
          script_flags: 33,
          version: 128,
          game_type: 1,
          script_version: 10,
          client_data_size: 0,
          spawn_id: 2354,
          inherited: Box::new(AlifeObjectSpaceRestrictor {
            base: AlifeObjectAbstract {
              game_vertex_id: 5473,
              distance: 45.5,
              direct_control: 373574,
              level_vertex_id: 253,
              flags: 0,
              custom_data: String::from("custom-data"),
              story_id: 3564,
              spawn_story_id: 38754,
            },
            shape: vec![],
            restrictor_type: 0,
          }),
          update_data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        },
      ],
    };

    let mut writer: ChunkWriter = ChunkWriter::new();

    original.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 419);

    let bytes_written: usize = writer.flush_chunk_into::<SpawnByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 419);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 419 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read: SpawnALifeSpawnsChunk = SpawnALifeSpawnsChunk::read::<SpawnByteOrder>(&mut reader)?;

    assert_eq!(read.objects.len(), original.objects.len());

    for (index, object) in read.objects.iter().enumerate() {
      let another: &AlifeObjectBase = original.objects.get(index).unwrap();

      assert_eq!(object.index, another.index);
      assert_eq!(object.id, another.id);
      assert_eq!(object.net_action, another.net_action);
      assert_eq!(object.section, another.section);
      assert_eq!(object.clsid, another.clsid);
      assert_eq!(object.name, another.name);
      assert_eq!(object.script_game_id, another.script_game_id);
      assert_eq!(object.script_rp, another.script_rp);
      assert_eq!(object.position, another.position);
      assert_eq!(object.direction, another.direction);
      assert_eq!(object.respawn_time, another.respawn_time);
      assert_eq!(object.parent_id, another.parent_id);
      assert_eq!(object.phantom_id, another.phantom_id);
      assert_eq!(object.script_flags, another.script_flags);
      assert_eq!(object.version, another.version);
      assert_eq!(object.game_type, another.game_type);
      assert_eq!(object.script_version, another.script_version);
      assert_eq!(object.client_data_size, another.client_data_size);
      assert_eq!(object.spawn_id, another.spawn_id);
      assert_eq!(object.update_data, another.update_data);
    }

    Ok(())
  }
}
