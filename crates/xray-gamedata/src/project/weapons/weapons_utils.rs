use xray_ltx::Section;

/// Get weapon animation from LTX field value with comma-separated parameters.
/// First param is motion name.
pub fn get_weapon_animation_name(value: &str) -> String {
  String::from(
    *value
      .split(",")
      .collect::<Vec<&str>>()
      .first()
      .unwrap_or(&value),
  )
}

pub fn is_weapon_section(section: &Section) -> bool {
  section.contains_key("class")
    && section.contains_key("weapon_class")
    && (section
      .get("$scheme")
      .is_some_and(|it| it == "$item_weapon")
      || section.contains_key("flame_particles")
      || section.contains_key("flame"))
}

pub fn is_player_hud_section(section: &Section) -> bool {
  section.contains_key("visual")
    && section.contains_key("position")
    && section.contains_key("orientation")
    && section.contains_key("ancor_0")
    && section.contains_key("ancor_1")
}
