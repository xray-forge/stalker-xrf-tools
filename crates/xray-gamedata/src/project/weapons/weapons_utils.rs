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
