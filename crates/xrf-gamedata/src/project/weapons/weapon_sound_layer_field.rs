#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct WeaponSoundLayerField {
  layer: u32,
  variant: Option<u32>,
}

impl WeaponSoundLayerField {
  pub(crate) fn parse(field_name: &str) -> Option<Self> {
    let (layer, variant): (&str, &str) = field_name.strip_prefix("snd_")?.split_once("_layer")?;

    let layer: u32 = Self::parse_positive_index(layer)?;

    let variant: Option<u32> = if variant.is_empty() {
      None
    } else {
      Some(Self::parse_positive_index(variant)?)
    };

    Some(Self { layer, variant })
  }

  pub(crate) fn layer(self) -> u32 {
    self.layer
  }

  pub(crate) fn variant(self) -> Option<u32> {
    self.variant
  }

  fn parse_positive_index(value: &str) -> Option<u32> {
    if value.starts_with('0') {
      return None;
    }

    value.parse().ok()
  }
}
