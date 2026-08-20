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
  use super::sound_reference_name;

  #[test]
  fn strips_the_implied_root_and_extension_from_a_name() {
    assert_eq!(
      sound_reference_name(" sounds/weapons/ak74_shot.ogg "),
      "weapons\\ak74_shot"
    );
    assert_eq!(sound_reference_name("Weapons\\AK74_Shot"), "weapons\\ak74_shot");
  }
}
