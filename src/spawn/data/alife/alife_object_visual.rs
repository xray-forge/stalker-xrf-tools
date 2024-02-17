use crate::spawn::chunk_utils::read_null_terminated_string;
use crate::spawn::data::alife::alife_object_abstract::AlifeObjectAbstract;
use byteorder::ReadBytesExt;
use fileslice::FileSlice;

pub struct AlifeObjectVisual {
  pub base: AlifeObjectAbstract,
  pub visual_name: String,
  pub visual_flags: u8,
}

impl AlifeObjectVisual {
  pub fn from_file(file: &mut FileSlice) -> AlifeObjectVisual {
    let base: AlifeObjectAbstract = AlifeObjectAbstract::from_file(file);

    let visual_name: String = read_null_terminated_string(file);
    let visual_flags: u8 = file.read_u8().unwrap();

    AlifeObjectVisual {
      base,
      visual_name,
      visual_flags,
    }
  }
}
