/// Fields required in every weather time section.
pub const WEATHER_REQUIRED_FIELDS: [&str; 25] = [
  "ambient",
  "ambient_color",
  "clouds_color",
  "clouds_texture",
  "far_plane",
  "fog_color",
  "fog_density",
  "fog_distance",
  "hemisphere_color",
  "rain_color",
  "rain_density",
  "sky_color",
  "sky_rotation",
  "sky_texture",
  "sun",
  "sun_altitude",
  "sun_color",
  "sun_longitude",
  "sun_shafts_intensity",
  "thunderbolt_collection",
  "thunderbolt_duration",
  "thunderbolt_period",
  "water_intensity",
  "wind_direction",
  "wind_velocity",
];

/// Fields that must contain one finite floating-point value.
pub const WEATHER_FLOAT_FIELDS: [&str; 13] = [
  "far_plane",
  "fog_density",
  "fog_distance",
  "rain_density",
  "sky_rotation",
  "sun_altitude",
  "sun_longitude",
  "sun_shafts_intensity",
  "thunderbolt_duration",
  "thunderbolt_period",
  "water_intensity",
  "wind_direction",
  "wind_velocity",
];

/// Fields that must contain exactly three finite floating-point values.
pub const WEATHER_VECTOR3_FIELDS: [&str; 5] = [
  "ambient_color",
  "fog_color",
  "rain_color",
  "sky_color",
  "sun_color",
];

/// Fields that must contain exactly four finite floating-point values.
pub const WEATHER_VECTOR4_FIELDS: [&str; 2] = ["clouds_color", "hemisphere_color"];

/// Fields that must contain a non-empty string.
pub const WEATHER_NONEMPTY_STRING_FIELDS: [&str; 3] = ["ambient", "clouds_texture", "sky_texture"];

/// Floating-point fields that must be finite and greater than or equal to zero.
pub const WEATHER_NONNEGATIVE_FLOAT_FIELDS: [&str; 5] = [
  "far_plane",
  "fog_distance",
  "thunderbolt_duration",
  "thunderbolt_period",
  "wind_velocity",
];

/// Parses a canonical `HH:MM:SS` section name into seconds since midnight.
///
/// Returns `None` unless every component has two ASCII digits and is within its clock range.
pub fn parse_weather_time(section_name: &str) -> Option<u32> {
  let mut parts: std::str::Split<'_, char> = section_name.split(':');

  let hours: &str = parts.next()?;
  let minutes: &str = parts.next()?;
  let seconds: &str = parts.next()?;

  if parts.next().is_some()
    || [hours, minutes, seconds]
      .iter()
      .any(|part| part.len() != 2 || !part.bytes().all(|byte| byte.is_ascii_digit()))
  {
    return None;
  }

  let hours: u32 = hours.parse().ok()?;
  let minutes: u32 = minutes.parse().ok()?;
  let seconds: u32 = seconds.parse().ok()?;

  if hours >= 24 || minutes >= 60 || seconds >= 60 {
    None
  } else {
    Some(hours * 3600 + minutes * 60 + seconds)
  }
}

/// Checks the value shape for a known canonical weather field.
///
/// Returns `true` for unrecognized fields so renderer- or engine-specific extensions remain
/// available to other validation layers.
pub fn is_valid_weather_field_value(field_name: &str, value: &str) -> bool {
  if WEATHER_NONNEGATIVE_FLOAT_FIELDS.contains(&field_name) {
    parse_finite_float(value).is_some_and(|value| value >= 0.0)
  } else if WEATHER_FLOAT_FIELDS.contains(&field_name) {
    parse_finite_float(value).is_some()
  } else if WEATHER_VECTOR3_FIELDS.contains(&field_name) {
    is_finite_float_list(value, 3)
  } else if WEATHER_VECTOR4_FIELDS.contains(&field_name) {
    is_finite_float_list(value, 4)
  } else if WEATHER_NONEMPTY_STRING_FIELDS.contains(&field_name) {
    !value.is_empty()
  } else {
    true
  }
}

fn is_finite_float_list(value: &str, expected_length: usize) -> bool {
  let values: Vec<&str> = value.split(',').map(str::trim).collect();

  values.len() == expected_length
    && values
      .iter()
      .all(|value| parse_finite_float(value).is_some())
}

fn parse_finite_float(value: &str) -> Option<f32> {
  value.parse::<f32>().ok().filter(|value| value.is_finite())
}
