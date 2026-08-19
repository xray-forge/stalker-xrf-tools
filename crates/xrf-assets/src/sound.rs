/// Converts an X-Ray sound reference into the OGG logical path it loads.
///
/// A reference may already carry the extension, in any case, which is why this is not a bare append.
pub fn ogg_logical_path(reference: &str) -> String {
  if reference
    .rsplit_once('.')
    .is_some_and(|(_, extension)| extension.eq_ignore_ascii_case("ogg"))
  {
    reference.to_string()
  } else {
    format!("{reference}.ogg")
  }
}

/// Converts a sound reference into the name the engine registers it under.
///
/// Both the `sounds` root and the `.ogg` extension are implied, and a config may spell either out. Case and separators are
/// normalized too, so a reference matches a registered name whichever way it was written.
pub fn sound_reference_name(reference: &str) -> String {
  let reference: String = reference.trim().replace('/', "\\").to_ascii_lowercase();
  let reference: &str = reference.strip_prefix("sounds\\").unwrap_or(&reference);

  reference.strip_suffix(".ogg").unwrap_or(reference).to_string()
}

#[cfg(test)]
mod tests {
  use super::{ogg_logical_path, sound_reference_name};

  #[test]
  fn keeps_an_extension_a_reference_already_carries() {
    assert_eq!(ogg_logical_path("weapons\\ak74_shot.ogg"), "weapons\\ak74_shot.ogg");
    assert_eq!(
      ogg_logical_path("weapons\\ak74_shot.OGG"),
      "weapons\\ak74_shot.OGG",
      "an uppercase extension is not doubled"
    );
  }

  #[test]
  fn appends_ogg_when_a_reference_omits_it() {
    assert_eq!(ogg_logical_path("weapons\\ak74_shot"), "weapons\\ak74_shot.ogg");
  }

  #[test]
  fn strips_the_implied_root_and_extension_from_a_name() {
    assert_eq!(
      sound_reference_name(" sounds/weapons/ak74_shot.ogg "),
      "weapons\\ak74_shot"
    );
    assert_eq!(sound_reference_name("Weapons\\AK74_Shot"), "weapons\\ak74_shot");
  }
}
