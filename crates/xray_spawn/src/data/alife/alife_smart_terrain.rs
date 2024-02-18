use crate::chunk::chunk::Chunk;
use crate::data::alife::alife_smart_zone::AlifeSmartZone;
use crate::data::alife_object::AlifeObjectInherited;
use crate::types::SpawnByteOrder;
use byteorder::ReadBytesExt;

pub struct AlifeSmartTerrain {
  pub base: AlifeSmartZone,
}

/*
{ name => 'arriving_npc',			type => 'l8u16v',	default => [] },
  { name => 'npc_info',				type => 'npc_info',		default => [] },
  { name => 'dead_times',				type => 'times',	default => [] },
  { name => 'is_base_on_actor_control',	type => 'u8',		default => 0 },

if ($_[0]->{is_base_on_actor_control} == 1) {
        $_[1]->set_save_marker($_[0], 'load', 0, 'CBaseOnActorControl') if $_[0]->{version} > 123;
        $_[1]->unpack_properties($_[0], (cs_cop_properties_info)[4..5]);
        $_[1]->set_save_marker($_[0], 'load', 1, 'CBaseOnActorControl') if $_[0]->{version} > 123;
      }
  { name => 'status',					type => 'u8',		default => 0 },
  { name => 'alarm_time',				type => 'CTime', default => 0},

  { name => 'is_respawn_point',		type => 'u8',		default => 0 },

if ($_[0]->{is_respawn_point} == 1) {
      $_[1]->unpack_properties($_[0], (cs_cop_properties_info)[7]);
      if ($_[0]->{script_version} > 11) {
        $_[1]->unpack_properties($_[0], (cs_cop_properties_info)[8]);
      }
    }
    $_[1]->unpack_properties($_[0], (cs_cop_properties_info)[9]);

  { name => 'respawn_count',			type => 'l8szbv',		default => [] },
  { name => 'last_respawn_update',	type => 'complex_time', default => 0},
  { name => 'population',				type => 'u8',		default => 0 },


  + save mark
 */
impl AlifeObjectInherited<AlifeSmartTerrain> for AlifeSmartTerrain {
  fn from_chunk(chunk: &mut Chunk) -> AlifeSmartTerrain {
    let base: AlifeSmartZone = AlifeSmartZone::from_chunk(chunk);

    let arriving_objects_count: u8 = chunk.read_u8().unwrap();

    assert_eq!(
      arriving_objects_count, 0,
      "Unexpected arriving objects in smart terrain."
    );

    let object_job_descriptors_count: u8 = chunk.read_u8().unwrap();

    assert_eq!(
      object_job_descriptors_count, 0,
      "Unexpected job objects in smart terrain."
    );

    let dead_objects_infos_count: u8 = chunk.read_u8().unwrap();

    assert_eq!(
      object_job_descriptors_count, 0,
      "Unexpected dead objects in smart terrain."
    );

    let smart_terrain_actor_control: u8 = chunk.read_u8().unwrap();

    assert_eq!(
      smart_terrain_actor_control, 0,
      "Unexpected smart terrain actor control."
    );

    let respawn_point: u8 = chunk.read_u8().unwrap();

    if respawn_point != 0 {
      panic!("Not expected respawn point handler.")
    }

    let staying_objects_count: u8 = chunk.read_u8().unwrap();

    assert_eq!(
      staying_objects_count, 0,
      "Unexpected smart terrain staying objects."
    );

    let save_marker: u16 = chunk.read_u16::<SpawnByteOrder>().unwrap();

    assert_eq!(
      save_marker, 6,
      "Unexpected data provided with smart terrain save."
    );

    AlifeSmartTerrain { base }
  }
}
