use crate::spawn::chunk_utils::read_shape_description;
use crate::spawn::data::alife::alife_object_abstract::AlifeObjectAbstract;
use fileslice::FileSlice;

pub struct AlifeObjectShape {
  pub base: AlifeObjectAbstract,
  pub shapes: Vec<f32>,
}

impl AlifeObjectShape {
  pub fn from_file(file: &mut FileSlice) -> AlifeObjectShape {
    let base: AlifeObjectAbstract = AlifeObjectAbstract::from_file(file);

    let shapes: Vec<f32> = read_shape_description(file);

    AlifeObjectShape { base, shapes }
  }
}
