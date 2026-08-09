use xray_ltx::{Ltx, Section};

use crate::project::weapons::weapon_sound_value::WeaponSoundValue;

#[derive(Debug)]
pub(crate) enum WeaponSoundSource<'value, 'section> {
  Asset {
    name: &'value str,
  },
  LayeredSection {
    name: &'value str,
    has_parameters: bool,
    section: &'section Section,
  },
}

impl<'value, 'section> WeaponSoundSource<'value, 'section> {
  pub(crate) fn classify(ltx: &'section Ltx, value: WeaponSoundValue<'value>) -> Self {
    match ltx.section(value.name()) {
      Some(section) => Self::LayeredSection {
        name: value.name(),
        has_parameters: value.has_parameters(),
        section,
      },
      None => Self::Asset { name: value.name() },
    }
  }
}

#[cfg(test)]
mod tests {
  use xray_ltx::Ltx;

  use super::WeaponSoundSource;
  use crate::project::weapons::weapon_sound_value::WeaponSoundValue;

  #[test]
  fn classifies_an_ltx_section_as_a_layered_weapon_sound() {
    let ltx: Ltx = Ltx::read_from_str(
      "[layered_shot]\n\
       snd_1_layer = weapons\\ak74\\shot\n",
    )
    .expect("test LTX is valid");

    match WeaponSoundSource::classify(&ltx, WeaponSoundValue::parse("layered_shot, 1.0, 0.1")) {
      WeaponSoundSource::LayeredSection {
        name,
        has_parameters,
        section,
      } => {
        assert_eq!(name, "layered_shot");
        assert!(has_parameters);
        assert!(section.contains_key("snd_1_layer"));
      }
      WeaponSoundSource::Asset { .. } => panic!("sound section should be classified as layered"),
    }
  }

  #[test]
  fn classifies_a_non_section_as_a_direct_weapon_sound_asset() {
    let ltx: Ltx = Ltx::new();

    match WeaponSoundSource::classify(&ltx, WeaponSoundValue::parse("weapons\\ak74\\shot, 1.0, 0.1")) {
      WeaponSoundSource::Asset { name } => assert_eq!(name, "weapons\\ak74\\shot"),
      WeaponSoundSource::LayeredSection { .. } => {
        panic!("sound asset should not be classified as a layered section")
      }
    }
  }
}
