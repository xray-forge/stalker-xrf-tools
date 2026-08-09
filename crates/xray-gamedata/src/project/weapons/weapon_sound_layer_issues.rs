use std::collections::{BTreeMap, BTreeSet};

use xray_ltx::{LTX_SYMBOL_SCHEME, Section};

use crate::project::weapons::weapon_sound_layer_field::WeaponSoundLayerField;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum WeaponSoundLayerIssue {
  InvalidFieldName { field_name: String },
  MissingLayer { expected: u32, found: Option<u32> },
  MissingBaseLayer { layer: u32 },
  MissingVariant { layer: u32, expected: u32, found: u32 },
}

pub(crate) fn weapon_sound_layer_issues(section: &Section) -> Vec<WeaponSoundLayerIssue> {
  let mut issues: Vec<WeaponSoundLayerIssue> = Vec::new();
  let mut layers: BTreeMap<u32, BTreeSet<Option<u32>>> = BTreeMap::new();

  for (field_name, _) in section {
    // Metadata fields such as `$scheme` describe the section itself and are not sound layers.
    if field_name.starts_with(LTX_SYMBOL_SCHEME) {
      continue;
    }

    let Some(field) = WeaponSoundLayerField::parse(field_name) else {
      issues.push(WeaponSoundLayerIssue::InvalidFieldName {
        field_name: String::from(field_name),
      });
      continue;
    };

    layers.entry(field.layer()).or_default().insert(field.variant());
  }

  let mut expected_layer: u32 = 1;
  for (layer, variants) in &layers {
    if *layer != expected_layer {
      issues.push(WeaponSoundLayerIssue::MissingLayer {
        expected: expected_layer,
        found: Some(*layer),
      });
    }

    if !variants.contains(&None) {
      issues.push(WeaponSoundLayerIssue::MissingBaseLayer { layer: *layer });
    }

    let mut expected_variant: u32 = 1;
    for variant in variants.iter().flatten() {
      if *variant != expected_variant {
        issues.push(WeaponSoundLayerIssue::MissingVariant {
          layer: *layer,
          expected: expected_variant,
          found: *variant,
        });
      }

      expected_variant = variant.saturating_add(1);
    }

    expected_layer = layer.saturating_add(1);
  }

  if layers.is_empty() {
    issues.push(WeaponSoundLayerIssue::MissingLayer {
      expected: 1,
      found: None,
    });
  }

  issues
}

#[cfg(test)]
mod tests {
  use xray_ltx::Ltx;

  use super::{WeaponSoundLayerIssue, weapon_sound_layer_issues};

  #[test]
  fn accepts_contiguous_layer_and_variant_names() {
    let ltx: Ltx = Ltx::read_from_str(
      "[layered_shot]\n\
       snd_1_layer = weapons\\ak74\\shot\n\
       snd_1_layer1 = weapons\\ak74\\shot1\n\
       snd_2_layer = weapons\\ak74\\distant\n",
    )
    .expect("test LTX is valid");

    assert!(weapon_sound_layer_issues(&ltx["layered_shot"]).is_empty());
  }

  #[test]
  fn ignores_section_metadata_fields() {
    let ltx: Ltx = Ltx::read_from_str(
      "[layered_shot]\n\
       $scheme = $item_weapon_sound_layers\n\
       snd_1_layer = weapons\\ak74\\shot\n\
       snd_2_layer = weapons\\ak74\\distant\n",
    )
    .expect("test LTX is valid");

    assert!(
      weapon_sound_layer_issues(&ltx["layered_shot"]).is_empty(),
      "Expect $scheme to describe the section rather than be read as a sound layer"
    );
  }

  #[test]
  fn reports_invalid_and_non_contiguous_layer_names() {
    let ltx: Ltx = Ltx::read_from_str(
      "[layered_shot]\n\
       snd_1_layer = weapons\\ak74\\shot\n\
       snd_1_layer2 = weapons\\ak74\\shot2\n\
       snd_3_layer = weapons\\ak74\\distant\n\
       snd_4_layer1 = weapons\\ak74\\distant1\n\
       unknown = weapons\\ak74\\ignored\n",
    )
    .expect("test LTX is valid");

    let issues: Vec<WeaponSoundLayerIssue> = weapon_sound_layer_issues(&ltx["layered_shot"]);

    assert!(issues.contains(&WeaponSoundLayerIssue::InvalidFieldName {
      field_name: String::from("unknown"),
    }));
    assert!(issues.contains(&WeaponSoundLayerIssue::MissingLayer {
      expected: 2,
      found: Some(3),
    }));
    assert!(issues.contains(&WeaponSoundLayerIssue::MissingVariant {
      layer: 1,
      expected: 1,
      found: 2,
    }));
    assert!(issues.contains(&WeaponSoundLayerIssue::MissingBaseLayer { layer: 4 }));
  }
}
