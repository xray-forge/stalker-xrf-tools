use crate::spawn::data::alife::alife_object_shape::AlifeObjectShape;
use crate::spawn::utils::read_null_terminated_string;
use fileslice::FileSlice;

pub struct AlifeObjectClimable {
  pub base: AlifeObjectShape,
  pub game_material: String,
}

impl AlifeObjectClimable {
  pub fn from_file(file: &mut FileSlice) -> AlifeObjectClimable {
    let base: AlifeObjectShape = AlifeObjectShape::from_file(file);

    let game_material: String = read_null_terminated_string(file);

    assert_eq!(
      file.cursor_pos(),
      file.end_pos(),
      "Expected all data to be read from chunk."
    );

    AlifeObjectClimable {
      base,
      game_material,
    }
  }
}
