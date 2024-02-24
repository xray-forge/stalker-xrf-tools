use crate::chunk::chunk::Chunk;
use crate::chunk::writer::ChunkWriter;
use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
use crate::data::alife::alife_object_inherited_reader::{
  AlifeObjectGeneric, AlifeObjectInheritedReader,
};
use crate::data::time::Time;
use crate::types::SpawnByteOrder;
use byteorder::{ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Clone, Debug, PartialEq)]
pub struct AlifeObjectAnomalyZone {
  pub base: AlifeObjectCustomZone,
  pub offline_interactive_radius: f32,
  pub artefact_spawn_count: u16,
  pub artefact_position_offset: u32,
  pub last_spawn_time: Option<Time>,
}

impl AlifeObjectInheritedReader<AlifeObjectAnomalyZone> for AlifeObjectAnomalyZone {
  /// Read anomaly zone object data from the chunk.
  fn read_from_chunk<T: ByteOrder>(chunk: &mut Chunk) -> io::Result<AlifeObjectAnomalyZone> {
    let base: AlifeObjectCustomZone = AlifeObjectCustomZone::read_from_chunk::<T>(chunk)?;

    let offline_interactive_radius: f32 = chunk.read_f32::<SpawnByteOrder>()?;
    let artefact_spawn_count: u16 = chunk.read_u16::<SpawnByteOrder>()?;
    let artefact_position_offset: u32 = chunk.read_u32::<SpawnByteOrder>()?;

    // Last spawn time for artefacts, legacy approach:
    let last_spawn_time: Option<Time> = if chunk.is_ended() || chunk.read_u8()? == 0 {
      None
    } else {
      Some(Time::read_from_chunk::<SpawnByteOrder>(chunk)?)
    };

    Ok(AlifeObjectAnomalyZone {
      base,
      offline_interactive_radius,
      artefact_spawn_count,
      artefact_position_offset,
      last_spawn_time,
    })
  }

  /// Write anomaly zone object data into the writer.
  fn write<T: ByteOrder>(&self, writer: &mut ChunkWriter) -> io::Result<()> {
    self.base.write::<T>(writer)?;

    writer.write_f32::<T>(self.offline_interactive_radius)?;
    writer.write_u16::<T>(self.artefact_spawn_count)?;
    writer.write_u32::<T>(self.artefact_position_offset)?;

    if self.last_spawn_time.is_some() {
      writer.write_u8(1)?;
      self.last_spawn_time.as_ref().unwrap().write::<T>(writer)?;
    } else {
      writer.write_u8(0)?;
    }

    Ok(())
  }
}

impl AlifeObjectGeneric for AlifeObjectAnomalyZone {}

#[cfg(test)]
mod tests {
  use crate::chunk::chunk::Chunk;
  use crate::chunk::writer::ChunkWriter;
  use crate::data::alife::alife_object_abstract::AlifeObjectAbstract;
  use crate::data::alife::alife_object_anomaly_zone::AlifeObjectAnomalyZone;
  use crate::data::alife::alife_object_custom_zone::AlifeObjectCustomZone;
  use crate::data::alife::alife_object_inherited_reader::AlifeObjectInheritedReader;
  use crate::data::alife::alife_object_space_restrictor::AlifeObjectSpaceRestrictor;
  use crate::data::shape::Shape;
  use crate::data::time::Time;
  use crate::test::utils::{
    get_test_chunk_file_sub_dir, open_test_resource_as_slice, overwrite_test_resource_as_file,
  };
  use crate::types::SpawnByteOrder;
  use fileslice::FileSlice;
  use std::io;

  #[test]
  fn test_read_write_object() -> io::Result<()> {
    let mut writer: ChunkWriter = ChunkWriter::new();
    let filename: String =
      get_test_chunk_file_sub_dir(file!(), &String::from("alife_object_anomaly_zone.chunk"));

    let object: AlifeObjectAnomalyZone = AlifeObjectAnomalyZone {
      base: AlifeObjectCustomZone {
        base: AlifeObjectSpaceRestrictor {
          base: AlifeObjectAbstract {
            game_vertex_id: 241,
            distance: 55.3,
            direct_control: 4,
            level_vertex_id: 87,
            flags: 12,
            custom_data: "".to_string(),
            story_id: 6211,
            spawn_story_id: 143,
          },
          shape: vec![
            Shape::Sphere(((0.5, 0.5, 0.5), 1.0)),
            Shape::Box((
              (4.1, 1.1, 32.1),
              (1.1, 2.2, 3.3),
              (4.0, 5.0, 1.4),
              (9.2, 8.3, 1.0),
            )),
          ],
          restrictor_type: 4,
        },
        max_power: 255.33,
        owner_id: 1,
        enabled_time: 3312,
        disabled_time: 521,
        start_time_shift: 250,
      },
      offline_interactive_radius: -3231.1,
      artefact_spawn_count: 3,
      artefact_position_offset: 5,
      last_spawn_time: Some(Time {
        year: 12,
        month: 11,
        day: 4,
        hour: 21,
        minute: 30,
        second: 10,
        millis: 550,
      }),
    };

    object.write::<SpawnByteOrder>(&mut writer)?;

    assert_eq!(writer.bytes_written(), 134);

    let bytes_written: usize = writer.flush_chunk_into_file::<SpawnByteOrder>(
      &mut overwrite_test_resource_as_file(&filename)?,
      0,
    )?;

    assert_eq!(bytes_written, 134);

    let file: FileSlice = open_test_resource_as_slice(&filename)?;

    assert_eq!(file.bytes_remaining(), 134 + 8);

    let mut chunk: Chunk = Chunk::from_file(file)?.read_child_by_index(0)?;
    let read_object: AlifeObjectAnomalyZone =
      AlifeObjectAnomalyZone::read_from_chunk::<SpawnByteOrder>(&mut chunk)?;

    assert_eq!(read_object, object);

    Ok(())
  }
}
