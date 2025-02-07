use crate::data::alife::alife_object::AlifeObject;
use crate::export::{FileImportExport, LtxImportExport};
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::Write;
use std::path::Path;
use xray_chunk::{ChunkIterator, ChunkReadWrite, ChunkReader, ChunkWriter};
use xray_error::XRayResult;
use xray_ltx::Ltx;
use xray_utils::{assert_equal, assert_length, open_export_file};

/// ALife spawns chunk has the following structure:
/// 0 - count
/// 1 - objects
/// 2 - edges
#[derive(Serialize, Deserialize)]
pub struct SpawnALifeSpawnsChunk {
  pub objects: Vec<AlifeObject>,
}

impl SpawnALifeSpawnsChunk {
  pub const CHUNK_ID: u32 = 1;

  pub const COUNT_CHUNK_ID: u32 = 0;
  pub const OBJECTS_CHUNK_ID: u32 = 1;
  pub const VERTEX_CHUNK_ID: u32 = 2;

  pub const OBJECT_INDEX_CHUNK_ID: u32 = 0;
  pub const OBJECT_DATA_CHUNK_ID: u32 = 1;
}

impl ChunkReadWrite for SpawnALifeSpawnsChunk {
  /// Read spawns chunk by position descriptor from the chunk.
  fn read<T: ByteOrder>(reader: &mut ChunkReader) -> XRayResult<Self> {
    log::info!(
      "Reading ALife spawns chunk, {} bytes",
      reader.read_bytes_remain()
    );

    let mut count_reader: ChunkReader = reader.read_child_by_index(Self::COUNT_CHUNK_ID)?;
    let mut objects_reader: ChunkReader = reader.read_child_by_index(Self::OBJECTS_CHUNK_ID)?;
    let vertex_reader: ChunkReader = reader.read_child_by_index(Self::VERTEX_CHUNK_ID)?;

    let count: u32 = count_reader.read_u32::<T>()?;
    let mut objects: Vec<AlifeObject> = Vec::with_capacity(count as usize);

    for mut object_reader in ChunkIterator::from_start(&mut objects_reader) {
      let mut index_reader: ChunkReader =
        object_reader.read_child_by_index(Self::OBJECT_INDEX_CHUNK_ID)?;
      let index: u16 = index_reader.read_u16::<T>()?;

      assert_equal(
        index as u32,
        object_reader.id,
        "Expected index and chunk ID to be equal",
      )?;
      index_reader.assert_read("Expect ALife object index to be read")?;

      let mut data_reader: ChunkReader =
        object_reader.read_child_by_index(Self::OBJECT_DATA_CHUNK_ID)?;
      let data: AlifeObject = data_reader.read_xr::<T, _>()?;

      objects.push(data);

      data_reader.assert_read("Expect ALife object data to be read")?;
    }

    assert_length(&objects, count as usize, "Expect all object read")?;

    count_reader.assert_read("Expect count chunk to be ended")?;
    objects_reader.assert_read("Expect objects chunk to be ended")?;
    vertex_reader.assert_read("Parsing of edges in spawn chunk is not implemented")?;
    reader.assert_read("Expect ALife spawns chunk to be ended")?;

    Ok(Self { objects })
  }

  /// Write ALife chunk data into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> XRayResult {
    let mut count_writer: ChunkWriter = ChunkWriter::new();
    let mut objects_writer: ChunkWriter = ChunkWriter::new();
    let mut vertex_writer: ChunkWriter = ChunkWriter::new();

    count_writer.write_u32::<T>(self.objects.len() as u32)?;

    for (index, object) in self.objects.iter().enumerate() {
      let mut object_writer: ChunkWriter = ChunkWriter::new();

      let mut index_writer: ChunkWriter = ChunkWriter::new();
      index_writer.write_u16::<T>(index as u16)?;
      object_writer.write_all(
        index_writer
          .flush_chunk_into_buffer::<T>(Self::OBJECT_INDEX_CHUNK_ID)?
          .as_slice(),
      )?;

      let mut data_writer: ChunkWriter = ChunkWriter::new();
      object.write::<T>(&mut data_writer)?;
      object_writer.write_all(
        data_writer
          .flush_chunk_into_buffer::<T>(Self::OBJECT_DATA_CHUNK_ID)?
          .as_slice(),
      )?;

      objects_writer.write_all(
        object_writer
          .flush_chunk_into_buffer::<T>(index as u32)?
          .as_slice(),
      )?;
    }

