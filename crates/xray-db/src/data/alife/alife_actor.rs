use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_actor::AlifeObjectActor;
use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeActor {
  pub base: AlifeObjectActor,
  pub start_position_filled: u8,
  pub save_marker: u16,
}

impl AlifeObjectInheritedReader<AlifeActor> for AlifeActor {
  /// Read actor data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeActor> {
    let base: AlifeObjectActor = AlifeObjectActor::read_from_chunk::<T>(chunk)?;

    let start_position_filled: u8 = chunk.read_u8()?;
    let save_marker: u16 = chunk.read_u16::<SpawnByteOrder>()?;

    assert_eq!(
      save_marker, 1,
      "Unexpected save data for actor object provided."
    );

    Ok(AlifeActor {
      base,
      start_position_filled,
      save_marker,
    })
  }
}

impl AlifeObjectGeneric for AlifeActor {
  type Order = SpawnByteOrder;

  /// Write object data into the writer.
  fn write(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write(writer)?;

    writer.write_u8(self.start_position_filled)?;
    writer.write_u16::<Self::Order>(self.save_marker)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_actor::AlifeActor;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_actor::AlifeObjectActor;
  use crate::data::alife::alife_object_creature::AlifeObjectCreature;
  use crate::data::alife::alife_object_generic::AlifeObjectGeneric;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_skeleton::AlifeObjectSkeleton;
  use crate::data::alife::alife_object_trader_abstract::AlifeObjectTraderAbstract;
  use crate::data::alife::alife_object_visual::AlifeObjectVisual;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String = get_test_chunk_file_sub_dir(file!(), &String::from("alife_actor.chunk"));

    let object: AlifeActor = AlifeActor {
      base: AlifeObjectActor {
        base: AlifeObjectCreature {
          base: AlifeObjectVisual {
            base: AlifeObjectAbstract {
              game_vertex_id: 621,
              distance: 55.25,
              direct_control: 15,
              level_vertex_id: 52235,
              flags: 72,
              custom_data: String::from("custom-data"),
              story_id: 15,
              spawn_story_id: 334,
            },
            visual_name: String::from("visual-name"),
            visual_flags: 13,
          },
          team: 2,
          squad: 3,
          group: 4,
          health: 1.0,
          dynamic_out_restrictions: vec![1, 2, 3, 4],
          dynamic_in_restrictions: vec![5, 6, 7, 8],
          killer_id: 0,
          game_death_time: 0,
        },
        trader: AlifeObjectTraderAbstract {
          money: 5000,
          specific_character: String::from("specific-character-0"),
          trader_flags: 23,
          character_profile: String::from("character-profile-0"),
          community_index: 1,
          rank: 2,
          reputation: 3,
          character_name: String::from("character-name-0"),
          dead_body_can_take: 1,
          dead_body_closed: 1,
        },
        skeleton: AlifeObjectSkeleton {
          name: String::from("skeleton-name-0"),
          flags: 98,
          source_id: 12,
        },
        holder_id: 0,
      },
      start_position_filled: 1,
      save_marker: 1,
    };

    object.write(&mut writer)?;

    assert_eq!(writer.bytes_written(), 196);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 196);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 196 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeActor = AlifeActor::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
