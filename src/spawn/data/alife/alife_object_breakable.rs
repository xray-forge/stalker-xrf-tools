use crate::spawn::chunk_utils::read_null_terminated_string;
use byteorder::{LittleEndian, ReadBytesExt};
use fileslice::FileSlice;

pub struct AlifeObjectBreakable {
  pub health: f32,
}

impl AlifeObjectBreakable {
  pub fn from_file(file: &mut FileSlice) -> AlifeObjectBreakable {
    let game_vertex_id: u16 = file.read_u16::<LittleEndian>().unwrap();
    let distance: f32 = file.read_f32::<LittleEndian>().unwrap();
    let direct_control: u32 = file.read_u32::<LittleEndian>().unwrap();
    let level_vertex_id: u32 = file.read_u32::<LittleEndian>().unwrap();
    let flags: u32 = file.read_u32::<LittleEndian>().unwrap();
    let custom_data: String = read_null_terminated_string(file);
    let story_id: i32 = file.read_i32::<LittleEndian>().unwrap();
    let spawn_story_id: i32 = file.read_i32::<LittleEndian>().unwrap();

    let visual_name: String = read_null_terminated_string(file);
    let visual_flags: u8 = file.read_u8().unwrap();

    let health: f32 = file.read_f32::<LittleEndian>().unwrap();

    assert_eq!(
      file.cursor_pos(),
      file.end_pos(),
      "Expected all data to be read from chunk."
    );

    AlifeObjectBreakable { health }
  }
}
