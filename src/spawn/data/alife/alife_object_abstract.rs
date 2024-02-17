use crate::spawn::utils::read_null_terminated_string;
use byteorder::{LittleEndian, ReadBytesExt};
use fileslice::FileSlice;

pub struct AlifeObjectAbstract {
  pub game_vertex_id: u16,
  pub distance: f32,
  pub direct_control: u32,
  pub level_vertex_id: u32,
  pub flags: u32,
  pub custom_data: String,
  pub story_id: i32,
  pub spawn_story_id: i32,
}

impl AlifeObjectAbstract {
  pub fn from_file(file: &mut FileSlice) -> AlifeObjectAbstract {
    let game_vertex_id: u16 = file.read_u16::<LittleEndian>().unwrap();
    let distance: f32 = file.read_f32::<LittleEndian>().unwrap();
    let direct_control: u32 = file.read_u32::<LittleEndian>().unwrap();
    let level_vertex_id: u32 = file.read_u32::<LittleEndian>().unwrap();
    let flags: u32 = file.read_u32::<LittleEndian>().unwrap();
    let custom_data: String = read_null_terminated_string(file);
    let story_id: i32 = file.read_i32::<LittleEndian>().unwrap();
    let spawn_story_id: i32 = file.read_i32::<LittleEndian>().unwrap();

    AlifeObjectAbstract {
      game_vertex_id,
      distance,
      direct_control,
      level_vertex_id,
      flags,
      custom_data,
      story_id,
      spawn_story_id,
    }
  }
}