    writer.write_all(
      count_writer
        .flush_chunk_into_buffer::<T>(Self::COUNT_CHUNK_ID)?
        .as_slice(),
    )?;
    writer.write_all(
      objects_writer
        .flush_chunk_into_buffer::<T>(Self::OBJECTS_CHUNK_ID)?
        .as_slice(),
    )?;
    writer.write_all(
      vertex_writer
        .flush_chunk_into_buffer::<T>(Self::VERTEX_CHUNK_ID)?
        .as_slice(),
    )?;

    log::info!(
      "Written ALife spawns chunk, {} bytes",
      writer.bytes_written()
    );

    Ok(())
  }
}

impl FileImportExport for SpawnALifeSpawnsChunk {
  /// Import ALife spawns data from provided path.
  fn import<P: AsRef<Path>>(path: &P) -> XRayResult<Self> {
    let ltx: Ltx = Ltx::read_from_path(path.as_ref().join("alife_spawns.ltx"))?;
    let mut objects: Vec<AlifeObject> = Vec::with_capacity(ltx.sections.len());

    for (section_name, _) in &ltx {
      objects.push(AlifeObject::import(section_name, &ltx)?);
    }

    log::info!("Imported ALife spawns chunk");

    Ok(Self { objects })
  }

  /// Export ALife spawns data into provided path.
  fn export<P: AsRef<Path>>(&self, path: &P) -> XRayResult {
    let mut ltx: Ltx = Ltx::new();

    for (index, object) in self.objects.iter().enumerate() {
      object.export(&index.to_string(), &mut ltx)?;
    }

    ltx.write_to(&mut open_export_file(
      path.as_ref().join("alife_spawns.ltx"),
    )?)?;

    log::info!("Exported ALife spawns chunk");

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
  use crate::data::alife::alife_object::AlifeObject;
  use crate::data::alife::alife_object_inherited::AlifeObjectInherited;
  use crate::data::alife::inherited::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::inherited::alife_object_dynamic_visual::AlifeObjectDynamicVisual;
  use crate::data::alife::inherited::alife_object_item::AlifeObjectItem;
  use crate::data::alife::inherited::alife_object_item_custom_outfit::AlifeObjectItemCustomOutfit;
  use crate::data::alife::inherited::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::generic::vector_3d::Vector3d;
  use crate::data::meta::cls_id::ClsId;
  use crate::spawn::chunks::spawn_alife_spawns_chunk::SpawnALifeSpawnsChunk;
  use xray_chunk::{ChunkReadWrite, ChunkReader, ChunkWriter, XRayByteOrder};
  use xray_error::XRayResult;
  use xray_test_utils::utils::{
    get_relative_test_sample_file_path, open_test_resource_as_slice,
    overwrite_test_relative_resource_as_file,
  };
  use xray_test_utils::FileSlice;

  #[test]
  fn test_read_write_empty() -> XRayResult {
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write_empty.chunk");

    let original: SpawnALifeSpawnsChunk = SpawnALifeSpawnsChunk { objects: vec![] };

    let mut writer: ChunkWriter = ChunkWriter::new();

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 28);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 28);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 28 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read: SpawnALifeSpawnsChunk = SpawnALifeSpawnsChunk::read::<XRayByteOrder>(&mut reader)?;

    assert_eq!(read.objects.len(), original.objects.len());

    Ok(())
  }

  #[test]
  fn test_read_write() -> XRayResult {
    let filename: String = get_relative_test_sample_file_path(file!(), "read_write.chunk");

    let original: SpawnALifeSpawnsChunk = SpawnALifeSpawnsChunk {
      objects: vec![
        AlifeObject {
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
          inherited: AlifeObjectInherited::CseAlifeItemCustomOutfit(Box::new(
            AlifeObjectItemCustomOutfit {
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
            },
          )),
          update_data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        },
        AlifeObject {
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
          inherited: AlifeObjectInherited::CseAlifeSpaceRestrictor(Box::new(
            AlifeObjectSpaceRestrictor {
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
            },
          )),
          update_data: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        },
      ],
    };

    let mut writer: ChunkWriter = ChunkWriter::new();

    original.write::<XRayByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 419);

    let bytes_written: usize = writer.flush_chunk_into::<XRayByteOrder>(
      &mut overwrite_test_relative_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 419);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 419 + 8);

    let mut reader: ChunkReader = ChunkReader::from_slice(file)?
      .read_child_by_index(0)
      .expect("0 index chunk to exist");

    let read: SpawnALifeSpawnsChunk = SpawnALifeSpawnsChunk::read::<XRayByteOrder>(&mut reader)?;

    assert_eq!(read.objects.len(), original.objects.len());

    for (index, object) in read.objects.iter().enumerate() {
      assert_eq!(object, original.objects.get(index).unwrap());
    }

    Ok(())
  }
}
