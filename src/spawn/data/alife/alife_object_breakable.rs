use crate::spawn::data::alife::alife_object_visual::AlifeObjectVisual;
use byteorder::{LittleEndian, ReadBytesExt};
use fileslice::FileSlice;

pub struct AlifeObjectBreakable {
  pub base: AlifeObjectVisual,
  pub health: f32,
}

impl AlifeObjectBreakable {
  pub fn from_file(file: &mut FileSlice) -> AlifeObjectBreakable {
    let base: AlifeObjectVisual = AlifeObjectVisual::from_file(file);
    let health: f32 = file.read_f32::<LittleEndian>().unwrap();

    assert_eq!(
      file.cursor_pos(),
      file.end_pos(),
      "Expected all data to be read from chunk."
    );

    AlifeObjectBreakable { base, health }
  }
}
